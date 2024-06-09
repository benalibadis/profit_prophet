#[cfg(test)]
    mod tests {
        use profit_prophet::connector::http::{HttpClient, HttpClientError};
        use reqwest::Method;
        use mockito::{mock, Matcher};
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Deserialize)]
        struct TestResponse {
            message: String,
        }

        #[tokio::test]
        async fn test_get_request_success() {
            let _m = mock("GET", "/test")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(r#"{"message": "Hello, World!"}"#)
                .create();

            let client = HttpClient::new();
            let url = &mockito::server_url();
            let full_url = format!("{}/test", url);

            let response: TestResponse = client
                .request(Method::GET, &full_url, None::<&()>, None, None, None)
                .await
                .unwrap();

            assert_eq!(response.message, "Hello, World!");
        }

        #[tokio::test]
        async fn test_post_request_success() {
            let _m = mock("POST", "/test")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(r#"{"message": "Data received!"}"#)
                .match_header("content-type", "application/json")
                .match_body(Matcher::JsonString(r#"{"name":"John Doe","age":30}"#.to_string()))
                .create();

            let client = HttpClient::new();
            let url = &mockito::server_url();
            let full_url = format!("{}/test", url);

            #[derive(Debug, Serialize)]
            struct PostBody {
                name: String,
                age: u8,
            }

            let post_body = PostBody {
                name: "John Doe".to_string(),
                age: 30,
            };

            let response: TestResponse = client
                .request(Method::POST, &full_url, Some(&post_body), None, None, None)
                .await
                .unwrap();

            assert_eq!(response.message, "Data received!");
        }

        #[tokio::test]
        async fn test_get_request_deserialize_error() {
            let _m = mock("GET", "/test")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(r#"{"wrong_field": "This will cause a deserialize error"}"#)
                .create();

            let client = HttpClient::new();
            let url = &mockito::server_url();
            let full_url = format!("{}/test", url);

            let result: Result<TestResponse, HttpClientError> = client
                .request(Method::GET, &full_url, None::<&()>, None, None, None)
                .await;

            assert!(matches!(result, Err(HttpClientError::DeserializeError(_))));
        }
    }