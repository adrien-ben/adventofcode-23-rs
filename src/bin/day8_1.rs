use std::collections::HashMap;

use adventofcode_23_rs::util;

fn main() {
    let result = Day8::parse().solve();

    println!("{result}"); // 20221
}

#[derive(Debug)]
struct Day8 {
    sequence: String,
    map: HashMap<String, (String, String)>,
}

impl Day8 {
    fn parse() -> Day8 {
        let mut lines = util::lines("inputs/day8.txt");

        let sequence = lines.next().unwrap().to_owned();
        lines.next();

        let map = lines
            .map(|l| {
                let (node_str, children) = l.split_once(" = ").unwrap();

                let node = node_str.to_owned();

                let (left_str, right_str) = children.split_once(", ").unwrap();

                let left = left_str.chars().skip(1).collect();

                let right = right_str.chars().take(3).collect();

                (node, (left, right))
            })
            .collect();

        Day8 { sequence, map }
    }

    fn solve(self) -> u32 {
        let mut next_node = "AAA".to_owned();
        for (iter, dir) in self.sequence.chars().cycle().enumerate() {
            let children = self.map.get(&next_node).unwrap();
            if dir == 'L' {
                next_node = children.0.clone();
            } else {
                next_node = children.1.clone();
            }

            if next_node == "ZZZ" {
                return iter as u32 + 1;
            }
        }

        panic!("Couldn't reach the end!");
    }
}
