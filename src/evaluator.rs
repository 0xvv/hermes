use log::debug;
use phf::phf_ordered_map;

pub(crate) static RANKS: phf::OrderedMap<char, u32> = phf_ordered_map! {
    '2' => 2,
    '3' => 3,
    '4' => 5,
    '5' => 7,
    '6' => 11,
    '7' => 13,
    '8' => 17,
    '9' => 19,
    'T' => 23,
    'J' => 29,
    'Q' => 31,
    'K' => 37,
    'A' => 41,
};

pub(crate) static SUITS: phf::OrderedMap<char, u32> = phf_ordered_map! {
    'c' => 1 << 27,
    'd' => 1 << 28,
    'h' => 1 << 29,
    's' => 1 << 30,
};

#[derive(Debug)]
pub struct Hand {
    has_pair: bool,
    val: u32,
    rank: u32,
}

pub struct Evaluator {
    flushes: fnv::FnvHashMap<u32, u32>,
    non_flushes: fnv::FnvHashMap<u32, u32>,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        let mut e = Evaluator {
            flushes: fnv::FnvHashMap::default(),
            non_flushes: fnv::FnvHashMap::default(),
        };
        e.init();
        e
    }

    /// Checks if the hand only contains one suit
    pub fn is_flush(handkey: u32) -> bool {
        let c = handkey & SUITS[&'c'] > 0;
        let d = handkey & SUITS[&'d'] > 0;
        let h = handkey & SUITS[&'h'] > 0;
        let s = handkey & SUITS[&'s'] > 0;

        !c && !d && (h ^ s) || !h && !s && (c ^ d)
    }

    /// Get the rank of the hand, lower is better
    pub fn get_hand_rank(&self, handkey: u32) -> u32 {
        if Evaluator::is_flush(handkey) {
            self.flushes[&(handkey & 0x07FFFFFF)]
        } else {
            self.non_flushes[&(handkey & 0x07FFFFFF)]
        }
    }

    fn init(&mut self) {
        let ranks: Vec<char> = RANKS.keys().rev().cloned().collect();
        debug!("Ranks: {:?}\n\n\n", ranks);

        let mut straights: Vec<[&char; 5]> = vec![];
        for i in 0..9 {
            let straight = &ranks[i..i + 5];
            straights.push([
                &straight[0],
                &straight[1],
                &straight[2],
                &straight[3],
                &straight[4],
            ]);
        }
        straights.push([&'5', &'4', &'3', &'2', &'A']);

        debug!("Straights: {:?}\n\n\n", straights);

        let ranks_asc: Vec<&char> = RANKS.keys().collect();

        let non_pairs: Vec<[&char; 5]> = make_sets(ranks_asc, 5);
        let mut filtered_non_pairs: Vec<[&char; 5]> = vec![];

        for set in &non_pairs {
            if !straights
                .iter()
                .map(|cards| cards.iter().collect::<Vec<_>>())
                .any(|cards| cards == set.iter().collect::<Vec<_>>())
            {
                filtered_non_pairs.push(*set);
            }
        }
        filtered_non_pairs.reverse();

        debug!("Filtered non pairs: {:?}\n\n\n", filtered_non_pairs.len());

        let mut quads: Vec<[&char; 5]> = vec![];
        for quad_rank in &ranks {
            for kicker_rank in ranks
                .iter()
                .filter(|&x| x != quad_rank)
                .collect::<Vec<&char>>()
            {
                quads.push([quad_rank, quad_rank, quad_rank, quad_rank, kicker_rank]);
            }
        }

        let mut fulls: Vec<[&char; 5]> = vec![];
        for trips_rank in &ranks {
            for pair_rank in ranks
                .iter()
                .filter(|&x| x != trips_rank)
                .collect::<Vec<&char>>()
            {
                fulls.push([trips_rank, trips_rank, trips_rank, pair_rank, pair_rank]);
            }
        }

        debug!("Fulls: {:?}\n\n\n", fulls);

        let mut trips: Vec<[&char; 5]> = vec![];
        for trips_rank in &ranks {
            for kicker1_rank in ranks
                .iter()
                .filter(|&x| x != trips_rank)
                .collect::<Vec<&char>>()
            {
                for kicker2_rank in ranks
                    .iter()
                    .filter(|&x| x != trips_rank && x != kicker1_rank)
                    .collect::<Vec<&char>>()
                {
                    trips.push([
                        trips_rank,
                        trips_rank,
                        trips_rank,
                        kicker1_rank,
                        kicker2_rank,
                    ]);
                }
            }
        }

        debug!("Trips: {:?}\n\n\n", trips);

        let mut two_pairs: Vec<[&char; 5]> = vec![];
        for pair1_rank in &ranks {
            for pair2_rank in ranks
                .iter()
                .filter(|&x| x != pair1_rank)
                .collect::<Vec<&char>>()
            {
                for kicker_rank in ranks
                    .iter()
                    .filter(|&x| x != pair1_rank && x != pair2_rank)
                    .collect::<Vec<&char>>()
                {
                    two_pairs.push([pair1_rank, pair1_rank, pair2_rank, pair2_rank, kicker_rank]);
                }
            }
        }

        debug!("Two pairs: {:?}\n\n\n", two_pairs);

        let mut pairs: Vec<[&char; 5]> = vec![];
        for pair_rank in &ranks {
            for kicker1_rank in ranks
                .iter()
                .filter(|&x| x != pair_rank)
                .collect::<Vec<&char>>()
            {
                for kicker2_rank in ranks
                    .iter()
                    .filter(|&x| x != pair_rank && x != kicker1_rank)
                    .collect::<Vec<&char>>()
                {
                    for kicker3_rank in ranks
                        .iter()
                        .filter(|&x| x != pair_rank && x != kicker1_rank && x != kicker2_rank)
                        .collect::<Vec<&char>>()
                    {
                        pairs.push([
                            pair_rank,
                            pair_rank,
                            kicker1_rank,
                            kicker2_rank,
                            kicker3_rank,
                        ]);
                    }
                }
            }
        }

        debug!("Pairs {:?}\n\n\n", pairs);

        let all_hands = [
            straights.clone(),
            quads,
            fulls,
            filtered_non_pairs.clone(),
            straights,
            trips,
            two_pairs,
            pairs,
            filtered_non_pairs,
        ]
        .concat();

        debug!("All: {:?}\n\n\n", all_hands);

        let mut evaluated_hands: Vec<Hand> = vec![];
        let mut i = 1;
        for hand in all_hands {
            evaluated_hands.push(Hand {
                has_pair: contains_pair(hand),
                val: get_val(hand),
                rank: i,
            });
            i += 1;
        }

        debug!("Evaluated hands: {:?}\n\n\n", evaluated_hands);

        for (i, hand) in evaluated_hands.iter().enumerate() {
            if i > 1598 || hand.has_pair {
                self.non_flushes.insert(hand.val, hand.rank);
            } else {
                self.flushes.insert(hand.val, hand.rank);
            }
        }

        debug!("Final flushes map: {:?}\n\n\n", self.flushes);
        debug!("Final non flushes map: {:?}\n\n\n", self.non_flushes);
    }
}

