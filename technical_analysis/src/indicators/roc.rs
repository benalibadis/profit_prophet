pub use crate::indicators::Indicator;
use crate::CircularBuffer;

pub struct RateOfChange {
    buffer: CircularBuffer,
}

impl RateOfChange {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        RateOfChange {
            buffer: CircularBuffer::new(period),
        }
    }
}

impl Default for RateOfChange {
    fn default() -> Self {
        Self::new(12)
    }
}

impl Indicator for RateOfChange {
    type Output = f64;

    #[inline(always)]
    fn next(&mut self, input: f64) -> f64 {
        let old_value = self.buffer.push(input);

        if old_value == 0.0 {
            0.0
        } else {
            ((input - old_value) / old_value) * 100.0
        }
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
    }
}
