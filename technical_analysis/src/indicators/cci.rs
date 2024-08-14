use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct CCI {
    buffer: CircularBuffer,
    period: usize,
}

impl CCI {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        CCI {
            buffer: CircularBuffer::new(period),
            period,
        }
    }
}

impl Default for CCI {
    fn default() -> Self {
        CCI::new(20) // Default period of 20
    }
}

impl Indicator for CCI {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = IndicatorValue;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close) = input;
        let typical_price = (high + low + close) / 3.0.into();
        self.buffer.push(typical_price);

        let mean = self.buffer.iter().sum::<IndicatorValue>() / self.period.into();

        let mean_deviation = self.buffer.iter().map(|tp| (tp - mean).abs()).sum::<IndicatorValue>() / self.period.into();

        (typical_price - mean) / (mean_deviation * 0.015.into())
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(0.0.into(), |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.buffer.clear();
    }
}
