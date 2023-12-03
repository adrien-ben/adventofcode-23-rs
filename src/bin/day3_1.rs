use adventofcode_23_rs::util;

fn main() {
    let result = Puzzle::parse().solve();

    println!("{result}"); // 533784
}

#[derive(Debug)]
struct Puzzle {
    parts: Vec<Part>,
    width: usize,
    height: usize,
    schema: Vec<Vec<Cell>>,
}

impl Puzzle {
    fn parse() -> Self {
        let mut parts = vec![];
        let mut current_part = None;
        let mut schema = vec![];

        for (y, l) in util::lines("inputs/day3.txt").enumerate() {
            let mut cells = vec![];
            for (x, c) in l.chars().enumerate() {
                let cell: Cell = c.into();
                cells.push(cell);
                if let Cell::Part(v) = cell {
                    let p = current_part.get_or_insert(Part::from_start(x, y));
                    p.value = p.value * 10 + v;
                    p.len += 1;
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
        self.parts
            .iter()
            .filter_map(|p| self.is_valid_part(p).then_some(p.value))
            .sum()
    }

    fn is_valid_part(&self, part: &Part) -> bool {
        (0..part.len)
            .map(|i| (part.x + i, part.y))
            .any(|(x, y)| self.touches_symbol(x, y))
    }

    fn touches_symbol(&self, x: usize, y: usize) -> bool {
        let min_x = if x > 0 { x - 1 } else { x + 1 };
        let max_x = if x < self.width - 1 { x + 1 } else { x - 1 };
        let min_y = if y > 0 { y - 1 } else { y + 1 };
        let max_y = if y < self.height - 1 { y + 1 } else { y - 1 };

        for nx in min_x..=max_x {
            for ny in min_y..=max_y {
                if self.schema[ny][nx].is_symbol() {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    value: u32,
    x: usize,
    y: usize,
    len: usize,
}

impl Part {
    fn from_start(x: usize, y: usize) -> Self {
        Self {
            value: 0,
            x,
            y,
            len: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Part(u32),
    Symbol,
    Empty,
}

impl Cell {
    fn is_symbol(self) -> bool {
        self == Self::Symbol
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        if let Some(d) = value.to_digit(10) {
            Cell::Part(d)
        } else if value == '.' {
            Cell::Empty
        } else {
            Cell::Symbol
        }
    }
}
