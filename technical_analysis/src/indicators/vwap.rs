use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct VolumeWeightedAveragePrice {
    cumulative_vp: IndicatorValue,
    cumulative_volume: IndicatorValue,
}

impl VolumeWeightedAveragePrice {
    #[inline(always)]
    pub fn new() -> Self {
        VolumeWeightedAveragePrice {
            cumulative_vp: 0.0.into(),
            cumulative_volume: 0.0.into(),
        }
    }
}

impl Default for VolumeWeightedAveragePrice {
    fn default() -> Self {
        VolumeWeightedAveragePrice::new()
    }
}

impl Indicator for VolumeWeightedAveragePrice {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = IndicatorValue;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close, volume) = input;
        let typical_price = (high + low + close) / 3.0.into();
        self.cumulative_vp += typical_price * volume;
        self.cumulative_volume += volume;

        self.cumulative_vp / self.cumulative_volume
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(0.0.into(), |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.cumulative_vp = 0.0.into();
        self.cumulative_volume = 0.0.into();
    }
}
