use connector::http::HttpClient;
use connector::influxdb::InfluxDbClient;
use connector::postgresql::PostgresClient;
use connector::DataConnector;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ConnectorConfig {
    HttpClient,
    InfluxDbClient { url: String, token: String },
    Postgresql { connection_string: String },
}

impl ConnectorConfig {
    pub fn create_connector(&self) -> Arc<dyn DataConnector> {
        match self {
            ConnectorConfig::HttpClient => Arc::new(HttpClient::new()),
            ConnectorConfig::InfluxDbClient { url, token } => Arc::new(InfluxDbClient::new(url, token)),
            ConnectorConfig::Postgresql { connection_string } => {
                let client = async {
                    PostgresClient::new(connection_string).await.unwrap()
                };
                Arc::new(tokio::runtime::Handle::current().block_on(client))
            }
        }
    }
}
