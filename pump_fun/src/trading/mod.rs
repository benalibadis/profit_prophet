pub mod pump_fun;

use serde::Deserialize;
use async_trait::async_trait;
use connector::http::{HttpClient, HttpClientError};

#[derive(Deserialize)]
pub struct Candlestick {
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Deserialize)]
pub struct Token {
    address: String,
    name: String,
    symbol: String,
    description: String,
    twitter: String,
    telegram: String
}

#[async_trait]
pub trait TradingPlatform {
    async fn fetch_candlestick_data(
        &self,
        client: &HttpClient,
        token_address: &str,
        timeframe: usize,
        offset: usize,
        limit: usize
    ) -> Result<Vec<Candlestick>, HttpClientError>;

    async fn fetch_latest_token(&self, client: &HttpClient) -> Result<Vec<(String, String)>, HttpClientError>;
}