use adventofcode_23_rs::util;

fn main() {
    let result: i32 = util::lines("inputs/day9.txt")
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(complete_sequence)
        .sum();

    println!("{result}"); // 1789635132
}

fn complete_sequence(sequence: Vec<i32>) -> i32 {
    let mut diffs = vec![sequence];

    while !diffs[0].iter().all(|v| v == &0) {
        let new_diffs = diffs[0]
            .windows(2)
            .filter_map(|ab| match ab {
                [a, b] => Some(b - a),
                _ => None,
            })
            .collect::<Vec<_>>();

        diffs.insert(0, new_diffs);
    }

    let mut last;
    let mut next = 0;
    for diff in diffs.iter().skip(1) {
        last = next;
        next = diff.last().unwrap() + last;
    }

    next
}
