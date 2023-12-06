use adventofcode_23_rs::util;

fn main() {
    let result = solve(&parse());

    println!("{result}"); // 3317888
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn parse() -> Vec<Race> {
    let mut lines = util::lines("inputs/day6.txt");

    let times = lines.next().unwrap();
    let (_, times) = times.split_once(':').unwrap();
    let times = times
        .trim()
        .split_ascii_whitespace()
        .map(|t| t.parse::<u32>().unwrap());

    let distances = lines.next().unwrap();
    let (_, distances) = distances.split_once(':').unwrap();
    let distances = distances
        .trim()
        .split_ascii_whitespace()
        .map(|t| t.parse::<u32>().unwrap());

    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn solve(races: &[Race]) -> u32 {
    races
        .iter()
        .map(|r| {
            let mut victories = 0;
            for t_hold in 1..r.time {
                let t_move = r.time - t_hold;
                let speed = t_hold;
                let distance = speed * t_move;

                if distance > r.distance {
                    victories += 1;
                }
            }
            victories
        })
        .product()
}
