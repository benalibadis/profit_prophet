use async_trait::async_trait;
use std::time::Duration;
use crate::trading::{Candlestick, Token, TradingPlatform};
use connector::http::{HttpClient, HttpRequest, HttpResponse, HttpClientError};

pub struct PumpFun;

#[async_trait]
impl TradingPlatform for PumpFun {
    async fn fetch_candlestick_data(
        &self,
        client: &HttpClient,
        token_address: &str,
        timeframe: usize,
        offset: usize,
        limit: usize
    ) -> Result<Vec<Candlestick>, HttpClientError> {
        let url = format!(
            "https://frontend-api.pump.fun/candlesticks/{}?offset={}&limit={}&timeframe={}",
            token_address, offset, limit, timeframe
        );
        
        let request = HttpRequest {
            method: "GET".to_string(),
            url,
            body: None::<()>,
            headers: None,
            query_params: None,
            timeout_duration: Some(Duration::from_secs(10)),
        };

        let response: HttpResponse<Vec<Candlestick>> = client.request(request).await?;
        
        match response.body {
            Some(candlesticks) => Ok(candlesticks),
            None => Err(HttpClientError::DeserializeError("No candlestick data found in the response".to_string())),
        }
    }

    async fn fetch_latest_token(&self, client: &HttpClient) -> Result<Vec<(String, String)>, HttpClientError> {
        let url = "https://frontend-api.pump.fun/coins/latest";
        let request = HttpRequest {
            method: "GET".to_string(),
            url: url.to_string(),
            body: None::<()>,
            headers: None,
            query_params: None,
            timeout_duration: Some(Duration::from_secs(10)),
        };

        let response: HttpResponse<Vec<Token>> = client.request(request).await?;

        match response.body {
            Some(tokens) => Ok(tokens.into_iter().map(|token| (token.address, token.symbol)).collect()),
            None => Err(HttpClientError::DeserializeError("No token data found".to_string())),
        }
    }
}