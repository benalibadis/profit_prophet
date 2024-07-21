use data_sink::influxdb::{TcpInfluxDbServer};
use tokio::net::TcpStream;
use tokio::time::Duration;
use tokio::io::AsyncWriteExt;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use serde_json::json;
use mockito::{mock, Matcher};
use connector::influxdb::InfluxDbClient;
use env_logger;

#[tokio::test]
async fn test_tcp_influxdb_server() {
    let _ = env_logger::builder().is_test(true).try_init();

    let _m = mock("POST", "/api/v2/write")
        .with_status(204)
        .with_header("content-type", "application/json")
        .create();

    let influxdb_client = InfluxDbClient::new(&mockito::server_url(), "KAtopm-k4IVJUagoQm_obwIZF1WyRfSTzlHIGVkBc91otqztpOjkLnbAu8WJ9H7hVZjiOBSDvlNztkWKpY1OVQ==");
    let address: SocketAddr = "127.0.0.1:12345".parse().unwrap();

    let server = TcpInfluxDbServer::new(influxdb_client, address.clone());

    tokio::spawn(async move {
        if let Err(e) = server.run().await {
            eprintln!("Server error: {:?}", e);
        }
    });

    // Give the server some time to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Connect to the server and send data
    let mut stream = TcpStream::connect(address).await.unwrap();
    let data = json!({
        "measurement": "test",
        "tags": {"tag1": "value1"},
        "fields": {"field1": 42}
    }).to_string();
    stream.write_all(data.as_bytes()).await.unwrap();

    // Verify the request was made to mockito
    _m.assert();
}

#[tokio::test]
async fn test_tcp_influxdb_server_error_handling() {
    let _ = env_logger::builder().is_test(true).try_init();

    let _m = mock("POST", "/api/v2/write")
        .with_status(500)
        .with_header("content-type", "application/json")
        .create();

    let influxdb_client = InfluxDbClient::new(&mockito::server_url(), "KAtopm-k4IVJUagoQm_obwIZF1WyRfSTzlHIGVkBc91otqztpOjkLnbAu8WJ9H7hVZjiOBSDvlNztkWKpY1OVQ==");
    let address: SocketAddr = "127.0.0.1:12346".parse().unwrap();

    let server = TcpInfluxDbServer::new(influxdb_client, address.clone());

    tokio::spawn(async move {
        if let Err(e) = server.run().await {
            eprintln!("Server error: {:?}", e);
        }
    });

    // Give the server some time to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Connect to the server and send invalid data
    let mut stream = TcpStream::connect(address).await.unwrap();
    let invalid_data = "invalid data";
    stream.write_all(invalid_data.as_bytes()).await.unwrap();

    // Verify the request was made to mockito
    _m.assert();
}
