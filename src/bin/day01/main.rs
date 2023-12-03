use advent_of_code::{create_runner, named, Named, Runner};
use lazy_regex::{Regex, Lazy, lazy_regex};
use std::str::Lines;

fn part1(input: Lines) -> String {
    input
        .map(|line| {
            let first = line.chars().find(|&c| char::is_numeric(c))
                .and_then(|c| c.to_digit(10));
            let last: Option<_> = line.chars().rfind(|&c| char::is_numeric(c))
                .and_then(|c| c.to_digit(10));
            first.zip(last).map(|(a, b)| a * 10 + b).unwrap_or_default()
        })
        .sum::<u32>()
        .to_string()
}

fn parse_num(num: &str) -> Option<u32> {
    match num {
        "1" | "one" | "eno" => Some(1),
        "2" | "two" | "owt" => Some(2),
        "3" | "three" | "eerht" => Some(3),
        "4" | "four" | "ruof" => Some(4),
        "5" | "five" | "evif" => Some(5),
        "6" | "six" | "xis" => Some(6),
        "7" | "seven" | "neves" => Some(7),
        "8" | "eight" | "thgie" => Some(8),
        "9" | "nine" | "enin" => Some(9),
        _ => None
    }
}

pub static DIGIT_REGEX: Lazy<Regex> = lazy_regex!("[1-9]|one|two|three|four|five|six|seven|eight|nine");
pub static DIGIT_REGEX_REV: Lazy<Regex> = lazy_regex!("[1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin");

fn first_last_digit_words(line: &str) -> Option<(u32, u32)> {
    let first = DIGIT_REGEX.find(line).and_then(|m| parse_num(m.as_str()));
    let reversed: String = line.chars().rev().collect();
    let last = DIGIT_REGEX_REV.find(&reversed).and_then(|m| parse_num(m.as_str()));
    first.zip(last)
}

fn part2(input: Lines) -> String {
    input
        .map(|line| {
            first_last_digit_words(line).map(|(a, b)| a * 10 + b).unwrap_or_default()
        })
        .sum::<u32>()
        .to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let runner: &Runner = create_runner!();
    runner.run(named!(part1), input);
    runner.run(named!(part2), input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::verify;

    #[test]
    fn test_first_last_digit_words() {
        assert_eq!(first_last_digit_words("abc"), None);
        assert_eq!(first_last_digit_words("one"), Some((1, 1)));
        assert_eq!(first_last_digit_words("oneight"), Some((1, 8)));
        assert_eq!(first_last_digit_words("twone"), Some((2, 1)));
        assert_eq!(first_last_digit_words("threeight"), Some((3, 8)));
        assert_eq!(first_last_digit_words("fiveight"), Some((5, 8)));
        assert_eq!(first_last_digit_words("sevenine"), Some((7, 9)));
        assert_eq!(first_last_digit_words("eightwo"), Some((8, 2)));
        assert_eq!(first_last_digit_words("eighthree"), Some((8, 3)));
        assert_eq!(first_last_digit_words("nineight"), Some((9, 8)));
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "142");
        let input2 = include_str!("example2.txt");
        verify!(part2, input2, "281");
    }
}
