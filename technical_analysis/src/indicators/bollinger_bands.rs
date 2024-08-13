use crate::indicators::Indicator;
use crate::indicators::sma::SimpleMovingAverage;
use crate::indicators::stdev::StandardDeviation;
use crate::IndicatorValue;

pub struct BollingerBands {
    multiplier: IndicatorValue,
    sma: SimpleMovingAverage,
    std_dev: StandardDeviation,
}

pub struct BollingerBandsOutput {
    pub upper_band: IndicatorValue,
    pub lower_band: IndicatorValue,
}

impl Default for BollingerBandsOutput {
    fn default() -> Self {
        Self { upper_band: 0.0.into(), lower_band: 0.0.into() }
    }
}

impl Default for BollingerBands {
    fn default() -> Self {
        Self::new(20, 2.0)
    }
}

impl BollingerBands {
    #[inline(always)]
    pub fn new(period: usize, multiplier: f64) -> Self {
        BollingerBands {
            multiplier: multiplier.into(),
            sma: SimpleMovingAverage::new(period),
            std_dev: StandardDeviation::new(period),
        }
    }

    #[inline(always)]
    fn compute_bands(&self, sma_value: IndicatorValue, std_dev_value: IndicatorValue) -> BollingerBandsOutput {
        let upper_band = sma_value + self.multiplier * std_dev_value;
        let lower_band = sma_value - self.multiplier * std_dev_value;
        
        BollingerBandsOutput{
            upper_band,
            lower_band
        }
    }
}

impl Indicator for BollingerBands {
    type Output = BollingerBandsOutput;

    #[inline(always)]
    fn next(&mut self, input: IndicatorValue) -> Self::Output {
        let sma_value = self.sma.next(input);
        let std_dev_value = self.std_dev.next(input);
        self.compute_bands(sma_value, std_dev_value)
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[IndicatorValue]) -> Self::Output {
        input.iter().fold(BollingerBandsOutput::default(), |_, &value| {
            self.next(value)
        })
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.sma.reset();
        self.std_dev.reset();
    }
}
