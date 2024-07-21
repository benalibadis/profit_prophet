use tokio::task;
use tokio::time::{Duration, Instant, interval_at};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use connector::http::HttpClient;
use connector::http::HttpClientError;
use log::{info, debug, error};

#[derive(Debug, thiserror::Error)]
pub enum HttpDataSourceError {
    #[error("HttpClient error: {0}")]
    HttpClientError(#[from] HttpClientError),
    #[error("TCP error: {0}")]
    TcpError(#[from] std::io::Error),
    #[error("Deserialize error: {0}")]
    DeserializeError(#[from] serde_json::Error),
}

#[derive(Clone, Debug)]
pub struct TcpServer {
    ip: String,
    port: u16,
}

impl TcpServer {
    pub fn new(ip: String, port: u16) -> Self {
        TcpServer { ip, port }
    }

    pub async fn send<T: Serialize>(&self, data: T) -> Result<(), std::io::Error> {
        let mut stream = TcpStream::connect((&self.ip as &str, self.port)).await?;
        let serialized_data = serde_json::to_string(&data).unwrap();
        stream.write_all(serialized_data.as_bytes()).await?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct HttpDataSource<B: Serialize + Clone + Send + Sync + Debug + 'static> {
    http_client: HttpClient,
    url: String,
    method: String,
    headers: Option<HashMap<String, String>>,
    query_params: Option<HashMap<String, String>>,
    body: Option<B>,
    interval: Option<Duration>,
    tcp_server: Option<TcpServer>,
}

impl<B: Serialize + Clone + Send + Sync + Debug + 'static> HttpDataSource<B> {
    pub fn new(
        url: String,
        method: String,
        headers: Option<HashMap<String, String>>,
        query_params: Option<HashMap<String, String>>,
        body: Option<B>,
        interval: Option<Duration>,
        tcp_server: Option<TcpServer>,
    ) -> Self {
        
        let data_source = HttpDataSource {
            http_client: HttpClient::new(),
            url,
            method,
            headers,
            query_params,
            body,
            interval,
            tcp_server,
        };
        debug!("Initialized HttpDataSource with URL: {}, Method: {}, Headers: {:?}, Query Params: {:?}, Body: {:?}, Interval: {:?}, TcpServer: {:?}",
              data_source.url,
              data_source.method,
              data_source.headers,
              data_source.query_params,
              data_source.body,
              data_source.interval,
              data_source.tcp_server
        );
        data_source
    }

    pub async fn start<R: DeserializeOwned + Serialize + Debug + Send + 'static>(self) -> Result<(), HttpDataSourceError> {
        if let Some(interval) = self.interval {
            let mut interval = interval_at(Instant::now() + interval, interval);

            loop {
                interval.tick().await;
                let self_clone = self.clone();
                task::spawn(async move {
                    match self_clone.query::<R>().await {
                        Ok(response) => {
                            if let Some(ref tcp_server) = self_clone.tcp_server {
                                if let Err(e) = tcp_server.send(response).await {
                                    error!("Error sending to TCP server: {:?}", e);
                                }
                            } else {
                                info!("Received response: {:?}", response);
                            }
                        }
                        Err(e) => error!("Error querying endpoint: {:?}", e),
                    }
                });
            }
        } else {
            let self_clone = self.clone();
            let handle = task::spawn(async move {
                match self_clone.query::<R>().await {
                    Ok(response) => {
                        if let Some(ref tcp_server) = self_clone.tcp_server {
                            if let Err(e) = tcp_server.send(response).await {
                                error!("Error sending to TCP server: {:?}", e);
                            }
                        } else {
                            info!("Received response: {:?}", response);
                        }
                    }
                    Err(e) => error!("Error querying endpoint: {:?}", e),
                }
            });

            handle.await.map_err(|e| {
                error!("Task join error: {:?}", e);
                HttpDataSourceError::TcpError(std::io::Error::new(std::io::ErrorKind::Other, e))
            })?;
        }

        Ok(())
    }
    
    async fn query<R: DeserializeOwned>(&self) -> Result<Option<R>, HttpClientError> {
        let body = self.body.as_ref().map(|b| serde_json::to_value(b).unwrap());

        self.http_client
            .request::<R, _>(
                &self.method,
                &self.url,
                body.as_ref(),
                self.headers.clone(),
                self.query_params.clone(),
                None,
            )
            .await
    }
}
