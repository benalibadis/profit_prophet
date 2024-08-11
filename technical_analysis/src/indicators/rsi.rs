pub use crate::indicators::Indicator;

#[derive(Debug, Clone)]
pub struct RelativeStrengthIndex {
    period: usize,
    avg_gain: f64,
    avg_loss: f64,
    prev_close: Option<f64>,
}

impl RelativeStrengthIndex {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        RelativeStrengthIndex {
            period,
            avg_gain: 0.0,
            avg_loss: 0.0,
            prev_close: None,
        }
    }

    #[inline(always)]
    fn calculate_rsi(&self) -> f64 {
        if self.avg_loss == 0.0 {
            100.0
        } else {
            let rs = self.avg_gain / self.avg_loss;
            100.0 - (100.0 / (1.0 + rs))
        }
    }
}

impl Default for RelativeStrengthIndex {
    fn default() -> Self {
        Self::new(14)
    }
}

impl Indicator for RelativeStrengthIndex {
    type Output = f64;

    #[inline(always)]
    fn next(&mut self, input: f64) -> Self::Output {
        if let Some(prev) = self.prev_close {
            let change = input - prev;
            let (gain, loss) = if change > 0.0 {
                (change, 0.0)
            } else {
                (0.0, -change)
            };

            // Using single assignment with weighted average update
            let period_f64 = self.period as f64;
            self.avg_gain = (self.avg_gain * (period_f64 - 1.0) + gain) / period_f64;
            self.avg_loss = (self.avg_loss * (period_f64 - 1.0) + loss) / period_f64;
        }

        self.prev_close = Some(input);
        self.calculate_rsi()
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[f64]) -> Self::Output {
        input.iter().fold(0.0, |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.avg_gain = 0.0;
        self.avg_loss = 0.0;
        self.prev_close = None;
    }
}
