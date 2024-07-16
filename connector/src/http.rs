use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Timeout error: {0}")]
    TimeoutError(String),
    #[error("Failed to deserialize response: {0}")]
    DeserializeError(String),
    #[error("Invalid HTTP method: {0}")]
    InvalidMethodError(String),
    #[error("HTTP error: {0}")]
    HttpError(String),
}

#[derive(Clone, Debug)]
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
    ) -> Result<Option<T>, HttpClientError> {
        
        let method: Method = method.parse().map_err(|_| HttpClientError::InvalidMethodError(method.to_string()))?;
        
        let mut request_builder = self.client.request(method.clone(), url);

        if let Some(ref h) = headers {
            for (key, value) in h {
                request_builder = request_builder.header(key, value);
            }
        }

        if let Some(params) = query_params {
            request_builder = request_builder.query(&params);
        }


        if let Some(b) = body {
            if let Some(content_type) = headers.as_ref().and_then(|h| h.get("Content-Type")) {
                match content_type.as_str() {
                    "application/json" => {
                        request_builder = request_builder.json(b);
                    }
                    "application/x-www-form-urlencoded" => {
                        if let Ok(form_body) = serde_urlencoded::to_string(b) {
                            request_builder = request_builder.body(form_body);
                        }
                    }
                    "text/plain" => {
                        if let Ok(text_body) = serde_json::to_string(b) {
                            let text_body = text_body.replace("\\\"", "\"").trim_matches('"').to_string();
                            request_builder = request_builder.body(text_body);
                        }
                    }
                    content_type => {
                        todo!("Unmatched Content-Type {}", content_type);
                    }
                }
            } else {
                request_builder = request_builder.json(b);
            }
        }

        if let Some(duration) = timeout_duration {
            request_builder = request_builder.timeout(duration);
        }

        let request = request_builder.build()?;
        let response = self.client.execute(request).await;
        match response {
            Ok(resp) => {
                let status = resp.status();
                if !status.is_success() {
                    return Err(HttpClientError::HttpError(format!("HTTP error: {}", status)));
                }

                let response_text = resp.text().await.map_err(HttpClientError::RequestError)?;
                if response_text.is_empty() {
                    Ok(None)
                } else {
                    serde_json::from_str::<T>(&response_text)
                    .map(Some)
                    .map_err(|_| HttpClientError::DeserializeError(response_text))
                }
            }
            Err(e) if e.is_timeout() => Err(HttpClientError::TimeoutError(format!("Request timed out after {:?}", timeout_duration.unwrap_or_default()))),
            Err(e) => Err(HttpClientError::RequestError(e)),
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
