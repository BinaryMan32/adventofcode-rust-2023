use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::str::{FromStr, Lines};

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

#[derive(Default, Debug, PartialEq, Eq)]
struct Card {
    id: u32,
    winning: Vec<u8>,
    numbers: Vec<u8>,
}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, id, winning, numbers) = regex_captures!(r"Card\s+([0-9]+):\s*(.*)\s*\|\s*(.*)\s*$", s).ok_or(ParseError)?;
        let id = id.parse::<u32>().map_err(|_| ParseError)?;
        let winning = winning.split_whitespace().filter_map(|n| n.parse::<u8>().ok()).collect_vec();
        let numbers = numbers.split_whitespace().filter_map(|n| n.parse::<u8>().ok()).collect_vec();
        Ok(Card{id, winning, numbers})
    }
}

impl Card {
    fn num_winning(&self) -> usize {
        self.winning.iter().filter(|n| self.numbers.contains(n)).count()
    }

    fn worth(&self) -> u32 {
        let matches = self.num_winning() as u32;
        if matches > 0 { 2u32.pow(matches - 1) } else { 0 }
    }
}

fn part1(input: Lines) -> String {
    input.into_iter()
        .map(|line| line.parse::<Card>().unwrap())
        .map(|card| card.worth())
        .sum::<u32>()
        .to_string()
}

fn part2(input: Lines) -> String {
    let matching = input.into_iter()
        .map(|line| line.parse::<Card>().unwrap())
        .map(|card| card.num_winning())
        .collect_vec();
    let mut cards = vec![1usize; matching.len()];
    for (i, matches) in matching.into_iter().enumerate() {
        for j in 1..=matches {
            cards[i+j] += cards[i];
        }
    }
    cards.into_iter().sum::<usize>().to_string()
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
    fn test_from_str() {
        assert_eq!(
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".parse::<Card>(),
            Ok(Card{
                id: 3,
                winning: vec!(1, 21, 53, 59, 44),
                numbers: vec!(69, 82, 63, 72, 16, 21, 14, 1),
            })
        );
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "13");
        verify!(part2, input, "30");
    }
}
