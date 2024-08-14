#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::json;
    use connector::{Compression, DataConnector, DataConnectorError, Message, Protocol};
    use connector::postgresql::*;
    use connector::postgresql::PostgresClientError;

    async fn setup_database() -> Result<PostgresClient, PostgresClientError> {
        let client = PostgresClient::new("host=localhost port=5432 user=arslane password=supersecret123 dbname=arslane").await?;

        let create_schema = "CREATE SCHEMA IF NOT EXISTS public;";
        let create_table = "
            CREATE TABLE IF NOT EXISTS public.profit_postgres (
                column1 TEXT,
                column2 TEXT
            );";

        client.execute(create_schema).await?;
        client.execute(create_table).await?;

        Ok(client)
    }

    #[tokio::test]
    async fn test_write_data_success() -> Result<(), PostgresClientError> {
        let client = setup_database().await?;

        let mut rows = HashMap::new();
        rows.insert("column1".to_string(), json!("value1"));
        rows.insert("column2".to_string(), json!("value2"));

        let data = PostgresData {
            table_name: "profit_postgres".to_string(),
            schema: "public".to_string(),
            rows,
        };

        let result = client.write_data(data).await;

        assert!(result.is_ok(), "Failed to write data: {:?}", result.err());

        Ok(())
    }

    #[tokio::test]
    async fn test_serialization_error() -> Result<(), PostgresClientError> {
        let client = setup_database().await?;

        let malformed_json = json!({
        "unexpected_field": "value1",
        "another_field": "value2"
    });

        let message = Message {
            compression: Compression::None,
            payload: Protocol::Json(malformed_json),
        };

        let result = client.write(message).await;

        assert!(result.is_err(), "Expected a SerializationError, but got success.");
        if let DataConnectorError::PostgresClientError(PostgresClientError::SerializationError(_)) = result.unwrap_err() {
        } else {
            panic!("Expected SerializationError, but got a different error.");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_write_data_nonexistent_table() -> Result<(), PostgresClientError> {
        let client = setup_database().await?;

        let mut rows = HashMap::new();
        rows.insert("column1".to_string(), json!("value1"));
        rows.insert("column2".to_string(), json!("value2"));

        let data = PostgresData {
            table_name: "nonexistent_table".to_string(),
            schema: "public".to_string(),
            rows,
        };

        let result = client.write_data(data).await;

        assert!(result.is_err(), "Expected an error when writing to a nonexistent table, but got success.");
        if let PostgresClientError::PostgresError(_) = result.unwrap_err() {
        } else {
            panic!("Expected PostgresError, but got a different error.");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_write_data_invalid_column() -> Result<(), PostgresClientError> {
        let client = setup_database().await?;

        let mut rows = HashMap::new();
        rows.insert("nonexistent_column".to_string(), json!("value1"));

        let data = PostgresData {
            table_name: "profit_postgres".to_string(),
            schema: "public".to_string(),
            rows,
        };

        let result = client.write_data(data).await;

        assert!(result.is_err(), "Expected an error when writing with an invalid column, but got success.");
        if let PostgresClientError::PostgresError(_) = result.unwrap_err() {
        } else {
            panic!("Expected PostgresError, but got a different error.");
        }

        Ok(())
    }
}
