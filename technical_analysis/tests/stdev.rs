#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, StandardDeviation};

    #[test]
    fn test_stdev_next() {
        let mut stdev = StandardDeviation::new(3);
        assert_eq!(stdev.next(1.0), 0.0);
        assert_eq!(0.5, stdev.next(2.0));
        assert_eq!(0.82, stdev.next(3.0));
        assert_eq!(1.0, stdev.next(4.0));
    }

    #[test]
    fn test_stdev_next_chunk() {
        let mut stdev = StandardDeviation::new(3);
        let result = stdev.next_chunk(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(1.0, result);
    }

    #[test]
    fn test_stdev_reset() {
        let mut stdev = StandardDeviation::new(3);
        stdev.next(1.0);
        stdev.next(2.0);
        stdev.reset();
        assert_eq!(stdev.next(3.0), 0.0);
    }

    #[test]
    fn test_stdev_with_large_data() {
        let mut stdev = StandardDeviation::new(100);
        let data: Vec<f64> = (1..=1000).map(|x| x as f64).collect();
        let result = stdev.next_chunk(&data);
        assert!(result > 28.5 && result < 29.5); // Standard deviation around 28.87 for first 100 numbers
    }

    #[test]
    fn test_stdev_all_same_values() {
        let mut stdev = StandardDeviation::new(3);
        let result = stdev.next_chunk(&[2.0, 2.0, 2.0, 2.0]);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_stdev_with_zeros() {
        let mut stdev = StandardDeviation::new(3);
        let result = stdev.next_chunk(&[0.0, 0.0, 0.0, 0.0]);
        assert_eq!(result, 0.0);
    }

}
