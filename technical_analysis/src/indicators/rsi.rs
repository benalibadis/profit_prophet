use crate::indicators::Indicator;
use crate::IndicatorValue;

#[derive(Debug, Clone)]
pub struct RelativeStrengthIndex {
    period: IndicatorValue,
    period_reciprocal: IndicatorValue,
    avg_gain: IndicatorValue,
    avg_loss: IndicatorValue,
    prev_close: Option<IndicatorValue>,
}

impl RelativeStrengthIndex {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        RelativeStrengthIndex {
            period: period.into(),
            period_reciprocal: IndicatorValue::from(1 / period),
            avg_gain: 0.0.into(),
            avg_loss: 0.0.into(),
            prev_close: None,
        }
    }

    #[inline(always)]
    fn calculate_rsi(&self) -> IndicatorValue {
        let hundred = IndicatorValue::from(100.0);
        if self.avg_loss.value() == 0.0 {
            hundred
        } else {
            let rs = self.avg_gain / self.avg_loss;
            hundred - (hundred / (IndicatorValue::from(1.0) + rs))
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
            let gain = change.max(0.0.into());
            let loss = (-change).max(0.0.into());

            let period_minus_one = self.period - 1.0.into();

            self.avg_gain = (self.avg_gain * period_minus_one + gain) * self.period_reciprocal;
            self.avg_loss = (self.avg_loss * period_minus_one + loss) * self.period_reciprocal;
        } else {
            self.prev_close = Some(input);
            return IndicatorValue::from(50.0);
        }

        self.prev_close = Some(input);
        self.calculate_rsi()
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(self.calculate_rsi(), |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.avg_gain = 0.0.into();
        self.avg_loss = 0.0.into();
        self.prev_close = None;
    }
}
