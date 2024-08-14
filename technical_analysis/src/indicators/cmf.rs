use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct ChaikinMoneyFlow {
    buffer: CircularBuffer,
    period: usize,
}

impl ChaikinMoneyFlow {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        ChaikinMoneyFlow {
            buffer: CircularBuffer::new(period),
            period,
        }
    }
}

impl Default for ChaikinMoneyFlow {
    fn default() -> Self {
        ChaikinMoneyFlow::new(20) // Default period of 20
    }
}

impl Indicator for ChaikinMoneyFlow {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = IndicatorValue;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close, volume) = input;
        let mfv = ((close - low) - (high - close)) / (high - low) * volume;

        self.buffer.push(mfv);

        self.buffer.iter().sum::<IndicatorValue>() / self.period.into()
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
