#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, BollingerBands};

    #[test]
    fn test_bollinger_bands_next() {
        let mut bb = BollingerBands::new(20, 2.0);
        let prices = vec![
            22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29, 
            22.15, 22.39, 22.38, 22.61, 23.36, 24.05, 23.75, 23.83, 23.95, 23.63
        ];

        for price in prices {
            bb.next(price);
        }

        let result = bb.next(23.82);
        assert_eq!(22.41 + 2.0 * 0.52, result.upper_band); // Upper band example
        assert_eq!(22.41 - 2.0 * 0.52, result.lower_band); // Lower band example
    }

    #[test]
    fn test_bollinger_bands_next_chunk() {
        let mut bb = BollingerBands::new(20, 2.0);
        let prices = vec![
            22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29, 
            22.15, 22.39, 22.38, 22.61, 23.36, 24.05, 23.75, 23.83, 23.95, 23.63
        ];

        let result = bb.next_chunk(&prices);
        assert_eq!(22.41 + 2.0 * 0.52, result.upper_band); // Upper band after chunk
        assert_eq!(22.41 - 2.0 * 0.52, result.lower_band); // Lower band after chunk
    }

    #[test]
    fn test_bollinger_bands_reset() {
        let mut bb = BollingerBands::new(20, 2.0);
        let prices = vec![
            22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29, 
            22.15, 22.39, 22.38, 22.61, 23.36, 24.05, 23.75, 23.83, 23.95, 23.63
        ];

        for price in prices {
            bb.next(price);
        }

        bb.reset();
        let result = bb.next(23.82);
        assert_eq!(23.82, result.upper_band); // After reset, it should start fresh
        assert_eq!(23.82, result.lower_band); // After reset, lower band should equal price
    }

    #[test]
    fn test_bollinger_bands_with_constant_prices() {
        let mut bb = BollingerBands::new(20, 2.0);
        let prices = vec![50.0; 20]; // Constant prices

        let result = bb.next_chunk(&prices);
        assert_eq!(result.upper_band, 50.0 + 2.0 * 0.0); // Upper band with no variation
        assert_eq!(result.lower_band, 50.0 - 2.0 * 0.0); // Lower band with no variation
    }

    #[test]
    fn test_bollinger_bands_with_increasing_prices() {
        let mut bb = BollingerBands::new(20, 2.0);
        let prices: Vec<f64> = (1..=20).map(|x| x as f64).collect();

        let result = bb.next_chunk(&prices);
        assert!(result.upper_band > 20.0); // Upper band should be above SMA
        assert!(result.lower_band < 20.0); // Lower band should be below SMA
    }

    #[test]
    fn test_bollinger_bands_with_decreasing_prices() {
        let mut bb = BollingerBands::new(20, 2.0);
        let prices: Vec<f64> = (1..=20).rev().map(|x| x as f64).collect();

        let result = bb.next_chunk(&prices);
        assert!(result.upper_band > result.lower_band); // Upper band should be above lower band
    }
}
