use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::task;
use std::net::SocketAddr;

enum LoadBalancingStrategy {
    ROUND_ROBIN,
    LEAST_CONNECTIONS,
}

struct DataSender<T> {
    listeners: Vec<String>,
    receiver: mpsc::Receiver<T>,
}

impl<T> DataSender<T>
where
    T: Send + 'static + std::fmt::Debug + Clone,
{
    pub fn new(listeners: Vec<String>, receiver: mpsc::Receiver<T>) -> Self {
        Self { listeners, receiver }
    }

    pub async fn run(&mut self) {
        let listeners = self.listeners.clone();
        while let Some(data) = self.receiver.recv().await {
            for listener in &listeners {
                let data = data.clone(); // Ensure data is cloned for each listener
                let addr: SocketAddr = listener.parse().expect("Invalid listener address");

                task::spawn(async move {
                    match TcpStream::connect(addr).await {
                        Ok(mut stream) => {
                            let data_string = format!("{:?}", data);
                            if let Err(e) = stream.write_all(data_string.as_bytes()).await {
                                eprintln!("Failed to send data to {}: {}", addr, e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to connect to {}: {}", addr, e);
                        }
                    }
                });
            }
        }
    }
}

async fn start_listener(addr: String) {
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind to address");

    loop {
        let (mut socket, _) = listener.accept().await.expect("Failed to accept connection");
        let addr_clone = addr.clone();
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return, // connection closed
                Ok(n) => {
                    let received = String::from_utf8_lossy(&buf[..n]);
                    println!("Received on {}: {}", addr_clone, received);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket; err = {:?}", e);
                }
            }
        });
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);
    let listeners = vec!["127.0.0.1:8080".to_string(), "127.0.0.1:8081".to_string()];
    let mut data_sender = DataSender::new(listeners.clone(), rx);

    // Spawn the DataSender
    let data_sender_handle = tokio::spawn(async move {
        data_sender.run().await;
    });

    // Spawn TCP listeners
    let listener_handles: Vec<_> = listeners
        .into_iter()
        .map(|addr| tokio::spawn(start_listener(addr)))
        .collect();

    // Example sending data
    for i in 0..10 {
        tx.send(format!("Message {}", i)).await.unwrap();
    }

    // Await the handles to keep the main function running
    data_sender_handle.await.unwrap();
    for handle in listener_handles {
        handle.await.unwrap();
    }
}
