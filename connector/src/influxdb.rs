use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::http::{HttpClient, HttpClientError};

#[derive(Error, Debug)]
pub enum InfluxDbClientError {
    #[error("HTTP client error: {0}")]
    HttpClientError(#[from] HttpClientError),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Unsupported tag or field value type")]
    UnsupportedValueType(String),
    #[error("Time error: {0}")]
    TimeError(#[from] std::time::SystemTimeError),
}

#[derive(Clone, Debug)]
pub struct InfluxDbClient {
    http_client: HttpClient,
    base_url: String,
    token: String,
}

impl InfluxDbClient {
    pub fn new(base_url: &str, token: &str) -> Self {
        InfluxDbClient {
            http_client: HttpClient::new(),
            base_url: base_url.to_string(),
            token: token.to_string(),
        }
    }

    pub async fn write_data<T: Serialize, F: Serialize>(
        &self,
        org: &str,
        bucket: &str,
        data: InfluxDbDataPoint<T, F>
    ) -> Result<Option<String>, InfluxDbClientError> {
        let url = format!("{}/api/v2/write", self.base_url);
        let mut query_params = HashMap::new();
        query_params.insert("org".to_string(), org.to_string());
        query_params.insert("bucket".to_string(), bucket.to_string());
        query_params.insert("precision".to_string(), data.infer_precision());

        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Token {}", self.token));
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        let data_str = data.to_line_protocol()?;

        // Pass the string directly without additional quotes
        self.http_client
            .request::<String, _>(
                "POST",
                &url,
                Some(&data_str),
                Some(headers),
                Some(query_params),
                None,
            )
            .await
            .map_err(InfluxDbClientError::HttpClientError)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfluxDbDataPoint<T: Serialize, F: Serialize> {
    pub measurement: String,
    pub tags: T,
    pub fields: F,
    pub timestamp: Option<SystemTime>,
}

impl<T: Serialize, F: Serialize> InfluxDbDataPoint<T, F> {
    pub fn to_line_protocol(&self) -> Result<String, InfluxDbClientError> {
        let mut line = self.measurement.to_string();

        let tags_map: HashMap<String, serde_json::Value> = serde_json::from_value(serde_json::to_value(&self.tags)?)?;
        for (k, v) in tags_map {
            let tag_value = match v {
                serde_json::Value::String(s) => format!("{k}=\"{s}\"").to_string(),
                serde_json::Value::Number(n) => format!("{}={}", k, n),
                serde_json::Value::Bool(b) => format!("{}={}", k, b),
                unspported_value => return Err(InfluxDbClientError::UnsupportedValueType(format!("Unsupported tag value: {}", unspported_value))),
            };
            line.push_str(&format!(",{}", tag_value));
        }

        let fields_map: HashMap<String, serde_json::Value> = serde_json::from_value(serde_json::to_value(&self.fields)?)?;
        let mut first_field = true;
        for (k, v) in fields_map {
            let field_value = match v {
                serde_json::Value::String(s) => format!("{}=\"{}\"", k, s),
                serde_json::Value::Number(n) => format!("{}={}", k, n),
                serde_json::Value::Bool(b) => format!("{}={}", k, b),
                unspported_value => return Err(InfluxDbClientError::UnsupportedValueType(format!("Unsupported field value: {}", unspported_value))),
            };

            if first_field {
                line.push_str(&format!(" {}", field_value));
                first_field = false;
            } else {
                line.push_str(&format!(",{}", field_value));
            }
        }

        if let Some(ts) = self.timestamp {
            let duration = ts.duration_since(UNIX_EPOCH)?;
            line.push_str(&format!(" {}", duration.as_nanos()));
        }

        Ok(line)
    }

    pub fn infer_precision(&self) -> String {
        if let Some(ts) = self.timestamp {
            let duration = ts.duration_since(UNIX_EPOCH).expect("Time went backwards");
            let nanos = duration.as_nanos();
            if nanos % 1_000_000_000 == 0 {
                "s".to_string()
            } else if nanos % 1_000_000 == 0 {
                "ms".to_string()
            } else if nanos % 1_000 == 0 {
                "us".to_string()
            } else {
                "ns".to_string()
            }
        } else {
            "ns".to_string() // Default to nanoseconds if no timestamp is provided
        }
    }
}
