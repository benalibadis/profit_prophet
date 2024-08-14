mod aroon;
mod atr;
mod bollinger_bands;
mod cci;
mod cmf;
mod ema;
mod fibonacci;
mod ichimoku;
mod macd;
mod mean_abs_std;
mod median_abs_std;
mod obv;
mod roc;
mod rsi;
mod sma;
mod stdev;

pub use aroon::{Aroon, AroonOutput};
pub use atr::AverageTrueRange;
pub use bollinger_bands::{BollingerBands, BollingerBandsOutput};
pub use cci::CCI;
pub use cmf::ChaikinMoneyFlow;
pub use ema::ExponentialMovingAverage;
pub use fibonacci::FibonacciRetracement;
pub use ichimoku::{IchimokuClouds, IchimokuCloudsOutput};
pub use macd::{MACD, MACDOutput};
pub use mean_abs_std::MeanAbsDev;
pub use median_abs_std::MedianAbsDev;
pub use obv::OnBalanceVolume;
pub use roc::RateOfChange;
pub use rsi::RelativeStrengthIndex;
pub use sma::SimpleMovingAverage;
pub use stdev::StandardDeviation;

pub trait Indicator {
    type Output;
    type Input;

    fn next(&mut self, input: Self::Input) -> Self::Output;
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output;
    fn reset(&mut self);
}
