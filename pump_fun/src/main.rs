use data_source::load_balancing::{RoundRobinLoadBalancingStrategy, LeastConnectionsLoadBalancingStrategy};
use data_source::sender::DataSender;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};
use tokio::io::AsyncReadExt;

async fn start_listener(addr: String, ready_tx: oneshot::Sender<()>) {
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind to address");
    ready_tx.send(()).expect("Failed to send ready signal");

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

    // Choose the strategy here
    let round_robin_strategy = RoundRobinLoadBalancingStrategy::new(&listeners);
    let least_connections_strategy = LeastConnectionsLoadBalancingStrategy::new(&listeners);

    // Use one of the strategies
    let mut data_sender = DataSender::new(listeners.clone(), rx, round_robin_strategy);

    // Vector to hold the oneshot receiver handles
    let mut ready_receivers = Vec::new();

    // Spawn TCP listeners
    let listener_handles: Vec<_> = listeners
        .into_iter()
        .map(|addr| {
            let (ready_tx, ready_rx) = oneshot::channel();
            ready_receivers.push(ready_rx);
            tokio::spawn(start_listener(addr, ready_tx))
        })
        .collect();

    // Wait for all listeners to be ready
    for ready_rx in ready_receivers {
        ready_rx.await.expect("Failed to receive ready signal");
    }

    // Channel to signal DataSender readiness
    let (data_sender_ready_tx, data_sender_ready_rx) = oneshot::channel();

    // Spawn the DataSender
    let data_sender_handle = tokio::spawn(async move {
        data_sender.run().await;
        data_sender_ready_tx.send(()).expect("Failed to send DataSender ready signal");
    });

    // Example sending data
    for i in 0..10 {
        tx.send(format!("Message {}", i)).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    // Wait for the DataSender to finish
    data_sender_ready_rx.await.expect("Failed to receive DataSender ready signal");

    // Await the listener handles to keep the main function running
    for handle in listener_handles {
        handle.await.unwrap();
    }

    // Ensure DataSender has finished
    data_sender_handle.await.unwrap();
}