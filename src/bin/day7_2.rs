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

    println!("{result}"); // 252898370
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
    fn typ(mut self) -> HandType {
        // Get the most frequent card in the hand a replace each occurence of J with it
        let mut card_counts = HashMap::new();
        self.cards.into_iter().for_each(|c| {
            card_counts.entry(c).and_modify(|e| *e += 1).or_insert(1u32);
        });

        let (most_frequent_card, _) = card_counts
            .into_iter()
            .filter(|(c, _)| c != &Card::J)
            .max_by(|(_, count_a), (_, count_b)| count_a.cmp(count_b))
            .unwrap_or((Card::A, 0));

        self.cards
            .iter_mut()
            .filter(|c| c == &&Card::J)
            .for_each(|c| *c = most_frequent_card);

        // Compute the type base on the new cards
        let mut card_counts = HashMap::new();
        self.cards.into_iter().for_each(|c| {
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
    J, N(char), T, Q, K, A
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
