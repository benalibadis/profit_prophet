#[cfg(test)]
mod tests {
    use connector::postgresql::*;
    use mockito::{mock, server_url};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_insert_data_success() {
        let _m = mock("POST", "/execute")
            .match_header("Content-Type", "text/plain")
            .with_status(200)
            .create();

        let client = PostgresClient::new(&server_url());

        let data = PostgresData {
            table_name: "test_table".to_string(),
            schema: "public".to_string(),
            rows: HashMap::from([("field1".to_string(), 42.into())]),
        };

        let result = client.insert_data(data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response, None);
    }

    #[tokio::test]
    async fn test_insert_data_http_error() {
        let _m = mock("POST", "/execute")
            .match_header("Content-Type", "text/plain")
            .with_status(500)
            .create();

        let client = PostgresClient::new(&server_url());

        let data = PostgresData {
            table_name: "test_table".to_string(),
            schema: "public".to_string(),
            rows: HashMap::from([("field1".to_string(), 42.into())]),
        };

        let result = client.insert_data(data).await;

        assert!(result.is_err());
        if let PostgresClientError::HttpClientError(_) = result.unwrap_err() {
        } else {
            panic!("Expected HttpClientError");
        }
    }
}
