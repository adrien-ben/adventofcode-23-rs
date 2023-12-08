use std::collections::{HashMap, HashSet};

use adventofcode_23_rs::util;

fn main() {
    let result = Day8::parse().solve();

    println!("{result}"); // 14616363770447
}

#[derive(Debug)]
struct Day8 {
    sequence: String,
    starting_nodes: HashSet<String>,
    map: HashMap<String, (String, String)>,
}

impl Day8 {
    fn parse() -> Day8 {
        let mut lines = util::lines("inputs/day8.txt");

        let sequence = lines.next().unwrap().to_owned();
        lines.next();

        let mut starting_nodes = HashSet::new();

        let map = lines
            .map(|l| {
                let (node_str, children) = l.split_once(" = ").unwrap();

                let node = node_str.to_owned();

                if node_str.ends_with('A') {
                    starting_nodes.insert(node_str.to_owned());
                }

                let (left_str, right_str) = children.split_once(", ").unwrap();

                let left = left_str.chars().skip(1).collect();

                let right = right_str.chars().take(3).collect();

                (node, (left, right))
            })
            .collect();

        Day8 {
            sequence,
            starting_nodes,
            map,
        }
    }

    fn solve(self) -> u64 {
        // for each starting node, get the number of iterations
        let mut iters = vec![];
        'outter: for next_node in &self.starting_nodes {
            let mut next_node = next_node.clone();
            for (iter, dir) in self.sequence.chars().cycle().enumerate() {
                let children = self.map.get(&next_node).unwrap();
                if dir == 'L' {
                    next_node = children.0.clone();
                } else {
                    next_node = children.1.clone();
                }

                if next_node.ends_with('Z') {
                    iters.push(iter as u64 + 1);
                    continue 'outter;
                }
            }
        }

        // compute the least common multiple of all iterations
        iters
            .into_iter()
            .reduce(|acc, i| {
                if acc == 0 {
                    i
                } else {
                    num::integer::lcm(i, acc)
                }
            })
            .unwrap()
    }
}
