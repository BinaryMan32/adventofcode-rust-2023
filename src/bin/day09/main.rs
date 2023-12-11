use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::str::Lines;

fn parse_value_history(line: &str) -> Vec<i64> {
    line.split_whitespace().map(|x| x.parse::<i64>().expect("signed integer")).collect_vec()
}

fn compute_differences(values: &mut [i64]) {
    if values.iter().any(|&v| v != 0) {
        let last = values.len() - 1;
        for i in 0..last {
            values[i] = values[i+1] - values[i];
        }
        compute_differences(&mut values[0..last])    
    }
}

fn extrapolate(mut values: Vec<i64>) -> i64
{
    compute_differences(&mut values);
    values.into_iter().sum()
}

fn part1(input: Lines) -> String {
    input
        .map(parse_value_history)
        .map(extrapolate)
        .sum::<i64>()
        .to_string()
}

fn compute_differences_prev(values: &mut [i64]) {
    if values.iter().any(|&v| v != 0) {
        for i in (1..values.len()).rev() {
            values[i] -= values[i-1]
        }
        compute_differences_prev(&mut values[1..])    
    }
}

fn extrapolate_prev(mut values: Vec<i64>) -> i64
{
    compute_differences_prev(&mut values);
    values.into_iter().rev().fold(0, |acc, v| v - acc)
}

fn part2(input: Lines) -> String {
    input
        .map(parse_value_history)
        .map(extrapolate_prev)
        .sum::<i64>()
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
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "114");
        verify!(part2, input, "2");
    }
}
