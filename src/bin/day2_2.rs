use std::str::FromStr;

use adventofcode_23_rs::util;

fn main() {
    let result: u32 = util::lines("inputs/day2.txt")
        .map(|l| l.parse::<Game>().unwrap())
        .map(|g| g.get_minimum_possible_set().power())
        .sum();

    println!("{result}"); // 72970
}

#[derive(Debug)]
#[allow(dead_code)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn get_minimum_possible_set(&self) -> Set {
        let mut min_set = Set::default();

        for s in &self.sets {
            if s.red > min_set.red {
                min_set.red = s.red;
            }
            if s.green > min_set.green {
                min_set.green = s.green;
            }
            if s.blue > min_set.blue {
                min_set.blue = s.blue;
            }
        }

        min_set
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
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
