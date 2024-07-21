use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use connector::influxdb::{InfluxDbClient, InfluxDbDataPoint, InfluxDbClientError};
use log::{info, debug, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct TcpDataPoint {
    measurement: String,
    tags: HashMap<String, String>,
    fields: HashMap<String, serde_json::Value>,
    timestamp: Option<SystemTime>,
}

#[derive(Debug, Error)]
pub enum TcpInfluxDbServerError {
    #[error("TCP server error: {0}")]
    TcpServerError(String),
    #[error("InfluxDB client error: {0}")]
    InfluxDbClientError(#[from] InfluxDbClientError),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Clone, Debug)]
pub struct TcpInfluxDbServer {
    influxdb_client: InfluxDbClient,
    address: SocketAddr,
}

impl TcpInfluxDbServer {
    pub fn new(influxdb_client: InfluxDbClient, address: SocketAddr) -> Self {
        TcpInfluxDbServer {
            influxdb_client,
            address,
        }
    }

    pub async fn run(&self) -> Result<(), TcpInfluxDbServerError> {
        let listener = TcpListener::bind(&self.address)
            .await
            .map_err(|e| TcpInfluxDbServerError::TcpServerError(e.to_string()))?;
        let (tx, mut rx) = mpsc::channel::<Result<TcpDataPoint, TcpInfluxDbServerError>>(32);

        let influxdb_client = self.influxdb_client.clone();

        // Spawn a task to handle incoming connections
        tokio::spawn(async move {
            while let Some(data) = rx.recv().await {
                match data {
                    Ok(data_point) => {
                        // Log the data point before sending to InfluxDB
                        debug!("Received TcpDataPoint: {:?}", data_point);

                        // Convert TcpDataPoint to InfluxDbDataPoint
                        let influx_data_point = InfluxDbDataPoint {
                            measurement: data_point.measurement,
                            tags: data_point.tags,
                            fields: data_point.fields,
                            timestamp: data_point.timestamp,
                        };

                        // Write data to InfluxDB
                        if let Err(e) = influxdb_client.write_data("org", "bucket", influx_data_point).await {
                            error!("Failed to write data to InfluxDB: {:?}", e);
                        } else {
                            info!("Successfully wrote data to InfluxDB");
                        }
                    }
                    Err(e) => error!("Failed to process data: {:?}", e),
                }
            }
        });

        info!("TCP Server listening on {}", self.address);

        loop {
            let (socket, _) = listener.accept().await.map_err(|e| TcpInfluxDbServerError::TcpServerError(e.to_string()))?;
            let tx = tx.clone();

            tokio::spawn(async move {
                let reader = BufReader::new(socket);
                let mut lines = reader.lines();
                while let Some(line) = lines.next_line().await.unwrap_or(None) {
                    match serde_json::from_str::<TcpDataPoint>(&line) {
                        Ok(data_point) => {
                            debug!("Deserialized TcpDataPoint: {:?}", data_point);
                            if let Err(e) = tx.send(Ok(data_point)).await {
                                error!("Failed to send data to channel: {:?}", e);
                            }
                        }
                        Err(e) => error!("Failed to deserialize data: {:?}", e),
                    }
                }
            });
        }
    }
}
