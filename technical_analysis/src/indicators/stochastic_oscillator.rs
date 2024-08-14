use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct StochasticOscillator {
    high_buffer: CircularBuffer,
    low_buffer: CircularBuffer,
    period: usize,
}

pub struct StochasticOutput {
    pub k: IndicatorValue,
    pub d: IndicatorValue,
}

impl StochasticOscillator {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        StochasticOscillator {
            high_buffer: CircularBuffer::new(period),
            low_buffer: CircularBuffer::new(period),
            period,
        }
    }
}

impl Default for StochasticOscillator {
    fn default() -> Self {
        StochasticOscillator::new(14) // Default period of 14
    }
}

impl Indicator for StochasticOscillator {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = StochasticOutput;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close) = input;
        self.high_buffer.push(high);
        self.low_buffer.push(low);

        let highest_high = self.high_buffer.iter().max().unwrap();
        let lowest_low = self.low_buffer.iter().min().unwrap();

        let k = (close - lowest_low) / (highest_high - lowest_low) * 100.0.into();
        let d = self.high_buffer.iter().take(3).sum::<IndicatorValue>() / 3.0.into();

        StochasticOutput { k, d }
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(StochasticOutput {
            k: 0.0.into(),
            d: 0.0.into(),
        }, |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.high_buffer.clear();
        self.low_buffer.clear();
    }
}
