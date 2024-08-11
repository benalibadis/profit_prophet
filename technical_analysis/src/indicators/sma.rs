use crate::indicators::Indicator;
use crate::CircularBuffer;

pub struct SimpleMovingAverage {
    buffer: CircularBuffer,
    sum: f64,
}

impl SimpleMovingAverage {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        SimpleMovingAverage {
            buffer: CircularBuffer::new(period),
            sum: 0.0,
        }
    }
}

impl Default for SimpleMovingAverage {
    fn default() -> Self {
        Self::new(14)
    }
}

impl Indicator for SimpleMovingAverage {
    type Output = f64;

    #[inline(always)]
    fn next(&mut self, input: f64) -> Self::Output {
        let old_value = self.buffer.push(input);
        self.sum += input - old_value;
        self.sum / self.buffer.len() as f64
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[f64]) -> Self::Output {
        let mut result = 0.0;
        for &value in input {
            result = self.next(value);
        }
        result
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.buffer.clear();
        self.sum = 0.0;
    }
}
