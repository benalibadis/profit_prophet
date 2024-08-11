use serde::Deserialize;
use tokio::time::{sleep, Duration};
use connector::http::{HttpClient, HttpRequest, HttpResponse, HttpClientError};

const RISK_TOLERANCE: f64 = 0.02;
const STOP_LOSS_THRESHOLD: f64 = 0.98;
const TAKE_PROFIT_THRESHOLD: f64 = 1.05;
const INITIAL_LIMIT: usize = 100;  // Fetch this many candlesticks initially
const SUBSEQUENT_LIMIT: usize = 1; // Fetch only the latest candlestick subsequently
const MAX_DCA_STEPS: usize = 3; // Maximum steps for DCA

#[derive(Clone, Copy, Debug, PartialEq)]
enum Indicator {
    BollingerBands { period: usize, multiplier: f64 },
    RSI { period: usize },
    MACD { short_period: usize, long_period: usize, signal_period: usize },
    // Add more indicators as needed
}

#[derive(Clone, Debug, PartialEq)]
enum Condition {
    RSI { below: Option<f64>, above: Option<f64> },
    BollingerBands { below_lower: bool, above_upper: bool },
    MACD { cross_above: bool, cross_below: bool },
    StopLossPercent { loss_percent: f64 },
    TakeProfitPercent { win_percent: f64 },
    TimeBasedExit { max_duration: usize },  // New condition for time-based exit
    // Add more conditions as needed
}

#[derive(Clone, Debug, PartialEq)]
enum ConditionGroup {
    Single(Condition),
    And(Vec<ConditionGroup>),
    Or(Vec<ConditionGroup>),
}

