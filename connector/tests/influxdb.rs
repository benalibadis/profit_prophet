

#[cfg(test)]
mod tests {
    use connector::influxdb::*;
    use mockito::{mock, server_url};
    use serde::{Serialize};
    use std::time::SystemTime;
    use std::collections::HashMap;
    use mockito::Matcher;

    #[derive(Serialize)]
    struct TestTags {
        tag1: String,
    }

    #[derive(Serialize)]
    struct TestFields {
        field1: i32,
    }

    #[tokio::test]
    async fn test_write_data_success() {
        let _m = mock("POST", "/api/v2/write")
            .match_header("Authorization", "Token my-token")
            .match_header("Content-Type", "text/plain")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("org".to_string(), "test_org".to_string()),
                Matcher::UrlEncoded("bucket".to_string(), "test_bucket".to_string()),
                Matcher::UrlEncoded("precision".to_string(), "ns".to_string()),
            ]))
            .with_status(204)
            .with_body("OK")
            .create();

        let client = InfluxDbClient::new(&server_url(), "my-token");

        let data_point = InfluxDbDataPoint {
            measurement: "test_measurement".to_string(),
            tags: TestTags { tag1: "tag_value".to_string() },
            fields: TestFields { field1: 42 },
            timestamp: Some(SystemTime::now()),
        };

        let result = client
            .write_data("test_org", "test_bucket", data_point)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_write_data_http_error() {
        let _m = mock("POST", "/api/v2/write")
            .match_header("Authorization", "Token my-token")
            .match_header("Content-Type", "text/plain")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("org".to_string(), "test_org".to_string()),
                Matcher::UrlEncoded("bucket".to_string(), "test_bucket".to_string()),
                Matcher::UrlEncoded("precision".to_string(), "ns".to_string()),
            ]))
            .with_status(500)
            .create();

        let client = InfluxDbClient::new(&server_url(), "my-token");

        let data_point = InfluxDbDataPoint {
            measurement: "test_measurement".to_string(),
            tags: TestTags { tag1: "tag_value".to_string() },
            fields: TestFields { field1: 42 },
            timestamp: Some(SystemTime::now()),
        };

        let result = client
            .write_data("test_org", "test_bucket", data_point)
            .await;
        assert!(result.is_err());
        if let InfluxDbClientError::HttpClientError(_) = result.unwrap_err() {
        } else {
            panic!("Expected HttpClientError");
        }
    }

    #[tokio::test]
    async fn test_write_data_unspported_value_error() {
        let client = InfluxDbClient::new(&server_url(), "my-token");

        // Create a data point with an unsupported tag value type to trigger serialization error
        let data_point = InfluxDbDataPoint {
            measurement: "test_measurement".to_string(),
            tags: HashMap::from([
                ("unsupported".to_string(), serde_json::Value::Null)
            ]),
            fields: TestFields { field1: 42 },
            timestamp: Some(SystemTime::now()),
        };

        let result = client
            .write_data("test_org", "test_bucket", data_point)
            .await;

        assert!(result.is_err());
        println!("HERE => {:?}", result);
        if let InfluxDbClientError::UnsupportedValueType(_) = result.unwrap_err() {
        } else {
            panic!("Expected UnsupportedValueType");
        }
    }
}
