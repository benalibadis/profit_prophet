use thiserror::Error;
use tokio_postgres::Error as PostgresError;
use serde_json::Error as SerdeError;
use crate::http::HttpClientError;

#[derive(Error, Debug)]
pub enum PostgresClientError {
    #[error("HTTP client error: {0}")]
    HttpClientError(#[from] HttpClientError),
    #[error("Database connection error: {0}")]
    ConnectionError(#[from] PostgresError),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] SerdeError),
    #[error("Unsupported value type")]
    UnsupportedValueType(String),
}
