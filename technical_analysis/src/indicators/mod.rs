mod sma;
mod stdev;
mod bollinger_bands;
mod roc;
mod rsi;

pub use sma::SimpleMovingAverage;
pub use stdev::StandardDeviation;
pub use bollinger_bands::{BollingerBands, BollingerBandsOutput};
pub use roc::RateOfChange;
pub use rsi::RelativeStrengthIndex;


pub trait Indicator {
    type Output;

    fn next(&mut self, input: f64) -> Self::Output;
    fn next_chunk(&mut self, input: &[f64]) -> Self::Output;
    fn reset(&mut self);
}