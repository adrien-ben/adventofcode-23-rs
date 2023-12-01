use core::fmt;
use std::str::FromStr;

use adventofcode_23_rs::util;

fn main() {
    let result = util::lines("inputs/day1.txt")
        .map(|l| {
            let line = Line(&l);
            let first = line.into_iter().next().unwrap_or_default().0;
            let last = line.into_iter().last().unwrap_or_default().0;
            10 * first + last
        })
        .sum::<u32>();

    println!("{result}"); // 54094
}

#[derive(Debug, Clone, Copy)]
struct Line<'a>(&'a str);

impl<'a> IntoIterator for Line<'a> {
    type Item = Digit;

    type IntoIter = DigitIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DigitIter::new(self.0)
    }
}

struct DigitIter<'a> {
    start: usize,
    end: usize,
    len: usize,
    line: &'a str,
}

impl<'a> DigitIter<'a> {
    fn new(line: &'a str) -> Self {
        Self {
            start: 0,
            end: 1,
            len: line.len(),
            line,
        }
    }
}

impl<'a> Iterator for DigitIter<'a> {
    type Item = Digit;

    fn next(&mut self) -> Option<Self::Item> {
        while self.start < self.len {
            match self.line[self.start..self.end].parse() {
                Ok(d) => {
                    self.start += 1;
                    self.end = self.start + 1;
                    return Some(d);
                }
                Err(ParseDigitError::Incomplete) => {
                    self.end += 1;
                }
                Err(ParseDigitError::Incorrect) => {
                    self.start += 1;
                    self.end = self.start + 1
                }
            }

            if self.end > self.len {
                self.start += 1;
                self.end = self.start + 1;
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Digit(u32);

#[derive(Debug)]
enum ParseDigitError {
    Incomplete,
    Incorrect,
}

impl fmt::Display for ParseDigitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseDigitError::Incomplete => write!(f, "Digit is not complete"),
            ParseDigitError::Incorrect => write!(f, "Not a digit"),
        }
    }
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

impl FromStr for Digit {
    type Err = ParseDigitError;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        if val.len() == 1 {
            if let Some(d) = val.chars().nth(0).unwrap().to_digit(10) {
                return Ok(Digit(d));
            }
        }

        if let Some(pos) = DIGITS.iter().position(|d| d == &val) {
            return Ok(Digit(pos as u32 + 1));
        }

        if DIGITS.iter().any(|d| d.starts_with(val)) {
            return Err(ParseDigitError::Incomplete);
        }

        Err(ParseDigitError::Incorrect)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Digit, DigitIter, ParseDigitError};

    #[test]
    fn parse_digit_from_digit() {
        let result = "5".parse::<Digit>();
        assert!(matches!(result, Ok(Digit(5))))
    }

    #[test]
    fn parse_digit_from_valid_string() {
        let result = "five".parse::<Digit>();
        assert!(matches!(result, Ok(Digit(5))))
    }

    #[test]
    fn parse_digit_from_incomplete_string() {
        let result = "fiv".parse::<Digit>();
        assert!(matches!(result, Err(ParseDigitError::Incomplete)))
    }

    #[test]
    fn parse_digit_from_number() {
        let result = "51".parse::<Digit>();
        assert!(matches!(result, Err(ParseDigitError::Incorrect)))
    }

    #[test]
    fn parse_digit_from_incorrect_string() {
        let result = "fives".parse::<Digit>();
        assert!(matches!(result, Err(ParseDigitError::Incorrect)))
    }

    #[test]
    fn parse_all_digits() {
        let line = "onehtzmgbpkjcninefive7twonebmlnvfhsreightthree";
        let mut iter = DigitIter::new(line);

        assert_eq!(Some(Digit(1)), iter.next());
        assert_eq!(Some(Digit(9)), iter.next());
        assert_eq!(Some(Digit(5)), iter.next());
        assert_eq!(Some(Digit(7)), iter.next());
        assert_eq!(Some(Digit(2)), iter.next());
        assert_eq!(Some(Digit(1)), iter.next());
        assert_eq!(Some(Digit(8)), iter.next());
        assert_eq!(Some(Digit(3)), iter.next());
        assert_eq!(None, iter.next());
    }
}
