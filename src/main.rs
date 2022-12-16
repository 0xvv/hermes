mod evaluator;

use crate::evaluator::{get_val, Evaluator, SUITS};
use std::time::Instant;

fn main() {
    env_logger::init();
    let start = Instant::now();

    let e = Evaluator::new();
    for _i in 0..1_000_000 {
        e.get_hand_rank(0b1111 << 27 | (41_u32.pow(4) * 37));
        e.get_hand_rank(SUITS[&'h'] | get_val([&'A', &'K', &'Q', &'J', &'T'])); // 1
        e.get_hand_rank(SUITS[&'c'] | get_val([&'9', &'K', &'Q', &'J', &'T'])); // 2
        e.get_hand_rank(0b1111 << 27 | get_val([&'A', &'A', &'A', &'A', &'K'])); // 11
        e.get_hand_rank(0b1111 << 27 | get_val([&'K', &'K', &'K', &'K', &'A'])); // 12
        e.get_hand_rank(0b1111 << 27 | get_val([&'A', &'A', &'A', &'K', &'K']));
        e.get_hand_rank(SUITS[&'c'] | get_val([&'A', &'Q', &'T', &'2', &'7']));
        e.get_hand_rank(0b1011 << 27 | get_val([&'A', &'K', &'Q', &'J', &'T']));
        e.get_hand_rank(0b0111 << 27 | get_val([&'A', &'3', &'4', &'J', &'T']));
        e.get_hand_rank(0b0111 << 27 | get_val([&'6', &'6', &'6', &'A', &'K']));
    }

    let end = Instant::now();
    println!("{:?}", end.checked_duration_since(start));
}
