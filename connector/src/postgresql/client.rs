use crate::connector::{DataConnector, DataConnectorError};
use crate::data_protocol::PostgresData;
use crate::http::{HttpClient, HttpRequest};
use crate::postgresql::error::PostgresClientError;
use crate::{Message, Protocol};
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;


#[derive(Deserialize, Debug)]
struct InsertResponse {
    rows_affected: u64,
}

#[derive(Clone, Debug)]
pub struct PostgresClient {
    http_client: HttpClient,
    base_url: String,
}

impl PostgresClient {
    pub fn new(base_url: &str) -> Self {
        PostgresClient {
            http_client: HttpClient::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn insert_data(
        &self,
        data: PostgresData
    ) -> Result<Option<String>, PostgresClientError> {
        let url = format!("{}/execute", self.base_url);
        let sql_query = data.to_sql_insert();  // Use the to_sql_insert method

        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        let request = HttpRequest {
            method: "POST".to_string(),
            url,
            body: Some(sql_query),
            headers: Some(headers),
            query_params: None,
            timeout_duration: None,
        };

        match self.http_client.request::<String, InsertResponse>(request).await {
            Ok(response) => Ok(Some(format!("Rows affected: {}", response.body.unwrap().rows_affected))),
            Err(err) => Err(PostgresClientError::HttpClientError(err)),
        }
    }

}

#[async_trait]
impl DataConnector for PostgresClient {
    async fn write(&self, data: Message) -> Result<Message, DataConnectorError> {
        let postgres_data: PostgresData = match data.payload {
            Protocol::Json(value) => {
                serde_json::from_value(value).map_err(PostgresClientError::SerializationError)?
            },
        };
        let result = self.insert_data(postgres_data).await?;
        let response_payload = serde_json::to_value(result).map_err(PostgresClientError::SerializationError)?;

        Ok(Message {
            compression: data.compression,
            payload: Protocol::Json(response_payload),
        })
    }

    async fn read(&self, _data: Message) -> Result<Message, DataConnectorError> {
        Err(DataConnectorError::PostgresClientError(
            PostgresClientError::UnsupportedValueType("Read operation is not supported".to_string())
        ))
    }
}