impl ConditionGroup {
    fn evaluate(&self, rsi: Option<f64>, bb_lower: Option<f64>, bb_upper: Option<f64>, macd: Option<f64>, current_profit: f64, close: f64, candle_timestamp: u64, entry_timestamp: u64) -> bool {
        match self {
            ConditionGroup::Single(cond) => {
                match cond {
                    Condition::RSI { below, above } => {
                        if let Some(rsi_value) = rsi {
                            if let Some(below_value) = below {
                                if rsi_value < *below_value {
                                    return true;
                                }
                            }
                            if let Some(above_value) = above {
                                if rsi_value > *above_value {
                                    return true;
                                }
                            }
                        }
                    }
                    Condition::BollingerBands { below_lower, above_upper } => {
                        if let (Some(bb_lower), Some(bb_upper)) = (bb_lower, bb_upper) {
                            if *below_lower && close < bb_lower {
                                return true;
                            }
                            if *above_upper && close > bb_upper {
                                return true;
                            }
                        }
                    }
                    Condition::MACD { cross_above, cross_below } => {
                        if let Some(macd_value) = macd {
                            if *cross_above && macd_value > 0.0 {
                                return true;
                            }
                            if *cross_below && macd_value < 0.0 {
                                return true;
                            }
                        }
                    }
                    Condition::StopLossPercent { loss_percent } => {
                        if current_profit <= -*loss_percent {
                            return true;
                        }
                    }
                    Condition::TakeProfitPercent { win_percent } => {
                        if current_profit >= *win_percent {
                            return true;
                        }
                    }
                    Condition::TimeBasedExit { max_duration } => {
                        if (candle_timestamp - entry_timestamp) >= (*max_duration as u64) * 60 {
                            return true;
                        }
                    }
                }
                false
            }
            ConditionGroup::And(conditions) => {
                conditions.iter().all(|condition| {
                    condition.evaluate(rsi, bb_lower, bb_upper, macd, current_profit, close, candle_timestamp, entry_timestamp)
                })
            }
            ConditionGroup::Or(conditions) => {
                conditions.iter().any(|condition| {
                    condition.evaluate(rsi, bb_lower, bb_upper, macd, current_profit, close, candle_timestamp, entry_timestamp)
                })
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Action {
    Buy,
    Sell,
    StopLoss,
    TakeProfit,
    PartialTakeProfit(f64), // New action for partial profit-taking
}

#[derive(Clone, Debug)]
struct Rule {
    condition_group: ConditionGroup,
    action: Action,
}

impl Rule {
    fn new(condition_group: ConditionGroup, action: Action) -> Self {
        Rule { condition_group, action }
    }
}

#[derive(Clone)]
struct StrategyConfig {
    indicators: Vec<Indicator>,
    rules: Vec<Rule>,
}

impl StrategyConfig {
    fn new(indicators: Vec<Indicator>, rules: Vec<Rule>) -> Self {
        StrategyConfig { indicators, rules }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct Candlestick {
    timestamp: u64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

async fn fetch_candlestick_data(
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
        body: None::<()>,  // No body needed for GET request
        headers: None,
        query_params: None,
        timeout_duration: Some(Duration::from_secs(10)), // Set a reasonable timeout
    };

    let response: HttpResponse<Vec<Candlestick>> = client.request(request).await?;
    
    match response.body {
        Some(candlesticks) => Ok(candlesticks),
        None => Err(HttpClientError::DeserializeError("No candlestick data found in the response".to_string())),
    }
}

// Function to calculate the Simple Moving Average (SMA)
fn calculate_sma(prices: &[f64], period: usize) -> Vec<f64> {
    prices
        .windows(period)
        .map(|window| window.iter().sum::<f64>() / period as f64)
        .collect()
}

// Function to calculate Bollinger Bands
fn calculate_bollinger_bands(prices: &[f64], period: usize, multiplier: f64) -> Vec<(f64, f64)> {
    if prices.len() < period {
        return vec![(0.0, 0.0); prices.len()];
    }

    let sma = calculate_sma(prices, period);
    sma.iter().enumerate().map(|(i, &ma)| {
        let std_dev: f64 = prices[i..i + period]
            .iter()
            .map(|price| (price - ma).powi(2))
            .sum::<f64>()
            .sqrt()
            / period as f64;
        let upper_band = ma + multiplier * std_dev;
        let lower_band = ma - multiplier * std_dev;
        (upper_band, lower_band)
    }).collect()
}

// Function to calculate RSI
fn calculate_rsi(prices: &[f64], period: usize) -> Vec<f64> {
    if prices.len() < period + 1 {
        return vec![0.0; prices.len()];
    }

    let mut rsi_values = vec![0.0; prices.len()];
    let mut gain_sum = 0.0;
    let mut loss_sum = 0.0;
    
    for i in 1..=period {
        let change = prices[i] - prices[i - 1];
        if change > 0.0 {
            gain_sum += change;
        } else {
            loss_sum -= change;
        }
    }

    let mut avg_gain = gain_sum / period as f64;
    let mut avg_loss = loss_sum / period as f64;

    rsi_values[period] = 100.0 - (100.0 / (1.0 + avg_gain / avg_loss));

    for i in (period + 1)..prices.len() {
        let change = prices[i] - prices[i - 1];
        let gain = if change > 0.0 { change } else { 0.0 };
        let loss = if change < 0.0 { -change } else { 0.0 };

        avg_gain = (avg_gain * (period as f64 - 1.0) + gain) / period as f64;
        avg_loss = (avg_loss * (period as f64 - 1.0) + loss) / period as f64;

        rsi_values[i] = 100.0 - (100.0 / (1.0 + avg_gain / avg_loss));
    }

    rsi_values
}

// Function to calculate EMA (Exponential Moving Average)
fn calculate_ema(prices: &[f64], period: usize) -> Vec<f64> {
    let mut ema_values = Vec::with_capacity(prices.len());
    let smoothing_constant = 2.0 / (period as f64 + 1.0);
    let mut ema = prices[0]; // Start with the first price as the initial EMA

    for &price in prices.iter() {
        ema = (price - ema) * smoothing_constant + ema;
        ema_values.push(ema);
    }

    ema_values
}

// Function to calculate MACD
fn calculate_macd(prices: &[f64], short_period: usize, long_period: usize, signal_period: usize) -> Vec<(f64, f64)> {
    if prices.len() < long_period {
        return vec![(0.0, 0.0); prices.len()];
    }

    let short_ema = calculate_ema(prices, short_period);
    let long_ema = calculate_ema(prices, long_period);

    let macd_line: Vec<f64> = short_ema.iter().zip(long_ema.iter()).map(|(short, long)| short - long).collect();
    let signal_line = calculate_ema(&macd_line, signal_period);

    macd_line.into_iter().zip(signal_line.into_iter()).collect()
}

fn execute_trades(candles: &[Candlestick], config: &StrategyConfig, token_name: &str) {
    let mut sol_balance = 1.0;
    let mut token_balance = 0.0;
    let mut stop_loss_price = None;
    let mut take_profit_price = None;
    let mut dca_step = 0; // Track DCA steps
    let mut entry_timestamp = None;

    let prices: Vec<f64> = candles.iter().map(|c| c.close).collect();

    let mut bb_bands = None;
    let mut rsi_values = None;
    let mut macd_values = None;

    for indicator in &config.indicators {
        match indicator {
            Indicator::BollingerBands { period, multiplier } => {
                bb_bands = Some(calculate_bollinger_bands(&prices, *period, *multiplier));
            }
            Indicator::RSI { period } => {
                rsi_values = Some(calculate_rsi(&prices, *period));
            }
            Indicator::MACD { short_period, long_period, signal_period } => {
                macd_values = Some(calculate_macd(&prices, *short_period, *long_period, *signal_period));
            }
        }
    }

    let last_candle_index = candles.len() - 1;

    let bb_upper = bb_bands
        .as_ref()
        .and_then(|bb| if last_candle_index < bb.len() { Some(bb[last_candle_index].0) } else { None });

    let bb_lower = bb_bands
        .as_ref()
        .and_then(|bb| if last_candle_index < bb.len() { Some(bb[last_candle_index].1) } else { None });

    let rsi = rsi_values
        .as_ref()
        .and_then(|rsi| if last_candle_index < rsi.len() { Some(rsi[last_candle_index]) } else { None });

    let macd = macd_values
        .as_ref()
        .and_then(|macd| if last_candle_index < macd.len() { Some(macd[last_candle_index].0) } else { None });

    let close = candles[last_candle_index].close;
    let candle_timestamp = candles[last_candle_index].timestamp;

    // Calculate percentage changes for stop loss and take profit
    let initial_buy_price = candles.first().unwrap().close;
    let current_profit = (close - initial_buy_price) / initial_buy_price * 100.0;

    // Print with precision to handle small numbers
    println!(
        "{} - Processing candle {}: close = {:.20e}, BB_upper = {:.20e}, BB_lower = {:.20e}, RSI = {:.2}, MACD = {:.20e}, Current Profit = {:.2}%",
        token_name, last_candle_index, close, bb_upper.unwrap_or(0.0), bb_lower.unwrap_or(0.0), rsi.unwrap_or(0.0), macd.unwrap_or(0.0), current_profit
    );

    for rule in &config.rules {
        if rule.condition_group.evaluate(rsi, bb_lower, bb_upper, macd, current_profit, close, candle_timestamp, entry_timestamp.unwrap_or(candle_timestamp)) {
            execute_action(
                rule.action,
                &mut sol_balance,
                &mut token_balance,
                &mut stop_loss_price,
                &mut take_profit_price,
                close,
                token_name,
                &mut dca_step,
                &mut entry_timestamp,
                candle_timestamp // Pass candle_timestamp here
            );
        }
    }

    println!("{} Final SOL balance: {:.20e}", token_name, sol_balance);
    println!("{} Final Token balance: {:.20}", token_name, token_balance);
}

fn execute_action(
    action: Action,
    sol_balance: &mut f64,
    token_balance: &mut f64,
    stop_loss_price: &mut Option<f64>,
    take_profit_price: &mut Option<f64>,
    close: f64,
    token_name: &str,
    dca_step: &mut usize,
    entry_timestamp: &mut Option<u64>,
    candle_timestamp: u64  // Add candle_timestamp parameter
) {
    match action {
        Action::Buy => {
            if *sol_balance > 0.0 && *dca_step < MAX_DCA_STEPS {
                let risk_amount = *sol_balance * RISK_TOLERANCE / (*dca_step + 1) as f64;
                let tokens_bought = risk_amount / (close * STOP_LOSS_THRESHOLD);
                *token_balance += tokens_bought;
                *sol_balance -= tokens_bought * close;
                *stop_loss_price = Some(close * STOP_LOSS_THRESHOLD);
                *take_profit_price = Some(close * TAKE_PROFIT_THRESHOLD);
                *dca_step += 1;
                *entry_timestamp = Some(entry_timestamp.unwrap_or(candle_timestamp)); // Use candle_timestamp here
                println!(
                    "{} DCA BUY: {:.20} tokens at {:.20e} SOL (Stop Loss: {:.20e}, Take Profit: {:.20e})",
                    token_name, tokens_bought, close, stop_loss_price.unwrap(), take_profit_price.unwrap()
                );
            }
        }
        Action::Sell => {
            if *token_balance > 0.0 {
                let sol_gained = *token_balance * close;
                *sol_balance += sol_gained;
                *token_balance = 0.0;
                *stop_loss_price = None;
                *take_profit_price = None;
                *dca_step = 0;
                *entry_timestamp = None;
                println!("{} SELL: {:.20e} SOL at {:.20e} SOL", token_name, sol_gained, close);
            }
        }
        Action::StopLoss => {
            if let Some(sl) = stop_loss_price {
                if close <= *sl && *token_balance > 0.0 {
                    let sol_gained = *token_balance * close;
                    *sol_balance += sol_gained;
                    *token_balance = 0.0;
                    *stop_loss_price = None;
                    *take_profit_price = None;
                    *dca_step = 0;
                    *entry_timestamp = None;
                    println!("{} STOP LOSS: Sold for {:.20e} SOL at {:.20e} SOL", token_name, sol_gained, close);
                }
            }
        }
        Action::TakeProfit => {
            if let Some(tp) = take_profit_price {
                if close >= *tp && *token_balance > 0.0 {
                    let sol_gained = *token_balance * close;
                    *sol_balance += sol_gained;
                    *token_balance = 0.0;
                    *stop_loss_price = None;
                    *take_profit_price = None;
                    *dca_step = 0;
                    *entry_timestamp = None;
                    println!("{} TAKE PROFIT: Sold for {:.20e} SOL at {:.20e} SOL", token_name, sol_gained, close);
                }
            }
        }
        Action::PartialTakeProfit(percentage) => {
            if *token_balance > 0.0 {
                let tokens_to_sell = *token_balance * percentage;
                let sol_gained = tokens_to_sell * close;
                *sol_balance += sol_gained;
                *token_balance -= tokens_to_sell;
                println!(
                    "{} PARTIAL TAKE PROFIT: Sold {:.20} tokens for {:.20e} SOL at {:.20e} SOL",
                    token_name, tokens_to_sell, sol_gained, close
                );
            }
        }
    }
}

async fn process_token(client: &HttpClient, token_address: &str, token_name: &str, config: &StrategyConfig) {
    let mut first_iteration = true;
    let mut candles: Vec<Candlestick> = Vec::new();

    loop {
        let limit = if first_iteration {
            INITIAL_LIMIT  // Get a good history on the first iteration
        } else {
            SUBSEQUENT_LIMIT  // Fetch only the latest candlestick on subsequent iterations
        };

        match fetch_candlestick_data(&client, token_address, 5, 0, limit).await {
            Ok(mut new_candles) => {
                if first_iteration {
                    // On the first iteration, build the initial history
                    candles.append(&mut new_candles);
                    first_iteration = false;  // Set flag to false after the first iteration
                } else {
                    // On subsequent iterations, only add the latest candle and make a trading decision
                    if let Some(latest_candle) = new_candles.last() {
                        // Check if the last candle in `candles` has a different timestamp
                        if candles.last().map_or(true, |c| c.timestamp != latest_candle.timestamp) {
                            // Add the latest candle to the history
                            candles.push(latest_candle.clone());

                            // Remove the oldest candle to maintain the window size, if necessary
                            if candles.len() > INITIAL_LIMIT {
                                candles.remove(0);
                            }
                        } else {
                            println!("Duplicate candle detected, skipping insertion.");
                        }
                    }
                }

                // Calculate indicators and make trading decisions based on the entire dataset
                if !candles.is_empty() {
                    execute_trades(&candles, config, token_name);
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch candlestick data for {}: {:?}", token_name, e);
            }
        }

        // Wait for 5 minutes before the next iteration
        sleep(Duration::from_secs(300)).await;
    }
}

#[tokio::main]
async fn main() {
    let client = HttpClient::new();
    let tokens = vec![
        ("BiVrV7ziFCtATKP6RgYwyhxzKYjm2nbePu1pZZ3vpump", "TOKEN1"),
        // Add more tokens as needed
    ];

    // Example strategy configuration with specific parameters
    let strategy_config = StrategyConfig::new(
        vec![
            Indicator::BollingerBands { period: 20, multiplier: 2.0 },
            Indicator::RSI { period: 14 },
            Indicator::MACD { short_period: 12, long_period: 26, signal_period: 9 },
        ],
        vec![
            Rule::new(
                ConditionGroup::And(vec![
                    ConditionGroup::Single(Condition::RSI { below: Some(30.0), above: None }),
                    ConditionGroup::Single(Condition::BollingerBands { below_lower: true, above_upper: false }),
                ]),
                Action::Buy
            ),
            Rule::new(
                ConditionGroup::Or(vec![
                    ConditionGroup::Single(Condition::RSI { below: None, above: Some(70.0) }),
                    ConditionGroup::Single(Condition::MACD { cross_below: true, cross_above: false }),
                ]),
                Action::Sell
            ),
            Rule::new(ConditionGroup::Single(Condition::StopLossPercent { loss_percent: 5.0 }), Action::StopLoss),
            Rule::new(ConditionGroup::Single(Condition::TakeProfitPercent { win_percent: 10.0 }), Action::TakeProfit),
            Rule::new(ConditionGroup::Single(Condition::TimeBasedExit { max_duration: 60 }), Action::Sell), // Time-based exit after 1 hour
            Rule::new(ConditionGroup::Single(Condition::TakeProfitPercent { win_percent: 5.0 }), Action::PartialTakeProfit(0.5)), // Partial profit at 5% gain
        ]
    );

    let mut tasks = vec![];

    for (token_address, token_name) in tokens {
        let client_clone = client.clone();
        let strategy_config_clone = strategy_config.clone();
        tasks.push(tokio::spawn(async move {
            process_token(&client_clone, token_address, token_name, &strategy_config_clone).await;
        }));
    }

    // Await each task individually
    for task in tasks {
        task.await.unwrap(); // Handle potential panics/errors here
    }
}
