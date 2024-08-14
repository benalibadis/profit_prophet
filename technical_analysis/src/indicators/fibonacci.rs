use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct FibonacciRetracement {
    high: IndicatorValue,
    low: IndicatorValue,
}

impl FibonacciRetracement {
    #[inline(always)]
    pub fn new() -> Self {
        FibonacciRetracement {
            high: IndicatorValue::from(0.0),
            low: IndicatorValue::from(0.0),
        }
    }

    #[inline(always)]
    pub fn set_high_low(&mut self, high: IndicatorValue, low: IndicatorValue) {
        self.high = high;
        self.low = low;
    }

    #[inline(always)]
    pub fn levels(&self) -> [IndicatorValue; 6] {
        let diff = self.high - self.low;
        [
            self.high,
            self.low + diff * 0.236.into(),
            self.low + diff * 0.382.into(),
            self.low + diff * 0.5.into(),
            self.low + diff * 0.618.into(),
            self.low,
        ]
    }
}
