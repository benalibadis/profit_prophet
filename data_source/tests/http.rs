use data_source::http::{HttpDataSource, TcpServer};
use mockito::{mock, Matcher};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio::time::Duration;
use tokio::io::AsyncReadExt;

#[derive(Debug, Deserialize, Serialize)]
struct TestResponse {
    message: String,
}

#[tokio::test]
async fn test_http_data_source_start_get_request_success() {
    let _m = mock("GET", "/test")
        .match_query(Matcher::UrlEncoded("param1".into(), "value1".into()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Hello, World!"}"#)
        .create();

    let url = format!("{}/test", &mockito::server_url());
    let method = "GET".to_string();

    let query_params = Some(HashMap::from([("param1".to_string(), "value1".to_string())]));

    let data_source = HttpDataSource::<()>::new(
        url.clone(),
        method.clone(),
        None,
        query_params,
        None,
        None,
        None,
    );

    // Spawn a task to start the data source
    let data_source_clone = data_source.clone();
    let handle = tokio::spawn(async move {
        data_source_clone.start::<TestResponse>().await.unwrap();
    });

    // Wait briefly to allow the request to be made
    tokio::time::sleep(Duration::from_millis(500)).await;
    handle.await.unwrap();
}

#[tokio::test]
async fn test_http_data_source_start_post_request_success() {
    let _m = mock("POST", "/test")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Data received!"}"#)
        .match_header("content-type", "application/json")
        .match_body(Matcher::JsonString(r#"{"name":"John Doe","age":30}"#.to_string()))
        .create();

    #[derive(Debug, Serialize, Clone)]
    struct PostBody {
        name: String,
        age: u8,
    }

    let post_body = PostBody {
        name: "John Doe".to_string(),
        age: 30,
    };

    let url = format!("{}/test", &mockito::server_url());
    let method = "POST".to_string();

    let headers = Some(HashMap::from([
        ("Content-Type".to_string(), "application/json".to_string())
    ]));

    let data_source = HttpDataSource::new(
        url.clone(),
        method.clone(),
        headers,
        None,
        Some(post_body.clone()),
        None,
        None,
    );

    // Spawn a task to start the data source
    let data_source_clone = data_source.clone();
    let handle = tokio::spawn(async move {
        data_source_clone.start::<TestResponse>().await.unwrap();
    });

    // Wait briefly to allow the request to be made
    tokio::time::sleep(Duration::from_millis(500)).await;
    handle.await.unwrap();
}

#[tokio::test]
async fn test_http_data_source_with_tcp_server() {
    // Mock the HTTP server
    let _m = mock("GET", "/test")
        .match_query(Matcher::UrlEncoded("param1".into(), "value1".into()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Hello, World!"}"#)
        .create();

    // Mock the TCP server using tokio::net::TcpListener
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let local_addr = listener.local_addr().unwrap();

    let url = format!("{}/test", &mockito::server_url());
    let method = "GET".to_string();

    let query_params = Some(HashMap::from([("param1".to_string(), "value1".to_string())]));

    let tcp_server = TcpServer::new(local_addr.ip().to_string(), local_addr.port());

    let data_source = HttpDataSource::<()>::new(
        url.clone(),
        method.clone(),
        None,
        query_params,
        None,
        None,
        Some(tcp_server.clone()),
    );

    // Spawn a task to start the data source
    let data_source_clone = data_source.clone();
    let handle = tokio::spawn(async move {
        data_source_clone.start::<TestResponse>().await.unwrap();
    });

    // Accept the TCP connection and verify the received message
    let tcp_handle = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = [0; 1024];
        let n = socket.read(&mut buf).await.unwrap();
        let received = String::from_utf8_lossy(&buf[..n]);
        assert!(received.contains("Hello, World!"));
    });

    // Wait briefly to allow the interaction to occur
    tokio::time::sleep(Duration::from_millis(500)).await;
    handle.await.unwrap();
    tcp_handle.await.unwrap();
}

/*  These can be activated once we implement a `stop` method on HttpDataSource
#[tokio::test]
async fn test_http_data_source_with_all_properties() {
    // Mock the HTTP server
    let _m = mock("POST", "/test")
        .match_query(Matcher::UrlEncoded("param1".into(), "value1".into()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Data received!"}"#)
        .match_header("content-type", "application/json")
        .match_body(Matcher::JsonString(r#"{"name":"John Doe","age":30}"#.to_string()))
        .create();

    // Mock the TCP server using tokio::net::TcpListener
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let local_addr = listener.local_addr().unwrap();

    #[derive(Debug, Serialize, Clone)]
    struct PostBody {
        name: String,
        age: u8,
    }

    let post_body = PostBody {
        name: "John Doe".to_string(),
        age: 30,
    };

    let url = format!("{}/test", &mockito::server_url());
    let method = "POST".to_string();

    let headers = Some(HashMap::from([
        ("Content-Type".to_string(), "application/json".to_string())
    ]));

    let query_params = Some(HashMap::from([
        ("param1".to_string(), "value1".to_string())
    ]));

    let tcp_server = TcpServer::new(local_addr.ip().to_string(), local_addr.port());

    let data_source = HttpDataSource::new(
        url.clone(),
        method.clone(),
        headers,
        query_params,
        Some(post_body.clone()),
        Some(Duration::from_secs(1)),
        Some(tcp_server.clone()),
    );

    // Spawn a task to start the data source
    let data_source_clone = data_source.clone();
    let handle = tokio::spawn(async move {
        data_source_clone.start::<TestResponse>().await.unwrap();
    });

    // Accept the TCP connection and verify the received message
    let tcp_handle = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = [0; 1024];
        let n = socket.read(&mut buf).await.unwrap();
        let received = String::from_utf8_lossy(&buf[..n]);
        assert!(received.contains("Data received!"));
    });

    // Wait briefly to allow the interaction to occur
    tokio::time::sleep(Duration::from_millis(500)).await;
    handle.await.unwrap();
    tcp_handle.await.unwrap();
}

#[tokio::test]
async fn test_http_data_source_with_interval() {
    let _m = mock("GET", "/test")
        .match_query(Matcher::UrlEncoded("param1".into(), "value1".into()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Hello, World!"}"#)
        .create();

    #[derive(Debug, Deserialize, Serialize)]
    struct TestResponse {
        message: String,
    }

    let url = format!("{}/test", &mockito::server_url());
    let method = "GET".to_string();

    let query_params = Some(HashMap::from([("param1".to_string(), "value1".to_string())]));

    let data_source = HttpDataSource::<()>::new(
        url.clone(),
        method.clone(),
        None,
        query_params,
        None,
        Some(Duration::from_secs(1)),
        None,
    );

    // Spawn a task to start the data source
    let data_source_clone = data_source.clone();
    let handle = tokio::spawn(async move {
        data_source_clone.start::<TestResponse>().await.unwrap();
    });

    // Wait for a few intervals to pass
    tokio::time::sleep(Duration::from_secs(3)).await;
    handle.await.unwrap();
}
*/