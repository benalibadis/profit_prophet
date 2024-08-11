#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, SimpleMovingAverage};

    #[test]
    fn test_sma_next() {
        let mut sma = SimpleMovingAverage::new(3);
        assert_eq!(sma.next(1.0), 1.0);
        assert_eq!(sma.next(2.0), 1.5);
        assert_eq!(sma.next(3.0), 2.0);
        assert_eq!(sma.next(4.0), 3.0);
    }

    #[test]
    fn test_sma_next_chunk() {
        let mut sma = SimpleMovingAverage::new(3);
        let result = sma.next_chunk(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_sma_reset() {
        let mut sma = SimpleMovingAverage::new(3);
        sma.next(1.0);
        sma.next(2.0);
        sma.reset();
        assert_eq!(sma.next(3.0), 3.0);
    }

    #[test]
    fn test_sma_with_large_data() {
        let mut sma = SimpleMovingAverage::new(100);
        let data: Vec<f64> = (1..=1000).map(|x| x as f64).collect();
        let result = sma.next_chunk(&data);
        assert_eq!(result, 950.5); // The average of the last 100 numbers (901 to 1000)
    }

    #[test]
    fn test_sma_all_same_values() {
        let mut sma = SimpleMovingAverage::new(3);
        assert_eq!(sma.next_chunk(&[2.0, 2.0, 2.0, 2.0]), 2.0);
    }

    #[test]
    fn test_sma_with_zeros() {
        let mut sma = SimpleMovingAverage::new(3);
        assert_eq!(sma.next_chunk(&[0.0, 0.0, 0.0, 0.0]), 0.0);
    }
}
