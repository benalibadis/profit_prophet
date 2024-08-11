#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, RateOfChange};

    #[test]
    fn test_roc_next() {
        let mut roc = RateOfChange::new(3);
        assert_eq!(roc.next(1.0), 0.0);
        assert_eq!(roc.next(2.0), 100.0);
        assert_eq!(roc.next(3.0), 50.0);
        assert_eq!(33.33, roc.next(4.0));
    }

    #[test]
    fn test_roc_next_chunk() {
        let mut roc = RateOfChange::new(3);
        let result = roc.next_chunk(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(33.33, result);
    }

    #[test]
    fn test_roc_reset() {
        let mut roc = RateOfChange::new(3);
        roc.next(1.0);
        roc.next(2.0);
        roc.reset();
        assert_eq!(roc.next(3.0), 0.0);
    }

    #[test]
    fn test_roc_with_large_data() {
        let mut roc = RateOfChange::new(100);
        let data: Vec<f64> = (1..=1000).map(|x| x as f64).collect();
        let result = roc.next_chunk(&data);
        assert!(result > 0.0 && result < 100.0); // ROC should be between 0% and 100% depending on data
    }

    #[test]
    fn test_roc_all_same_values() {
        let mut roc = RateOfChange::new(3);
        let result = roc.next_chunk(&[2.0, 2.0, 2.0, 2.0]);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_roc_with_zeros() {
        let mut roc = RateOfChange::new(3);
        let result = roc.next_chunk(&[0.0, 0.0, 0.0, 0.0]);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_roc_with_very_small_values() {
        let mut roc = RateOfChange::new(3);
        let result = roc.next_chunk(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(33.33, result);
    }
}
