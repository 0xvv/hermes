mod evaluator;

use crate::evaluator::Evaluator;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let e = Evaluator::new();
    e.get_hand_rank(0b1111 << 27 | (41_u32.pow(4) * 37));

    let end = Instant::now();
    println!("{:?}", end.checked_duration_since(start));
}
