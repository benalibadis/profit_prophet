use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct MeanAbsDev {
    buffer: CircularBuffer,
    sum: IndicatorValue,
    mean: IndicatorValue,
    reciprocal_period: IndicatorValue,
}

impl MeanAbsDev {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        let reciprocal_period = 1.0 / period as f64;
        MeanAbsDev {
            buffer: CircularBuffer::new(period),
            sum: 0.0.into(),
            mean: 0.0.into(),
            reciprocal_period: reciprocal_period.into(),
        }
    }

    #[inline(always)]
    fn update_mean(&mut self, new_value: IndicatorValue, old_value: IndicatorValue) {
        self.sum += new_value - old_value;
        self.mean = self.sum * self.reciprocal_period;
    }

    #[inline(always)]
    fn calculate_mad(&self) -> IndicatorValue {
        self.buffer.iter().map(|value| (value - self.mean).abs()).sum::<IndicatorValue>() * self.reciprocal_period
    }
}

impl Default for MeanAbsDev {
    fn default() -> Self {
        MeanAbsDev::new(20)
    }
}

impl Indicator for MeanAbsDev {
    type Input = IndicatorValue;
    type Output = IndicatorValue;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let old_value = if self.buffer.is_full() {
            self.buffer.push(input)
        } else {
            self.sum += input;
            self.buffer.push(input);
            0.0.into()
        };

        if self.buffer.is_full() {
            self.update_mean(input, old_value);
        } else {
            self.mean = self.sum / (self.buffer.len() as f64).into();
        }

        self.calculate_mad()
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(0.0.into(), |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.buffer.clear();
        self.sum = 0.0.into();
        self.mean = 0.0.into();
    }
}
