#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::too_many_lines)]

use std::cmp::Ordering;

// fn with match might be more convenient
// #[rustfmt::skip]
// const CARDS: [(char, u32); 13] = [
//                // 0
//     ('2', 2),  // 1
//     ('3', 3),  // 2
//     ('4', 5),  // 3
//     ('5', 7),  // 4     lowest straight, 10 possibilities
//     ('6', 11), // 5
//     ('7', 13), // 6
//     ('8', 17), // 7
//     ('9', 19), // 8
//     ('T', 23), // 9
//     ('J', 29), // 10
//     ('Q', 31), // 11
//     ('K', 37), // 12
//     ('A', 41), // 13
// ];

// , 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97
const VALUES: [u32; 14] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43];

fn card_value_from_rank(card_rank: char, ace_value: u32) -> u32 {
    match (card_rank, ace_value) {
        ('A', 2) => 2,
        ('2', _) => 3,
        ('3', _) => 5,
        ('4', _) => 7,
        ('5', _) => 11,
        ('6', _) => 13,
        ('7', _) => 17,
        ('8', _) => 19,
        ('9', _) => 23,
        ('T', _) => 29,
        ('J', _) => 31,
        ('Q', _) => 37,
        ('K', _) => 41,
        ('A', 43) => 43,
        _ => panic!(),
    }
}
// const HIGH_cards_values: [u32; 2] = [43 * 3 * 5 * 7 * 11, 41 * 37 * 31 * 29 * 23];
fn highest_card_value(n: u32) -> u32 {
    if n % 3 == 0 {
        return 0;
    }
    if n % 5 == 0 {
        return 1;
    }
    if n % 7 == 0 {
        return 2;
    }
    if n % 11 == 0 {
        return 3;
    }
    if n % 13 == 0 {
        return 4;
    }
    if n % 17 == 0 {
        return 5;
    }
    if n % 19 == 0 {
        return 6;
    }
    if n % 29 == 0 {
        return 7;
    }
    if n % 31 == 0 {
        return 8;
    }
    if n % 37 == 0 {
        return 9;
    }
    if n % 41 == 0 {
        return 10;
    }
    if n % 43 == 0 {
        return 11;
    }
    panic!();
}

const STRAIGHT_VALUES: [u32; 10] = [
    11 * 7 * 5 * 3 * 43,    // 5 4 3 2 A TODO: double check if this is correct
    13 * 11 * 7 * 5 * 3,    // 6 5 4 3 2
    17 * 13 * 11 * 7 * 5,   // 7 6 5 4 3
    19 * 17 * 13 * 11 * 7,  // 8 7 6 5 4
    23 * 19 * 17 * 13 * 11, // 9 8 7 6 5
    29 * 23 * 19 * 17 * 13, // T 9 8 7 6
    31 * 29 * 23 * 19 * 17, // J T 9 8 7
    37 * 31 * 29 * 23 * 19, // Q J T 9 8
    41 * 37 * 31 * 29 * 23, // K Q J T 9
    43 * 41 * 37 * 31 * 29, // A K Q J T
];

const QUAD_VALUES: [u32; 13] = [
    3 * 3 * 3 * 3,
    5 * 5 * 5 * 5,
    7 * 7 * 7 * 7,
    11 * 11 * 11 * 11,
    13 * 13 * 13 * 13,
    17 * 17 * 17 * 17,
    19 * 19 * 19 * 19,
    23 * 23 * 23 * 23,
    29 * 29 * 29 * 29,
    31 * 31 * 31 * 31,
    37 * 37 * 37 * 37,
    41 * 41 * 41 * 41,
    43 * 43 * 43 * 43,
];

const TRIP_VALUES: [u32; 13] = [
    3 * 3 * 3,
    5 * 5 * 5,
    7 * 7 * 7,
    11 * 11 * 11,
    13 * 13 * 13,
    17 * 17 * 17,
    19 * 19 * 19,
    23 * 23 * 23,
    29 * 29 * 29,
    31 * 31 * 31,
    37 * 37 * 37,
    41 * 41 * 41,
    43 * 43 * 43,
];

