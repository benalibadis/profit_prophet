#![feature(test)]

extern crate test;
use test::Bencher;
use technical_analysis::indicators::{Indicator, SimpleMovingAverage, StandardDeviation, RateOfChange, RelativeStrengthIndex, BollingerBands};
use technical_analysis::IndicatorValue;

// Function to generate 1000 pseudo-random numbers using a simple LCG
fn generate_random_data(seed: u64, len: usize) -> Vec<IndicatorValue> {
    let mut state = seed;
    let mut data = Vec::with_capacity(len);

    for _ in 0..len {
        // Simple LCG with default multiplier (6364136223846793005)
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let value = (state >> 32) as f64 / (1u64 << 32) as f64 * 100.0;
        data.push(IndicatorValue::from(value));
    }

    data
}

fn bench_indicator<T: Indicator<Input = IndicatorValue> + Default>(b: &mut Bencher) {
    let data = generate_random_data(12345, 1000);
    let mut iter = data.iter().cycle();
    let mut indicator = T::default();

    for _ in 0..50 {
        indicator.next(*iter.next().unwrap());
    }

    b.iter(|| indicator.next(*iter.next().unwrap()));
}

#[bench]
fn bench_sma_p14(b: &mut Bencher) {
    bench_indicator::<SimpleMovingAverage>(b);
}

#[bench]
fn bench_sma_p140(b: &mut Bencher) {
    let data = generate_random_data(12345, 1000);
    let mut iter = data.iter().cycle();
    let mut indicator = SimpleMovingAverage::new(140);

    for _ in 0..50 {
        indicator.next(*iter.next().unwrap());
    }

    b.iter(|| indicator.next(*iter.next().unwrap()));
}

#[bench]
fn bench_stdev_p14(b: &mut Bencher) {
    bench_indicator::<StandardDeviation>(b);
}

#[bench]
fn bench_stdev_p140(b: &mut Bencher) {
    let data = generate_random_data(12345, 1000);
    let mut iter = data.iter().cycle();
    let mut indicator = StandardDeviation::new(140);
    for _ in 0..50 {
        indicator.next(*iter.next().unwrap());
    }

    b.iter(|| indicator.next(*iter.next().unwrap()));
}

#[bench]
fn bench_stdev_simd_p14(b: &mut Bencher) {
    let data = generate_random_data(12345, 1000);
    let mut _iter = data.iter().cycle();
    let mut _indicator = StandardDeviation::new(14);

    b.iter(|| {
        _indicator.next_chunk(&[*_iter.next().unwrap()])
    });
}

#[bench]
fn bench_stdev_simd_p140(b: &mut Bencher) {
    let data = generate_random_data(12345, 1000);
    let mut _iter = data.iter().cycle();
    let mut _indicator = StandardDeviation::new(140);

    b.iter(|| {
        _indicator.next_chunk(&[*_iter.next().unwrap()])
    });
}

#[bench]
fn bench_roc(b: &mut Bencher) {
    bench_indicator::<RateOfChange>(b);
}

#[bench]
fn bench_rsi(b: &mut Bencher) {
    bench_indicator::<RelativeStrengthIndex>(b);
}

#[bench]
fn bench_bollinger_bands(b: &mut Bencher) {
    bench_indicator::<BollingerBands>(b);
}
