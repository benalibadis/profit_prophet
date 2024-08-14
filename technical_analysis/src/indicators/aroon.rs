use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct Aroon {
    high_buffer: CircularBuffer,
    low_buffer: CircularBuffer,
    period: usize,
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
        }
    }
}

impl Default for Aroon {
    fn default() -> Self {
        Aroon::new(14) // Default period of 14
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

        let highest_high = self.high_buffer.iter().max().unwrap();
        let lowest_low = self.low_buffer.iter().min().unwrap();

        let aroon_up = (highest_high / self.period.into()) * 100.0.into();
        let aroon_down = (lowest_low / self.period.into()) * 100.0.into();

        AroonOutput {
            aroon_up,
            aroon_down,
        }
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