const ONE_PAIR_VALUES: [u32; 13] = [
    3 * 3,
    5 * 5,
    7 * 7,
    11 * 11,
    13 * 13,
    17 * 17,
    19 * 19,
    23 * 23,
    29 * 29,
    31 * 31,
    37 * 37,
    41 * 41,
    43 * 43,
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRankingCategory {
    HighCard,
    OnePair,
    TwoPairs,
    Trip,
    Straight,
    Flush,
    FullHouse,
    Quad,
    StraightFlush,
}

#[derive(Debug)]
struct Hand<'a> {
    str: &'a str,
    category: HandRankingCategory,
    product: u32,
    position: usize,
    another_position: Option<usize>,
    // cards_values: Vec<u32>,
}

impl<'a> From<&'a str> for Hand<'a> {
    fn from(str: &'a str) -> Self {
        let mut is_flush = true;

        let cards = str.split(' ').map(str::as_bytes).collect::<Vec<_>>();

        let first_card_suit = cards.first().unwrap().last().unwrap();

        let cards_values = cards
            .iter()
            .map(|&card| {
                if first_card_suit != card.last().unwrap() {
                    is_flush = false;
                }

                let card_rank = if card.len() == 3 {
                    'T'
                } else {
                    *card.first().unwrap() as char
                };

                card_value_from_rank(card_rank, 43) // useless?
            })
            .collect::<Vec<u32>>();

        let hand_value = cards_values.iter().product();

        // straight flush
        let straight_value = STRAIGHT_VALUES.iter().position(|&v| v == hand_value);
        dbg!(straight_value, STRAIGHT_VALUES);
        if is_flush {
            if let Some(highest_card_value) = straight_value {
                return Hand {
                    str,
                    category: HandRankingCategory::StraightFlush,
                    // cards_values: vec![highest_card_value.try_into().unwrap()],
                    product: hand_value,
                    position: highest_card_value,
                    another_position: None,
                };
            }
        }

        // quad
        let quad_value = QUAD_VALUES.iter().find(|&v| hand_value % v == 0);
        if let Some(highest_card_value) = quad_value {
            return Hand {
                str,
                category: HandRankingCategory::Quad,
                product: hand_value,
                position: *highest_card_value as usize,
                another_position: None,
            };
        }
        // FullHouse,
        // we can simply check for position for this ones
        // let trip_value = TRIP_VALUES.iter().find(|&v| hand_value % v == 0);
        let trip_pos = TRIP_VALUES.iter().position(|&v| hand_value % v == 0);
        if let Some(trip_pos) = trip_pos {
            // dbg!(highest_card_value / 3);
            let pair_value = ONE_PAIR_VALUES
                .iter()
                .enumerate()
                .find(|&t| t.0 != trip_pos && hand_value % t.1 == 0);

            if let Some(ap) = pair_value {
                return Hand {
                    str,
                    category: HandRankingCategory::FullHouse,
                    product: hand_value, // TODO: this might be an option
                    position: trip_pos,
                    another_position: Some((*ap.1).try_into().unwrap()),
                };
            }
        }

        // Trip,
        if let Some(highest_card_value) = trip_pos {
            return Hand {
                str,
                category: HandRankingCategory::Trip,
                product: hand_value,
                // position: *highest_card_value as usize,
                position: highest_card_value as usize,
                another_position: None,
            };
        }

        // Straight,
        if let Some(highest_card_value) = straight_value {
            return Hand {
                str,
                category: HandRankingCategory::Straight,
                product: hand_value,
                position: highest_card_value,
                another_position: None,
            };
        }

        // Flush,
        if is_flush {
            return Hand {
                str,
                category: HandRankingCategory::Flush,
                // cards_values: vec![highest_card_value.try_into().unwrap()],
                product: hand_value,
                // TODO: change position to Option type
                position: 0,
                another_position: None,
            };
        }

        // two pairs
        // this can be done by calling one pair twice
        // just save the biggest in position
        // then use one pair value for dividing
        // and then check remaining card
        let one_pair_value = ONE_PAIR_VALUES.iter().find(|&v| hand_value % v == 0);
        if let Some(first_pair_highest_value) = one_pair_value {
            // check also that you don't find the same pair again
            let other_pair_value = ONE_PAIR_VALUES
                .iter()
                .find(|&v| v != first_pair_highest_value && hand_value % v == 0);
            if let Some(second_pair_highest_value) = other_pair_value {
                // if their are equal? cannot we already checked for quad
                if second_pair_highest_value > first_pair_highest_value {
                    return Hand {
                        str,
                        category: HandRankingCategory::TwoPairs,
                        product: hand_value,
                        position: *second_pair_highest_value as usize, // highest
                        another_position: Some(*first_pair_highest_value as usize), // lowest
                                                                       // this will simplify the sort
                    };
                }
                return Hand {
                    str,
                    category: HandRankingCategory::TwoPairs,
                    product: hand_value,
                    position: *first_pair_highest_value as usize, // highest
                    another_position: Some(*second_pair_highest_value as usize), // lowest
                                                                  // this will simplify the sort
                };
            }

            // if let Some(highest_card_value) = one_pair_value {
            return Hand {
                str,
                category: HandRankingCategory::OnePair,
                product: hand_value,
                position: *first_pair_highest_value as usize,
                another_position: None,
            };
            // }
        } // end of first pair scope

        // one pair
        // let one_pair_value = ONE_PAIR_VALUES.iter().find(|&v| hand_value % v == 0);
        // if let Some(highest_card_value) = one_pair_value {
        //     return Hand {
        //         str,
        //         category: HandRankingCategory::OnePair,
        //         product: hand_value,
        //         position: *highest_card_value as usize,
        //         another_position: None,
        //     };
        // }

        // high hand
        Hand {
            str,
            category: HandRankingCategory::HighCard,
            product: hand_value,
            position: highest_card_value(hand_value) as usize,
            another_position: None,
        }
    }
}

