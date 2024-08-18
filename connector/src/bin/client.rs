use connector::data_protocol::{FieldValue, PostgresData};
use connector::postgresql::PostgresClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let base_url = "http://localhost:8080";
    let postgres_client = PostgresClient::new(base_url);

    let data = PostgresData {
        table_name: "my_table".to_string(),
        schema: "public".to_string(),
        rows: HashMap::from([
            ("column1".to_string(), FieldValue::String("value1".to_string())),
            ("column2".to_string(), FieldValue::I64(42)),
        ]),
    };

    match postgres_client.insert_data(data).await {
        Ok(Some(message)) => {
            println!("{}", message);
        }
        Ok(None) => {
            println!("No rows affected");
        }
        Err(e) => {
            eprintln!("Failed to write data to PostgreSQL: {}", e);
        }
    }
}
