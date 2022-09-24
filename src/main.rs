mod evaluator;

use crate::evaluator::Evaluator;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let e = Evaluator::new();

    println!("{:32b}", 0b1111 << 27);
    println!("{:32b}", 0b1111 << 27 | 41_u32.pow(4) * 37);
    println!("{:?}", Evaluator::is_flush(0b1010 << 27));
    println!("{:?}", Evaluator::is_flush(0b1111 << 27));
    println!("{:?}", Evaluator::is_flush(0b0001 << 27));

    let end = Instant::now();
    println!("{:?}", end.checked_duration_since(start));
}
