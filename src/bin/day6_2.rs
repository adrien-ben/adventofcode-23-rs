use adventofcode_23_rs::util;

fn main() {
    let result = solve(parse());

    println!("{result}"); // 24655068
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse() -> Race {
    let mut lines = util::lines("inputs/day6.txt");

    let times = lines.next().unwrap();
    let (_, times) = times.split_once(':').unwrap();
    let time = times.trim().replace(' ', "").parse::<u64>().unwrap();

    let distances = lines.next().unwrap();
    let (_, distances) = distances.split_once(':').unwrap();
    let distance = distances.trim().replace(' ', "").parse::<u64>().unwrap();

    Race { time, distance }
}

fn solve(race: Race) -> u64 {
    let mut victories = 0;
    for t_hold in 1..race.time {
        let t_move = race.time - t_hold;
        let speed = t_hold;
        let distance = speed * t_move;

        if distance > race.distance {
            victories += 1;
        }
    }
    victories
}
