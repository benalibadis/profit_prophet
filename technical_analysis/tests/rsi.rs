#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, RelativeStrengthIndex};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_rsi_next() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices = vec![
             44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 
             45.89, 46.03, 45.61, 46.28
        ].into_iter().map(IndicatorValue::from).collect::<Vec<_>>();

        for price in prices {
            rsi.next(price);
        }

        let result = rsi.next(IndicatorValue::from(46.28));
        assert_eq!(result.to_f64(), 70.53); // Typical example RSI result for 14 periods
    }

    #[test]
    fn test_rsi_next_chunk() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices = vec![
             44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 
             45.89, 46.03, 45.61, 46.28
        ].into_iter().map(IndicatorValue::from).collect::<Vec<_>>();

        let result = rsi.next_chunk(&prices);
        assert_eq!(result.to_f64(), 70.53); // RSI after the chunk of prices
    }

    #[test]
    fn test_rsi_reset() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices = vec![
             44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 
             45.89, 46.03, 45.61, 46.28
        ].into_iter().map(IndicatorValue::from).collect::<Vec<_>>();

        for price in prices {
            rsi.next(price);
        }

        rsi.reset();
        let result = rsi.next(IndicatorValue::from(46.28));
        assert_eq!(result.to_f64(), 0.0); // After reset, RSI should start fresh
    }

    #[test]
    fn test_rsi_with_constant_prices() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices = vec![IndicatorValue::from(50.0); 20]; // Constant prices

        let result = rsi.next_chunk(&prices);
        assert_eq!(result.to_f64(), 50.0); // RSI should be 50 when there's no gain or loss
    }

    #[test]
    fn test_rsi_with_increasing_prices() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices: Vec<IndicatorValue> = (1..=20).map(|x| IndicatorValue::from(x as f64)).collect();

        let result = rsi.next_chunk(&prices);
        assert!(result.to_f64() > 50.0); // RSI should be above 50 with increasing prices
    }

    #[test]
    fn test_rsi_with_decreasing_prices() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices: Vec<IndicatorValue> = (1..=20).rev().map(|x| IndicatorValue::from(x as f64)).collect();

        let result = rsi.next_chunk(&prices);
        assert!(result.to_f64() < 50.0); // RSI should be below 50 with decreasing prices
    }
}
