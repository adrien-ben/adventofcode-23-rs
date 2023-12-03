use std::collections::HashSet;

use adventofcode_23_rs::util;

fn main() {
    let result = Puzzle::parse().solve();

    println!("{result}"); // 78826761
}

#[derive(Debug)]
struct Puzzle {
    parts: Vec<u32>,
    width: usize,
    height: usize,
    schema: Vec<Vec<Cell>>,
}

impl Puzzle {
    fn parse() -> Self {
        let mut parts = vec![];
        let mut current_part = None;
        let mut schema = vec![];

        for l in util::lines("inputs/day3.txt") {
            let mut cells = vec![];
            for c in l.chars() {
                let cell: Cell = (c, parts.len()).into();
                cells.push(cell);
                if let Cell::Part(v, _) = cell {
                    let p = current_part.get_or_insert(0);
                    *p = *p * 10 + v;
                } else if let Some(p) = current_part.take() {
                    parts.push(p);
                }
            }
            schema.push(cells);
        }

        Puzzle {
            width: schema[0].len(),
            height: schema.len(),
            schema,
            parts,
        }
    }

    fn solve(&self) -> u32 {
        let mut result = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.schema[y][x];
                if cell.is_gear() {
                    let adjacent_parts = self.adjacent_parts(x, y);
                    if adjacent_parts.len() == 2 {
                        result += adjacent_parts
                            .iter()
                            .map(|index| self.parts[*index])
                            .product::<u32>();
                    }
                }
            }
        }

        result
    }

    fn adjacent_parts(&self, x: usize, y: usize) -> HashSet<usize> {
        let mut indices = HashSet::new();

        let min_x = if x > 0 { x - 1 } else { x + 1 };
        let max_x = if x < self.width - 1 { x + 1 } else { x - 1 };
        let min_y = if y > 0 { y - 1 } else { y + 1 };
        let max_y = if y < self.height - 1 { y + 1 } else { y - 1 };

        for nx in min_x..=max_x {
            for ny in min_y..=max_y {
                if let Cell::Part(_, index) = self.schema[ny][nx] {
                    indices.insert(index);
                }
            }
        }

        indices
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Part(u32, usize),
    Gear,
    Symbol,
    Empty,
}

impl Cell {
    fn is_gear(self) -> bool {
        self == Self::Gear
    }
}

impl From<(char, usize)> for Cell {
    fn from((value, index): (char, usize)) -> Self {
        if let Some(d) = value.to_digit(10) {
            Cell::Part(d, index)
        } else if value == '.' {
            Cell::Empty
        } else if value == '*' {
            Cell::Gear
        } else {
            Cell::Symbol
        }
    }
}
