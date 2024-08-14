use crate::indicators::Indicator;
use crate::IndicatorValue;

#[derive(Debug, Clone)]
pub struct RelativeStrengthIndex {
    period: IndicatorValue,
    avg_gain: IndicatorValue,
    avg_loss: IndicatorValue,
    prev_close: Option<IndicatorValue>,
}

impl RelativeStrengthIndex {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        RelativeStrengthIndex {
            period: period.into(),
            avg_gain: 0.0.into(),
            avg_loss: 0.0.into(),
            prev_close: None,
        }
    }

    #[inline(always)]
    fn calculate_rsi(&self) -> IndicatorValue {
        if self.avg_loss == 0.0.into() {
            100.0.into()
        } else {
            let rs = self.avg_gain / self.avg_loss;
            IndicatorValue::from(100.0) - (IndicatorValue::from(100.0) / (IndicatorValue::from(1.0) + rs))
        }
    }
}

impl Default for RelativeStrengthIndex {
    fn default() -> Self {
        Self::new(14)
    }
}

impl Indicator for RelativeStrengthIndex {
    type Output = IndicatorValue;
    type Input = IndicatorValue;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        if let Some(prev) = self.prev_close {
            let change = input - prev;
            let (gain, loss) = if change > 0.0.into() {
                (change, 0.0.into())
            } else {
                (0.0.into(), -change)
            };

            self.avg_gain = (self.avg_gain * (self.period - 1.0.into()) + gain) / self.period;
            self.avg_loss = (self.avg_loss * (self.period - 1.0.into()) + loss) / self.period;
        }

        self.prev_close = Some(input);
        self.calculate_rsi()
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(0.0.into(), |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.avg_gain = 0.0.into();
        self.avg_loss = 0.0.into();
        self.prev_close = None;
    }
}
