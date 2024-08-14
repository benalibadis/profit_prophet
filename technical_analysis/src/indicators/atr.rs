use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct AverageTrueRange {
    buffer: CircularBuffer,
    period: usize,
    prev_close: Option<IndicatorValue>,
}

impl AverageTrueRange {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        AverageTrueRange {
            buffer: CircularBuffer::new(period),
            period,
            prev_close: None,
        }
    }
}

impl Default for AverageTrueRange {
    fn default() -> Self {
        AverageTrueRange::new(14) // Default period of 14
    }
}

impl Indicator for AverageTrueRange {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = IndicatorValue;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close) = input;
        let true_range = match self.prev_close {
            Some(prev_close) => (high - low).max((high - prev_close).abs()).max((low - prev_close).abs()),
            None => high - low,
        };

        self.prev_close = Some(close);
        self.buffer.push(true_range);

        self.buffer.iter().sum::<IndicatorValue>() / self.period.into()
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(0.0.into(), |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.buffer.clear();
        self.prev_close = None;
    }
}
