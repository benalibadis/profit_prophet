use thiserror::Error;
use tokio_postgres::Error as PgError;

#[derive(Error, Debug)]
pub enum PostgresError {
    #[error("PostgreSQL error: {0}")]
    PgError(#[from] PgError),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
