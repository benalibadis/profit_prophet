use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct IchimokuClouds {
    high_buffer: CircularBuffer,
    low_buffer: CircularBuffer,
    tenkan_period: usize,
    kijun_period: usize,
    senkou_b_period: usize,
}

pub struct IchimokuCloudsOutput {
    pub tenkan_sen: IndicatorValue,
    pub kijun_sen: IndicatorValue,
    pub senkou_span_a: IndicatorValue,
    pub senkou_span_b: IndicatorValue,
    pub chikou_span: IndicatorValue,
}

impl IchimokuClouds {
    #[inline(always)]
    pub fn new(tenkan_period: usize, kijun_period: usize, senkou_b_period: usize) -> Self {
        IchimokuClouds {
            high_buffer: CircularBuffer::new(senkou_b_period),
            low_buffer: CircularBuffer::new(senkou_b_period),
            tenkan_period,
            kijun_period,
            senkou_b_period,
        }
    }
}

impl Default for IchimokuClouds {
    fn default() -> Self {
        IchimokuClouds::new(9, 26, 52) // Default parameters
    }
}

impl Indicator for IchimokuClouds {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = IchimokuCloudsOutput;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close) = input;
        self.high_buffer.push(high);
        self.low_buffer.push(low);

        // Tenkan-sen calculation
        let tenkan_sen = (self.high_buffer.iter().take(self.tenkan_period).max().unwrap() 
            + self.low_buffer.iter().take(self.tenkan_period).min().unwrap()) / 2.0.into();

        // Kijun-sen calculation
        let kijun_sen = (self.high_buffer.iter().take(self.kijun_period).max().unwrap() 
            + self.low_buffer.iter().take(self.kijun_period).min().unwrap()) / 2.0.into();

        // Senkou Span A calculation (Plotted 26 periods ahead)
        let senkou_span_a = (tenkan_sen + kijun_sen) / 2.0.into();

        // Senkou Span B calculation (Plotted 26 periods ahead)
        let senkou_span_b = (self.high_buffer.iter().take(self.senkou_b_period).max().unwrap() 
            + self.low_buffer.iter().take(self.senkou_b_period).min().unwrap()) / 2.0.into();

        // Chikou Span (Lagging close price)
        let chikou_span = close;

        IchimokuCloudsOutput {
            tenkan_sen,
            kijun_sen,
            senkou_span_a,
            senkou_span_b,
            chikou_span,
        }
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(IchimokuCloudsOutput {
            tenkan_sen: 0.0.into(),
            kijun_sen: 0.0.into(),
            senkou_span_a: 0.0.into(),
            senkou_span_b: 0.0.into(),
            chikou_span: 0.0.into(),
        }, |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.high_buffer.clear();
        self.low_buffer.clear();
    }
}
