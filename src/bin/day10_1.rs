use adventofcode_23_rs::util;

fn main() {
    let result = Map::parse().farthest_distance();

    println!("{result}"); // 6867
}

type Pos = (usize, usize);

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    start: Pos,
}

impl Map {
    fn parse() -> Self {
        let mut start = (0, 0);
        let grid = util::lines("inputs/day10.txt")
            .enumerate()
            .map(|(y, l)| {
                l.char_indices()
                    .map(|(x, c)| {
                        let tile = c.into();
                        if tile == Tile::Start {
                            start = (x, y);
                        }
                        tile
                    })
                    .collect()
            })
            .collect::<Vec<Vec<_>>>();

        Self {
            width: grid[0].len(),
            height: grid.len(),
            grid,
            start,
        }
    }

    fn farthest_distance(&self) -> u32 {
        let mut distance = 0;

        let mut previous_left = self.start;
        let mut previous_right = self.start;
        let (mut left, mut right) = self.next_tiles_from_start();

        loop {
            distance += 1;

            if left == right {
                return distance;
            }

            let new_left = self.next_tile(left, previous_left);
            previous_left = left;
            left = new_left;

            let new_right = self.next_tile(right, previous_right);
            previous_right = right;
            right = new_right;
        }
    }

    fn next_tiles_from_start(&self) -> (Pos, Pos) {
        let (x, y) = self.start;

        let mut first = None;

        if x > 0 {
            let left = self.grid[y][x - 1];
            if let Tile::H | Tile::NE | Tile::SE = left {
                first = Some((x - 1, y));
            }
        }

        if x < self.width - 1 {
            let right = self.grid[y][x + 1];
            if let Tile::H | Tile::NW | Tile::SW = right {
                if let Some(first) = first {
                    return (first, (x + 1, y));
                }
                first = Some((x + 1, y));
            }
        }

        if y > 0 {
            let top = self.grid[y - 1][x];
            if let Tile::V | Tile::SW | Tile::SE = top {
                if let Some(first) = first {
                    return (first, (x, y - 1));
                }
                first = Some((x, y - 1));
            }
        }

        if y < self.height - 1 {
            let bottom = self.grid[y + 1][x];
            if let Tile::V | Tile::NE | Tile::NW = bottom {
                if let Some(first) = first {
                    return (first, (x, y + 1));
                }
            }
        }

        panic!("Did not found two tiles")
    }

    fn next_tile(&self, current: Pos, previous: Pos) -> Pos {
        let (path0, path1) = match self.grid[current.1][current.0] {
            Tile::H => ((current.0 - 1, current.1), (current.0 + 1, current.1)),
            Tile::V => ((current.0, current.1 - 1), (current.0, current.1 + 1)),
            Tile::NE => ((current.0, current.1 - 1), (current.0 + 1, current.1)),
            Tile::NW => ((current.0, current.1 - 1), (current.0 - 1, current.1)),
            Tile::SE => ((current.0, current.1 + 1), (current.0 + 1, current.1)),
            Tile::SW => ((current.0, current.1 + 1), (current.0 - 1, current.1)),
            _ => panic!("you shouldn't be here !"),
        };

        if path0 == previous {
            path1
        } else {
            path0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
enum Tile {
    V, H, NE, NW, SW, SE, G, Start
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Tile::*;
        match value {
            '|' => V,
            '-' => H,
            'L' => NE,
            'J' => NW,
            '7' => SW,
            'F' => SE,
            'S' => Start,
            _ => G,
        }
    }
}
