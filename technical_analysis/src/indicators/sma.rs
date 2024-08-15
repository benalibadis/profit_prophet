use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct SimpleMovingAverage {
    buffer: CircularBuffer,
    sum: IndicatorValue,
    inv_period: IndicatorValue,
}

impl SimpleMovingAverage {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        SimpleMovingAverage {
            buffer: CircularBuffer::new(period),
            sum: 0.0.into(),
            inv_period: IndicatorValue::from(1 / period),
        }
    }
}

impl Default for SimpleMovingAverage {
    fn default() -> Self {
        Self::new(14)
    }
}

impl Indicator for SimpleMovingAverage {
    type Output = IndicatorValue;
    type Input = IndicatorValue;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let old_value = self.buffer.push(input);
        self.sum += input - old_value;
        self.sum * self.inv_period
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        let mut result = 0.0.into();
        for &value in input.iter() {
            result = self.next(value);
        }
        result
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.buffer.clear();
        self.sum = 0.0.into();
    }
}
