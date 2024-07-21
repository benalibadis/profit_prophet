use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::io::AsyncWriteExt;
use tokio::task;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::oneshot;
use crate::load_balancing::LoadBalancingStrategy;

pub struct DataSender<T, S>
where
    S: LoadBalancingStrategy + Send + Sync + 'static + Clone,
{
    addresses: Vec<String>,
    receiver: mpsc::Receiver<T>,
    strategy: S,
    tcp_streams: HashMap<String, mpsc::Sender<(T, oneshot::Sender<()>)>>,
}

impl<T, S> DataSender<T, S>
where
    T: Send + 'static + std::fmt::Debug + Clone,
    S: LoadBalancingStrategy + Send + Sync + 'static + Clone,
{
    pub fn new(addresses: Vec<String>, receiver: mpsc::Receiver<T>, strategy: S) -> Self {
        Self {
            addresses,
            receiver,
            strategy,
            tcp_streams: HashMap::new(),
        }
    }

    pub async fn run(&mut self) {
        self.run_tcp_stream_managers().await;
        self.run_data_sender().await;
    }
    
    async fn run_data_sender(&mut self) {
        while let Some(data) = self.receiver.recv().await {
            let listener = self.strategy.select_listener(&self.addresses).await;
            if let Some(tx) = self.tcp_streams.get(&listener) {
                let (ack_tx, ack_rx) = oneshot::channel();
                if tx.send((data.clone(), ack_tx)).await.is_ok() {
                    let _ = ack_rx.await;
                } else {
                    eprintln!("Failed to send data to task for {:?}", listener);
                }
            } else {
                eprintln!("No sender found for listener {:?}", listener);
            }
        }
    }

    async fn run_tcp_stream_managers(&mut self) {
        for address in &self.addresses {
            let (tx, mut rx) = mpsc::channel(100);
            self.tcp_streams.insert(address.clone(), tx);
            let addr: SocketAddr = address.parse().expect("Invalid listener address");

            let mut strategy = self.strategy.clone();
            task::spawn(async move {
                let mut stream = TcpStream::connect(addr).await.unwrap();

                while let Some((data, ack_tx)) = rx.recv().await {
                    strategy.update_state(&addr.to_string(), true);
                    let data_string = format!("{:?}", data);

                    loop {
                        match stream.write_all(data_string.as_bytes()).await {
                            Ok(_) => {
                                println!("Data sent to {}: {:?}", addr, data);
                                let _ = ack_tx.send(());
                                strategy.update_state(&addr.to_string(), false);
                                break;
                            }
                            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe || e.kind() == std::io::ErrorKind::ConnectionReset => {
                                eprintln!("Failed to send data {:?} to {}: {}. Reconnecting...", data, addr, e);
                                stream = TcpStream::connect(addr).await.unwrap();
                                continue;    
                            }
                            Err(e) => {
                                eprintln!("Failed to send data to {}: {}", addr, e);
                                break;
                            }
                        }
                    }
                }
            });
        }
    }
}
