use crate::{CircularBuffer, IndicatorValue};
use crate::indicators::Indicator;
#[cfg(not(feature = "precision"))]
use std::simd::{f64x4, prelude::SimdFloat};

pub struct StandardDeviation {
    buffer: CircularBuffer,
    sum: IndicatorValue,
    sum_of_squares: IndicatorValue,
}

impl StandardDeviation {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        StandardDeviation {
            buffer: CircularBuffer::new(period),
            sum: 0.0.into(),
            sum_of_squares: 0.0.into(),
        }
    }
}

impl Default for StandardDeviation {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Indicator for StandardDeviation {
    type Output = IndicatorValue;
    type Input = IndicatorValue;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let old_value = self.buffer.push(input);

        // Update sum and sum_of_squares with IndicatorValue
        self.sum -= old_value;
        self.sum_of_squares -= old_value * old_value;

        self.sum += input;
        self.sum_of_squares += input * input;

        let mean = self.sum / self.buffer.len().into();
        let variance = (self.sum_of_squares / self.buffer.len().into()) - (mean * mean);

        variance.sqrt()
    }

    #[cfg(not(feature = "precision"))]
    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        let mut result: IndicatorValue = 0.0.into();

        for chunk in input.chunks_exact(4) {
            let values = f64x4::from_array([
                chunk[0].value(),
                chunk[1].value(),
                chunk[2].value(),
                chunk[3].value(),
            ]);

            let sum_vec = values.reduce_sum();
            let sum_of_squares_vec = (values * values).reduce_sum();

            for &value in chunk {
                let old_value = self.buffer.push(value);

                self.sum -= old_value;
                self.sum_of_squares -= old_value * old_value;
            }

            self.sum += sum_vec.into();
            self.sum_of_squares += sum_of_squares_vec.into();

            let mean = self.sum / self.buffer.len().into();
            let variance = (self.sum_of_squares / self.buffer.len().into()) - (mean * mean);

            result = variance.sqrt();
        }

        result
    }

    #[cfg(feature = "precision")]
    #[inline(always)]
    fn next_chunk(&mut self, input: &[IndicatorValue]) -> Self::Output {
        input.iter().fold(0.0.into(), |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.sum = 0.0.into();
        self.sum_of_squares = 0.0.into();
        self.buffer.clear();
    }
}
