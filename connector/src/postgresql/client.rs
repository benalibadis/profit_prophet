use std::sync::Arc;
use tokio_postgres::{Client, NoTls};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::{Protocol, Message, DataConnectorError};
use crate::connector::{DataConnector};
use crate::postgresql::error::PostgresError;

#[derive(Clone)]
pub struct PostgresClient {
    client: Arc<Client>,
}

impl PostgresClient {
    pub async fn new(connection_string: &str) -> Result<Self, PostgresError> {
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(PostgresClient {
            client: Arc::new(client),
        })
    }

    pub fn get_client(&self) -> Arc<Client> {
        Arc::clone(&self.client)
    }
}

#[async_trait]
impl DataConnector for PostgresClient {
    async fn write(&self, data: Message) -> Result<Message, DataConnectorError> {
        let payload = match data.payload {
            Protocol::Json(ref value) => value.clone(),
        };

        let json_str = serde_json::to_string(&payload).map_err(DataConnectorError::SerializationError)?;

        self.client
            .execute("INSERT INTO my_table (data) VALUES ($1::jsonb)", &[&json_str])
            .await
            .map_err(|e| DataConnectorError::PostgresError(PostgresError::PgError(e)))?;

        Ok(data)
    }

    async fn read(&self, _data: Message) -> Result<Message, DataConnectorError> {
        Err(DataConnectorError::OtherError("Read operation is not supported".to_string()))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostgresConfig {
    pub connection_string: String,
}
