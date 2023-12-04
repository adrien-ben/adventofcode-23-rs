use std::{collections::HashSet, str::FromStr};

use adventofcode_23_rs::util;

fn main() {
    let cards = util::lines("inputs/day4.txt")
        .map(|l| l.parse::<Card>().unwrap())
        .collect::<Vec<_>>();

    let mut counts = (0..cards.len()).map(|_| 1u32).collect::<Vec<_>>();

    for (i, c) in cards.iter().enumerate() {
        let matches = c.matches();
        let count = counts[i];

        for im in 0..matches {
            let next_index = i + im as usize + 1;
            counts[next_index] += count;
        }
    }

    let result: u32 = counts.into_iter().sum();

    println!("{result}"); // 5747443
}

#[derive(Debug)]
struct Card {
    wins: HashSet<u32>,
    yours: Vec<u32>,
}

impl Card {
    fn matches(&self) -> u32 {
        let mut matches = 0;
        for v in &self.yours {
            if self.wins.contains(v) {
                matches += 1;
            }
        }
        matches
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((_, lists)) = s.split_once(": ") {
            if let Some((l0, l1)) = lists.split_once(" | ") {
                let wins = l0.split_whitespace().map(|v| v.parse().unwrap()).collect();
                let yours = l1.split_whitespace().map(|v| v.parse().unwrap()).collect();

                return Ok(Card { wins, yours });
            }
        }

        Err("Not valid".to_owned())
    }
}
