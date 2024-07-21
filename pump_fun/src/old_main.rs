use std::time::{SystemTime};
use serde::{Serialize};
use connector::influxdb::{InfluxDbClient, InfluxDbDataPoint};

#[tokio::main]
async fn main() {
    let influxdb_url = "http://localhost:8086";
    let org = "arslanelabs";
    let token = "KAtopm-k4IVJUagoQm_obwIZF1WyRfSTzlHIGVkBc91otqztpOjkLnbAu8WJ9H7hVZjiOBSDvlNztkWKpY1OVQ==";
    let bucket = "testoz";

    let influxdb_client = InfluxDbClient::new(influxdb_url, token);

    // Example of writing data
    #[derive(Serialize)]
    struct Tags {
        tag: String,
    }

    #[derive(Serialize)]
    struct Fields {
        field: String,
        another_field: i64
    }

    let tags = Tags { tag: "value".to_string() };
    let fields = Fields { field: "value".to_string(), another_field: 2675 };

    let data = InfluxDbDataPoint {
        measurement: "tozzzzz".to_string(),
        tags,
        fields,
        timestamp: Some(SystemTime::now()),
    };

    match influxdb_client.write_data(org, bucket, data).await {
        Ok(_) => println!("Data written successfully"),
        Err(e) => eprintln!("Failed to write data: {:?}", e),
    }
}
