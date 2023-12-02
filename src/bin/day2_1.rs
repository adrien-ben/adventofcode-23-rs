use std::str::FromStr;

use adventofcode_23_rs::util;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let result: u32 = util::lines("inputs/day2.txt")
        .map(|l| l.parse::<Game>().unwrap())
        .filter_map(|g| g.is_possible().then_some(g.id))
        .sum();

    println!("{result}"); // 3099
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.sets.iter().all(|s| s.is_possible())
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn is_possible(&self) -> bool {
        self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((game, sets)) = s.split_once(": ") {
            let (_, id) = game.split_at(5);
            let id: u32 = id.parse().unwrap();

            let sets = sets
                .split("; ")
                .map(|set_str| {
                    let mut set = Set::default();

                    set_str.split(", ").for_each(|b| {
                        if let Some((count, color)) = b.split_once(' ') {
                            let count: u32 = count.parse().unwrap();
                            if color == "red" {
                                set.red = count
                            } else if color == "green" {
                                set.green = count;
                            } else {
                                set.blue = count
                            }
                        }
                    });

                    set
                })
                .collect::<Vec<_>>();

            return Ok(Game { id, sets });
        }
        Err(())
    }
}