// do we really need this?
impl<'a> PartialEq<Hand<'a>> for Hand<'a> {
    fn eq(&self, other: &Hand<'_>) -> bool {
        self.partial_cmp(other).unwrap_or(Ordering::Equal) == Ordering::Equal
    }
}

fn find_highest_by_product(l: u32, r: u32) -> Option<Ordering> {
    let mut ordering = Some(Ordering::Equal);

    for value in VALUES {
        match (l % value == 0, r % value == 0) {
            (true, false) => {
                ordering = Some(Ordering::Less);
                break;
            }
            (false, true) => {
                ordering = Some(Ordering::Greater);
                break;
            }
            _ => continue,
        };
    }

    ordering
}

impl<'a> PartialOrd<Hand<'a>> for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.product == other.product {
            return Some(Ordering::Equal);
        }

        if self.category == other.category {
            match self.category {
                HandRankingCategory::StraightFlush => {
                    return Some(self.position.cmp(&other.position));
                }
                HandRankingCategory::Quad => {
                    if self.position == other.position {
                        return find_highest_by_product(self.product, other.product);
                    }
                    return Some(self.position.cmp(&other.position));
                }
                HandRankingCategory::Trip => {
                    // panic!()
                    return find_highest_by_product(self.product, other.product);
                }
                HandRankingCategory::TwoPairs => {
                    // position is higher another_position is lower
                    if self.position == other.position {
                        if self.another_position == other.another_position {
                            return find_highest_by_product(self.product, other.product);
                        }

                        return Some(self.another_position.cmp(&other.another_position));
                    }
                    return Some(self.position.cmp(&other.position));
                }
                HandRankingCategory::FullHouse => {
                    // position is higher another_position is lower
                    if self.position == other.position {
                        if self.another_position == other.another_position {
                            return Some(Ordering::Equal);
                            // return find_highest_by_product(self.product, other.product);
                        }

                        return Some(self.another_position.cmp(&other.another_position));
                    }
                    return Some(self.position.cmp(&other.position));
                }
                HandRankingCategory::OnePair => {
                    if self.position == other.position {
                        return find_highest_by_product(self.product, other.product);
                    }
                    return Some(self.position.cmp(&other.position));
                }
                HandRankingCategory::HighCard => {
                    return find_highest_by_product(self.product, other.product);
                }
                HandRankingCategory::Straight => {
                    return Some(self.position.cmp(&other.position));
                    // return find_highest_by_product(self.product, other.product);
                }
                HandRankingCategory::Flush => {
                    return find_highest_by_product(self.product, other.product);
                }
            }
        }

        Some(self.category.cmp(&other.category))
    }
}

#[must_use]
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands: Vec<Hand> = hands.iter().map(|&hand| hand.into()).collect();

    hands.sort_by(|l, r| l.partial_cmp(r).unwrap());

    dbg!(&hands);

    hands
        .iter()
        .rev()
        .take_while(|&hand| hands.last().unwrap() == hand)
        .map(|hand| hand.str)
        .collect::<Vec<_>>()
}
