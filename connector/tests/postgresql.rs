#[cfg(test)]
mod tests {
    use connector::postgresql::PostgresClient;
    use connector::{DataConnector, Message, Protocol};
    use connector::DataConnectorError;

    async fn setup_database(client: &PostgresClient) {
        client.get_client()
            .batch_execute("DROP TABLE IF EXISTS my_table CASCADE;")
            .await
            .unwrap();

        client.get_client()
            .batch_execute(
                "CREATE TABLE my_table (
                id SERIAL PRIMARY KEY,
                data JSONB NOT NULL
            );"
            )
            .await
            .unwrap();
    }

    async fn cleanup_database(client: &PostgresClient) {
        client.get_client()
            .batch_execute("DROP TABLE IF EXISTS my_table;")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_write_data_success() {
        let client = PostgresClient::new("host=localhost user=arslane password=supersecret123 dbname=profitpostgre").await.unwrap();

        setup_database(&client).await;

        let test_payload = serde_json::json!({
            "field1": "value1",
            "field2": "value2"
        });

        let message = Message {
            compression: connector::Compression::None,
            payload: Protocol::Json(test_payload),
        };

        let result = client.write(message.clone()).await;

        assert!(result.is_ok(), "Result was not OK: {:?}", result);

        cleanup_database(&client).await;
    }

    #[tokio::test]
    async fn test_write_data_serialization_error() {
        let client = PostgresClient::new("host=localhost user=arslane password=supersecret123 dbname=profitpostgre").await.unwrap();

        setup_database(&client).await;

        let test_payload = match serde_json::Number::from_f64(f64::NAN) {
            Some(number) => serde_json::json!({"field1": number}),
            None => {
                println!("Cannot create a JSON Number from NaN, skipping serialization test.");
                return;
            }
        };

        let message = Message {
            compression: connector::Compression::None,
            payload: Protocol::Json(test_payload),
        };

        let result = client.write(message).await;

        assert!(matches!(result, Err(DataConnectorError::SerializationError(_))), "Expected SerializationError, got: {:?}", result);

        cleanup_database(&client).await;
    }

    #[tokio::test]
    async fn test_write_data_postgres_error() {
        let client = PostgresClient::new("host=localhost user=arslane password=supersecret123 dbname=profitpostgre").await.unwrap();

        let test_payload = serde_json::json!({
            "field1": "value1",
            "field2": "value2"
        });

        let message = Message {
            compression: connector::Compression::None,
            payload: Protocol::Json(test_payload),
        };

        cleanup_database(&client).await;

        let result = client.write(message).await;

        assert!(matches!(result, Err(DataConnectorError::PostgresError(_))), "Expected PostgresError, got: {:?}", result);
    }
}