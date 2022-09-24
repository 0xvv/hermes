use phf::phf_ordered_map;
use std::collections::HashMap;

static RANKS: phf::OrderedMap<char, u32> = phf_ordered_map! {
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

static SUITS: phf::OrderedMap<char, u32> = phf_ordered_map! {
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
    flushes: HashMap<u32, u32>,
    non_flushes: HashMap<u32, u32>,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        let mut e = Evaluator {
            flushes: HashMap::new(),
            non_flushes: HashMap::new(),
        };
        e.init();
        e
    }

    pub fn is_flush(handkey: u32) -> bool {
        (handkey & SUITS[&'c'] > 0)
            ^ (handkey & SUITS[&'d'] > 0)
            ^ (handkey & SUITS[&'h'] > 0)
            ^ (handkey & SUITS[&'s'] > 0)
    }

    pub fn get_hand_rank(&self, handkey: u32) -> u32 {
        !if Evaluator::is_flush(handkey) {
            self.flushes[&handkey]
        } else {
            self.non_flushes[&handkey]
        }
    }

    fn init(&mut self) {
        let ranks: Vec<char> = RANKS.keys().rev().cloned().collect();
        //println!('{:?}', ranks);

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

        //println!('{:?}', straights);
        let ranks_asc: Vec<&char> = RANKS.keys().collect();

        let non_pairs: Vec<[&char; 5]> = make_sets(ranks_asc, 5);

        let mut filtered_non_pairs = vec![];

        for set in &non_pairs {
            if !straights
                .iter()
                .map(|cards| cards.iter().collect::<Vec<_>>())
                .any(|cards| cards == set.iter().collect::<Vec<_>>())
            {
                filtered_non_pairs.push(set);
            }
        }
        filtered_non_pairs.reverse();

        //println!('{:?}', filtered_non_pairs.len());

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

        //println!('{:?}', fulls);

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

        //println!('{:?}', trips);

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

        //println!('{:?}', twoPairs);

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

        //println!('{:?}', pairs);

        let all_hands = [
            straights.clone(),
            quads,
            fulls,
            non_pairs.clone(),
            straights,
            trips,
            two_pairs,
            pairs,
            non_pairs,
        ]
        .concat();

        //println!('{:?}', all_hands);

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

        //println!('{:?}', evaluated_hands);

        for (i, hand) in evaluated_hands.iter().enumerate() {
            if i > 1598 || hand.has_pair {
                self.non_flushes.insert(hand.val, hand.rank);
            } else {
                self.flushes.insert(hand.val, hand.rank);
            }
        }
    }
}

fn get_val(hand: [&char; 5]) -> u32 {
    let mut val = RANKS[hand[0]];
    for rank in &hand[1..5] {
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

static RANKS: phf::OrderedMap<char, u32> = phf_ordered_map! {
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

static SUITS: phf::OrderedMap<char, u32> = phf_ordered_map! {
    'c' => 1 << 27,
    'd' => 1 << 28,
    'h' => 1 << 29,
    's' => 1 << 30,
};
