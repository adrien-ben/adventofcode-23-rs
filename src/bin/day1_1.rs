use adventofcode_23_rs::util;

fn main() {
    let result = util::lines("inputs/day1.txt")
        .map(|l| parse_line(&l))
        .sum::<u32>();

    println!("{result}"); // 54968
}

fn parse_line(line: &str) -> u32 {
    let mut first = None;
    let mut last = 0;
    for c in line.chars() {
        if let Some(d) = c.to_digit(10) {
            if first.is_none() {
                first = Some(d)
            }
            last = d
        }
    }

    10 * first.unwrap_or(0) + last
}
