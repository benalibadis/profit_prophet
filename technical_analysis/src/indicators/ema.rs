use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct ExponentialMovingAverage {
    period: usize,
    multiplier: IndicatorValue,
    current_ema: Option<IndicatorValue>,
}

impl ExponentialMovingAverage {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        let multiplier = IndicatorValue::from(2.0) / IndicatorValue::from(period + 1);
        ExponentialMovingAverage {
            period,
            multiplier,
            current_ema: None,
        }
    }
}

impl Default for ExponentialMovingAverage {
    fn default() -> Self {
        ExponentialMovingAverage::new(14) // Default period of 14
    }
}

impl Indicator for ExponentialMovingAverage {
    type Input = IndicatorValue;
    type Output = IndicatorValue;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        match self.current_ema {
            Some(previous_ema) => {
                let ema = (input - previous_ema) * self.multiplier + previous_ema;
                self.current_ema = Some(ema);
                ema
            }
            None => {
                self.current_ema = Some(input);
                input
            }
        }
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(self.current_ema.unwrap_or(0.0.into()), |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.current_ema = None;
    }
}
