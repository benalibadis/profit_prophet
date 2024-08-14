use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;


pub struct MedianAbsDev {
    buffer: CircularBuffer,
    period: usize,
}

impl MedianAbsDev {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        MedianAbsDev {
            buffer: CircularBuffer::new(period),
            period,
        }
    }
}

impl Default for MedianAbsDev {
    fn default() -> Self {
        MedianAbsDev::new(20) // Default period of 20
    }
}

impl Indicator for MedianAbsDev {
    type Input = IndicatorValue;
    type Output = IndicatorValue;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        self.buffer.push(input);

        let mut values: Vec<_> = self.buffer.iter().collect();
        values.sort_unstable();

        let median = values[values.len() / 2];

        let mut deviations: Vec<_> = self.buffer.iter().map(|value| (value - median).abs()).collect();
        deviations.sort_unstable();

        deviations[deviations.len() / 2]
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
