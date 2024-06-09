use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::timeout;

#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Timeout error: {0}")]
    TimeoutError(#[from] tokio::time::error::Elapsed),
    #[error("Failed to deserialize response: {0}")]
    DeserializeError(String),
    #[error("Invalid HTTP method: {0}")]
    InvalidMethodError(String),
}

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            client: Client::new(),
        }
    }

    pub async fn request<T: DeserializeOwned, U: Serialize>(
        &self,
        method: &str,
        url: &str,
        body: Option<&U>,
        headers: Option<HashMap<String, String>>,
        query_params: Option<HashMap<String, String>>,
        timeout_duration: Option<Duration>,
    ) -> Result<T, HttpClientError> {
        
        let method: Method = method.parse().map_err(|_| HttpClientError::InvalidMethodError(method.to_string()))?;
        
        let mut request_builder = self.client.request(method.clone(), url);

        if let Some(h) = headers {
            for (key, value) in h {
                request_builder = request_builder.header(&key, &value);
            }
        }

        if let Some(params) = query_params {
            request_builder = request_builder.query(&params);
        }

        if let Some(b) = body {
            request_builder = request_builder.json(b);
        }

        let request = request_builder.build()?;

        let response = match timeout_duration {
            Some(duration) => timeout(duration, self.client.execute(request)).await??,
            None => self.client.execute(request).await?,
        };

        let response_text = response.text().await.map_err(HttpClientError::RequestError)?;

        serde_json::from_str::<T>(&response_text).map_err(|_| HttpClientError::DeserializeError(response_text))
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
