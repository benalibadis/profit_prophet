use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct MeanAbsDev {
    buffer: CircularBuffer,
    period: usize,
}

impl MeanAbsDev {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        MeanAbsDev {
            buffer: CircularBuffer::new(period),
            period,
        }
    }
}

impl Default for MeanAbsDev {
    fn default() -> Self {
        MeanAbsDev::new(20) // Default period of 20
    }
}

impl Indicator for MeanAbsDev {
    type Input = IndicatorValue;
    type Output = IndicatorValue;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        self.buffer.push(input);

        let mean = self.buffer.iter().sum::<IndicatorValue>() / self.period.into();

        self.buffer.iter().map(|value| (value - mean).abs()).sum::<IndicatorValue>() / self.period.into()
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
