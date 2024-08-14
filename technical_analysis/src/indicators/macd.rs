use crate::indicators::Indicator;
use crate::IndicatorValue;
use crate::indicators::ExponentialMovingAverage;

pub struct MACD {
    short_ema: ExponentialMovingAverage,
    long_ema: ExponentialMovingAverage,
    signal_ema: ExponentialMovingAverage,
}

pub struct MACDOutput {
    pub macd_value: IndicatorValue,
    pub signal_value: IndicatorValue,
    pub histogram_value: IndicatorValue,
}

impl MACD {
    #[inline(always)]
    pub fn new(short_period: usize, long_period: usize, signal_period: usize) -> Self {
        MACD {
            short_ema: ExponentialMovingAverage::new(short_period),
            long_ema: ExponentialMovingAverage::new(long_period),
            signal_ema: ExponentialMovingAverage::new(signal_period),
        }
    }
}

impl Default for MACD {
    fn default() -> Self {
        MACD::new(12, 26, 9) // Default periods: short=12, long=26, signal=9
    }
}

impl Indicator for MACD {
    type Input = IndicatorValue;
    type Output = MACDOutput;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let short_ema_value = self.short_ema.next(input);
        let long_ema_value = self.long_ema.next(input);
        let macd_value = short_ema_value - long_ema_value;
        let signal_value = self.signal_ema.next(macd_value);
        let histogram_value = macd_value - signal_value;

        MACDOutput {
            macd_value,
            signal_value,
            histogram_value,
        }
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(MACDOutput {
            macd_value: 0.0.into(),
            signal_value: 0.0.into(),
            histogram_value: 0.0.into(),
        }, |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.short_ema.reset();
        self.long_ema.reset();
        self.signal_ema.reset();
    }
}
