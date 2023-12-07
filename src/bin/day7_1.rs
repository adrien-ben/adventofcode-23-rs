use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use adventofcode_23_rs::util;

fn main() {
    let mut hands = parse();
    hands.sort();
    let result: u32 = hands
        .into_iter()
        .enumerate()
        .map(|(i, c)| (i as u32 + 1) * c.bid)
        .sum();

    println!("{result}"); // 252052080
}

fn parse() -> Vec<Hand> {
    util::lines("inputs/day7.txt")
        .map(|l| l.parse().unwrap())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    fn typ(&self) -> HandType {
        let mut card_counts = HashMap::new();
        self.cards.iter().for_each(|c| {
            card_counts.entry(c).and_modify(|e| *e += 1).or_insert(1u32);
        });

        use HandType::*;
        match card_counts.len() {
            1 => Five,
            2 => {
                if card_counts.values().any(|c| *c == 4) {
                    Four
                } else {
                    Full
                }
            }
            3 => {
                if card_counts.values().any(|c| *c == 3) {
                    Three
                } else {
                    TwoPair
                }
            }
            4 => Pair,
            _ => High,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.typ().cmp(&other.typ()) {
            Ordering::Equal => {}
            ord => return ord,
        }

        for i in 0..5 {
            match self.cards[i].cmp(&other.cards[i]) {
                Ordering::Equal => {}
                ord => return ord,
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid) = s.split_once(' ').unwrap();

        let mut cards = [Card::A; 5];
        cards_str
            .char_indices()
            .for_each(|(i, c)| cards[i] = c.into());

        let bid = bid.parse().unwrap();

        Ok(Hand { cards, bid })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[rustfmt::skip]
enum HandType {
    High, Pair, TwoPair, Three, Full, Four, Five,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
enum Card {
    N(char), T, J, Q, K, A
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        use Card::*;
        match value {
            'T' => T,
            'J' => J,
            'Q' => Q,
            'K' => K,
            'A' => A,
            _ => N(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Card::{self, *},
        Hand, HandType,
    };

    #[test]
    fn test_hand_type_five() {
        let typ = hand_from_cards([Q, Q, Q, Q, Q]).typ();

        assert_eq!(HandType::Five, typ);
    }

    #[test]
    fn test_hand_type_four() {
        let typ = hand_from_cards([Q, Q, Q, Q, K]).typ();

        assert_eq!(HandType::Four, typ);
    }

    #[test]
    fn test_hand_type_full() {
        let typ = hand_from_cards([Q, Q, Q, K, K]).typ();

        assert_eq!(HandType::Full, typ);
    }

    #[test]
    fn test_hand_type_three() {
        let typ = hand_from_cards([Q, Q, Q, K, A]).typ();

        assert_eq!(HandType::Three, typ);
    }

    #[test]
    fn test_hand_type_twopair() {
        let typ = hand_from_cards([Q, Q, K, K, A]).typ();

        assert_eq!(HandType::TwoPair, typ);
    }

    #[test]
    fn test_hand_type_pair() {
        let typ = hand_from_cards([Q, Q, T, K, A]).typ();

        assert_eq!(HandType::Pair, typ);
    }

    #[test]
    fn test_hand_type_high() {
        let typ = hand_from_cards([J, Q, T, K, A]).typ();

        assert_eq!(HandType::High, typ);
    }

    #[test]
    fn test_compare_hands() {
        assert!(
            hand_from_cards([Q, Q, Q, Q, Q])
                > hand_from_cards([N('1'), N('2'), N('3'), N('4'), N('5')])
        );
    }

    #[test]
    fn test_compare_hands_five() {
        assert!(hand_from_cards([Q, Q, Q, Q, Q]) > hand_from_cards([J, J, J, J, J]));
    }

    #[test]
    fn test_compare_hands_four() {
        assert!(hand_from_cards([Q, Q, Q, Q, K]) > hand_from_cards([Q, Q, Q, Q, J]));
    }

    #[test]
    fn test_compare_hands_full() {
        assert!(hand_from_cards([Q, Q, Q, K, K]) > hand_from_cards([Q, Q, Q, J, J]));
    }

    #[test]
    fn test_compare_hands_three() {
        assert!(hand_from_cards([Q, Q, Q, K, A]) > hand_from_cards([Q, Q, Q, K, J]));
    }

    #[test]
    fn test_compare_hands_twopair() {
        assert!(hand_from_cards([Q, Q, K, K, A]) > hand_from_cards([Q, Q, K, K, J]));
    }

    #[test]
    fn test_compare_hands_pair() {
        assert!(hand_from_cards([Q, Q, K, J, A]) > hand_from_cards([Q, Q, K, J, N('1')]));
    }

    #[test]
    fn test_compare_hands_high() {
        assert!(hand_from_cards([J, Q, T, K, A]) > hand_from_cards([J, Q, T, K, N('1')]));
    }

    fn hand_from_cards(cards: [Card; 5]) -> Hand {
        Hand { cards, bid: 0 }
    }
}