pub(crate) fn get_val(hand: [&char; 5]) -> u32 {
    let mut val = 1;
    for rank in &hand[0..5] {
        val *= RANKS[rank];
    }
    val
}

fn contains_pair(hand: [&char; 5]) -> bool {
    let mut scanned = vec![];
    for &s in hand {
        if scanned.contains(&s) {
            return true;
        }
        scanned.push(s);
    }
    false
}

fn make_sets(ranks: Vec<&'static char>, size: usize) -> Vec<[&'static char; 5]> {
    let mut results: Vec<[&'static char; 5]> = vec![];
    let mut mask = size;
    let total = 2u32.pow(ranks.len() as u32);
    while (mask as u32) < total {
        let mut result = vec![];
        let mut i = ranks.len();
        loop {
            if i == 0 {
                break;
            }
            i -= 1;

            if mask & (1 << i) != 0 {
                result.push(&ranks[i])
            }
        }
        if result.len() == size {
            results.push([result[0], result[1], result[2], result[3], result[4]])
        }

        mask += 1;
    }
    results
}

#[cfg(test)]
mod tests {
    use crate::evaluator::{contains_pair, get_val, RANKS, SUITS};
    use crate::Evaluator;

    #[test]
    fn is_flush_test() {
        assert_eq!(false, Evaluator::is_flush(0b1010 << 27));
        assert_eq!(false, Evaluator::is_flush(0b0101 << 27));
        assert_eq!(false, Evaluator::is_flush(0b1111 << 27));
        assert_eq!(false, Evaluator::is_flush(0b1011 << 27));
        assert_eq!(false, Evaluator::is_flush(0b1101 << 27));
        assert_eq!(false, Evaluator::is_flush(0b1001 << 27));
        assert_eq!(false, Evaluator::is_flush(0b0110 << 27));

        assert!(Evaluator::is_flush(0b0001 << 27));
        assert!(Evaluator::is_flush(SUITS[&'c']));
        assert!(Evaluator::is_flush(0b0010 << 27));
        assert!(Evaluator::is_flush(SUITS[&'d']));
        assert!(Evaluator::is_flush(0b0100 << 27));
        assert!(Evaluator::is_flush(SUITS[&'s']));
        assert!(Evaluator::is_flush(0b1000 << 27));
        assert!(Evaluator::is_flush(SUITS[&'h']));

        assert!(Evaluator::is_flush(
            SUITS[&'c'] | get_val([&'9', &'K', &'Q', &'J', &'T'])
        ));
        assert_eq!(
            false,
            Evaluator::is_flush(0b1111 << 27 | (41_u32.pow(4) * 37))
        );
        assert!(Evaluator::is_flush(0b1000 << 27 | (41_u32.pow(4) * 37)));
        assert!(Evaluator::is_flush(0b0001 << 27 | (41_u32.pow(4) * 37)));
    }

    #[test]
    fn get_val_test() {
        assert_eq!(104_553_157, get_val([&'A', &'A', &'A', &'A', &'K']));
        assert_eq!(31_367_009, get_val([&'A', &'K', &'Q', &'J', &'T']));
        assert_eq!(630, get_val([&'2', &'3', &'3', &'4', &'5']));
        assert_eq!(457_653, get_val([&'5', &'Q', &'3', &'K', &'9']));
        assert_eq!(14_535_931, get_val([&'9', &'K', &'Q', &'J', &'T']));
    }

    #[test]
    fn contains_pair_test() {
        assert!(contains_pair([&'A', &'A', &'A', &'A', &'K']));
        assert!(contains_pair([&'2', &'3', &'3', &'4', &'5']));
        assert_eq!(false, contains_pair([&'5', &'Q', &'3', &'K', &'9']));
    }

    #[test]
    fn get_rank_test() {
        let e = Evaluator::new();
        assert_eq!(
            11,
            e.get_hand_rank(0b1111 << 27 | (RANKS[&'A'].pow(4) * RANKS[&'K']))
        ); // 4 aces and a K
        assert_eq!(
            1,
            e.get_hand_rank(0b1000 << 27 | get_val([&'A', &'K', &'Q', &'J', &'T']))
        ); // Best Hand
    }

    #[test]
    fn flush_agnostic_test() {
        let e = Evaluator::new();
        assert_eq!(
            e.get_hand_rank(0b0100 << 27 | get_val([&'A', &'K', &'Q', &'J', &'T'])),
            e.get_hand_rank(0b1000 << 27 | get_val([&'A', &'K', &'Q', &'J', &'T']))
        );
        assert_eq!(
            e.get_hand_rank(0b0010 << 27 | get_val([&'A', &'K', &'Q', &'J', &'T'])),
            e.get_hand_rank(0b0001 << 27 | get_val([&'A', &'K', &'Q', &'J', &'T']))
        );
    }

    #[test]
    fn inter_hands_compare_test() {
        let e = Evaluator::new();
        let rank_royal_flush =
            e.get_hand_rank(SUITS[&'h'] | get_val([&'A', &'K', &'Q', &'J', &'T'])); // 1
        let rank_straight_flush =
            e.get_hand_rank(SUITS[&'c'] | get_val([&'9', &'K', &'Q', &'J', &'T'])); // 2
        let rank_quads_aces_king =
            e.get_hand_rank(0b1111 << 27 | get_val([&'A', &'A', &'A', &'A', &'K'])); // 11
        let ranks_quads_kings_ace =
            e.get_hand_rank(0b1111 << 27 | get_val([&'K', &'K', &'K', &'K', &'A'])); // 12
        let rank_full_ak = e.get_hand_rank(0b1111 << 27 | get_val([&'A', &'A', &'A', &'K', &'K']));
        let rank_flush = e.get_hand_rank(SUITS[&'c'] | get_val([&'A', &'Q', &'T', &'2', &'7']));
        let rank_straight = e.get_hand_rank(0b1011 << 27 | get_val([&'A', &'K', &'Q', &'J', &'T']));
        let rank_high_ace = e.get_hand_rank(0b0111 << 27 | get_val([&'A', &'3', &'4', &'J', &'T']));
        let rank_trips = e.get_hand_rank(0b0111 << 27 | get_val([&'A', &'A', &'A', &'J', &'T']));
        let rank_dub_pairs =
            e.get_hand_rank(0b1011 << 27 | get_val([&'A', &'A', &'J', &'J', &'T']));
        let rank_pairs = e.get_hand_rank(0b0111 << 27 | get_val([&'A', &'A', &'4', &'J', &'T']));

        assert!(
            rank_royal_flush < rank_straight_flush,
            "{rank_royal_flush} > {rank_straight_flush}"
        );
        assert!(
            rank_straight_flush < rank_quads_aces_king,
            "{rank_straight_flush} > {rank_quads_aces_king}"
        );
        assert!(
            rank_quads_aces_king < ranks_quads_kings_ace,
            "{rank_quads_aces_king} > {ranks_quads_kings_ace}"
        );
        assert!(
            ranks_quads_kings_ace < rank_full_ak,
            "{ranks_quads_kings_ace} > {rank_full_ak}"
        );
        assert!(rank_full_ak < rank_flush, "{rank_full_ak} > {rank_flush}");
        assert!(rank_flush < rank_straight, "{rank_flush} > {rank_straight}");
        assert!(rank_straight < rank_trips, "{rank_straight} > {rank_trips}");
        assert!(
            rank_trips < rank_dub_pairs,
            "{rank_trips} > {rank_dub_pairs}"
        );

        assert!(
            rank_dub_pairs < rank_pairs,
            "{rank_dub_pairs} > {rank_pairs}"
        );

        assert!(rank_pairs < rank_high_ace, "{rank_pairs} > {rank_high_ace}");
    }
}
