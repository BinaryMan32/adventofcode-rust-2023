use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::{str::{Lines, FromStr}, cmp::max};

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

#[derive(Debug, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(ParseError)
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Sample {
    colors: [u32; 3],
}

impl Sample {
    fn add(&self, count: u32, color: Color) -> Self {
        let mut colors = self.colors;
        colors[color as usize] += count;
        Self { colors }
    }

    fn is_possible(&self, cubes: &Sample) -> bool {
        self.colors.iter().zip(cubes.colors.iter()).all(|(s, c)| s <= c)
    }

    fn max_by_color(&self, other: &Sample) -> Sample {
        let colors = [
            max(self.colors[0], other.colors[0]),
            max(self.colors[1], other.colors[1]),
            max(self.colors[2], other.colors[2]),
        ];
        Sample{colors}
    }

    fn power(&self) -> u32 {
        self.colors.iter().product()
    }
}

impl FromStr for Sample {
    type Err = ParseError;

    fn from_str(sample: &str) -> Result<Self, Self::Err> {
        sample
            .split(',')
            .try_fold(Sample::default(), |result, s| {
                let (_, count, color) = regex_captures!(r"([0-9]+)\s+(red|green|blue)", s).ok_or(ParseError)?;
                let count = count.parse::<u32>().map_err(|_| ParseError)?;
                let color = color.parse::<Color>()?;
                Ok(result.add(count, color))
            })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    samples: Vec<Sample>,
}

impl Game {
    fn is_possible(&self, cubes: &Sample) -> bool {
        self.samples.iter().all(|s| s.is_possible(cubes))
    }

    fn cubes_reqiured(&self) -> Sample {
        self.samples.iter().fold(Sample::default(), |r, s| s.max_by_color(&r))
    }
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, id, rest) = regex_captures!(r"Game\s+([0-9]+):\s+(.*)$", s).ok_or(ParseError)?;
        let id = id.parse::<u32>().map_err(|_| ParseError)?;
        let samples = rest.split(';').filter_map(|s| s.parse::<Sample>().ok()).collect_vec();
        Ok(Game{id, samples})
    }
}

fn part1(input: Lines) -> String {
    let cubes = Sample{ colors: [12, 13, 14]};
    input
        .map(|line| line.parse::<Game>().unwrap())
        .filter(|g| g.is_possible(&cubes))
        .map(|g| g.id)
        .sum::<u32>()
        .to_string()
}

fn part2(input: Lines) -> String {
    input
        .map(|line| line.parse::<Game>().unwrap())
        .map(|g| g.cubes_reqiured().power())
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
    fn test_from_str() {
        assert_eq!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".parse::<Game>(),
            Ok(Game{
                id: 1,
                samples: vec![
                    Sample{colors: [4, 0, 3]},
                    Sample{colors: [1, 2, 6]},
                    Sample{colors: [0, 2, 0]}
                ]
            })
        )
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "8");
        verify!(part2, input, "2286");
    }
}
