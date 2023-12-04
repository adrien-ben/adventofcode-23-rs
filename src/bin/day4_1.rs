use std::{collections::HashSet, str::FromStr};

use adventofcode_23_rs::util;

fn main() {
    let result: u32 = util::lines("inputs/day4.txt")
        .map(|l| l.parse::<Card>().unwrap())
        .map(|c| c.points())
        .sum();

    println!("{result}"); // 22674
}

#[derive(Debug)]
struct Card {
    wins: HashSet<u32>,
    yours: Vec<u32>,
}

impl Card {
    fn points(&self) -> u32 {
        let mut points = 0;
        for v in &self.yours {
            if self.wins.contains(v) {
                if points == 0 {
                    points = 1
                } else {
                    points *= 2;
                }
            }
        }
        points
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
