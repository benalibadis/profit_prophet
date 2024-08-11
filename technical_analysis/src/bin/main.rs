use technical_analysis::indicators::SimpleMovingAverage;
use technical_analysis::indicators::Indicator;

fn main() {
    let mut bb = SimpleMovingAverage::new(2);
    let data = vec![0.00000001, 0.00000002, 0.00000003, 0.00000004];

    println!("{:?}", bb.next_chunk(&data));
    
}
