use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;
use std::cmp::Ordering;

pub struct MedianAbsDev {
    buffer: CircularBuffer,
    temp_values: Vec<IndicatorValue>,
    temp_deviations: Vec<IndicatorValue>,
}

impl MedianAbsDev {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        MedianAbsDev {
            buffer: CircularBuffer::new(period),
            temp_values: Vec::with_capacity(period),
            temp_deviations: Vec::with_capacity(period),
        }
    }

    #[inline(always)]
    fn quickselect(arr: &mut [IndicatorValue], k: usize) -> IndicatorValue {
        if arr.len() == 1 {
            return arr[0];
        }

        let pivot_idx = arr.len() / 2;
        let pivot = arr[pivot_idx];
        arr.swap(pivot_idx, arr.len() - 1);

        let mut i = 0;
        for j in 0..arr.len() - 1 {
            if arr[j] < pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, arr.len() - 1);

        match i.cmp(&k) {
            Ordering::Equal => arr[i],
            Ordering::Less => MedianAbsDev::quickselect(&mut arr[i + 1..], k - i - 1),
            Ordering::Greater => MedianAbsDev::quickselect(&mut arr[..i], k),
        }
    }

    #[inline(always)]
    fn median(&mut self) -> IndicatorValue {
        self.temp_values.clear();
        self.temp_values.extend(self.buffer.iter());
        let mid = self.temp_values.len() / 2;
        MedianAbsDev::quickselect(&mut self.temp_values, mid)
    }

    #[inline(always)]
    fn median_abs_deviation(&mut self, median: IndicatorValue) -> IndicatorValue {
        self.temp_deviations.clear();
        self.temp_deviations.extend(self.buffer.iter().map(|value| (value - median).abs()));
        let mid = self.temp_deviations.len() / 2;
        MedianAbsDev::quickselect(&mut self.temp_deviations, mid)
    }
}

impl Default for MedianAbsDev {
    fn default() -> Self {
        MedianAbsDev::new(20)
    }
}

impl Indicator for MedianAbsDev {
    type Input = IndicatorValue;
    type Output = IndicatorValue;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        self.buffer.push(input);

        let median = self.median();
        self.median_abs_deviation(median)
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(0.0.into(), |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.buffer.clear();
        self.temp_values.clear();
        self.temp_deviations.clear();
    }
}
