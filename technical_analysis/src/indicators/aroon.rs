use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct Aroon {
    high_buffer: CircularBuffer,
    low_buffer: CircularBuffer,
    period: usize,
    period_reciprocal: f64,
}

pub struct AroonOutput {
    pub aroon_up: IndicatorValue,
    pub aroon_down: IndicatorValue,
}

impl Aroon {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        Aroon {
            high_buffer: CircularBuffer::new(period),
            low_buffer: CircularBuffer::new(period),
            period,
            period_reciprocal: 1.0 / period as f64
        }
    }

    #[inline(always)]
    fn calculate_aroon(&self) -> AroonOutput {
        let period = self.period as f64;

        let mut highest_index = 0;
        let mut lowest_index = 0;

        let mut highest_high = self.high_buffer.get(0);
        let mut lowest_low = self.low_buffer.get(0);

        for i in 0..self.period {
            let high = self.high_buffer.get(i);
            let low = self.low_buffer.get(i);

            if high > highest_high {
                highest_high = high;
                highest_index = i;
            }
            if low < lowest_low {
                lowest_low = low;
                lowest_index = i;
            }
        }

        let aroon_up = ((period - highest_index as f64) * self.period_reciprocal * 100.0).into();
        let aroon_down = ((period - lowest_index as f64) * self.period_reciprocal * 100.0).into();

        AroonOutput {
            aroon_up,
            aroon_down,
        }
    }
}

impl Default for Aroon {
    fn default() -> Self {
        Aroon::new(14)
    }
}

impl Indicator for Aroon {
    type Input = (IndicatorValue, IndicatorValue);
    type Output = AroonOutput;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low) = input;
        self.high_buffer.push(high);
        self.low_buffer.push(low);

        self.calculate_aroon()
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(AroonOutput {
            aroon_up: 0.0.into(),
            aroon_down: 0.0.into(),
        }, |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.high_buffer.clear();
        self.low_buffer.clear();
    }
}
