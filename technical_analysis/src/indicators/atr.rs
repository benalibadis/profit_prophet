use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct AverageTrueRange {
    buffer: CircularBuffer,
    period: usize,
    prev_close: IndicatorValue,
    running_sum: IndicatorValue,
    count: usize,
}

impl AverageTrueRange {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        AverageTrueRange {
            buffer: CircularBuffer::new(period),
            period,
            prev_close: 0.0.into(),
            running_sum: 0.0.into(),
            count: 0,
        }
    }
}

impl Default for AverageTrueRange {
    fn default() -> Self {
        AverageTrueRange::new(14)
    }
}

impl Indicator for AverageTrueRange {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = IndicatorValue;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close) = input;

        let true_range = if self.count == 0 {
            high - low
        } else {
            (high - low)
                .max((high - self.prev_close).abs())
                .max((low - self.prev_close).abs())
        };

        if self.buffer.is_full() {
            let oldest_tr = self.buffer.push(true_range);
            self.running_sum += true_range - oldest_tr;
        } else {
            self.buffer.push(true_range);
            self.running_sum += true_range;
            self.count += 1;
        }

        self.prev_close = close;

        self.running_sum / (self.count.min(self.period) as f64).into()
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(self.prev_close, |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.buffer.clear();
        self.prev_close = 0.0.into();
        self.running_sum = 0.0.into();
        self.count = 0;
    }
}
