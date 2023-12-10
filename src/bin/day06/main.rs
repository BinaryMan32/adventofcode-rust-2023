use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::str::Lines;

fn parse_numbers(line: &str) -> Vec<u64> {
    let numbers = line.split_once(':').unwrap().1;
    numbers.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect_vec()
}

/**
 * The boat charges for hold_time, and then travels at velocity hold_time for
 * the remaining time (time - hold_time). To see if a run beats the record,
 * solve the inequality:
 * (time - hold_time) * hold_time >= record_distance + 1
 * 
 * Note record_distance + 1 since we must beat the record.
 * 
 * Rearrange:
 * -1 * hold_time^2 + time * hold_time - (record_distance + 1) > 0
 *
 * using the quadratic formula
 * (time +- sqrt(time^2 - 4*(record_distance+1))) / 2
 */
fn count_ways_to_win(time: u64, record_distance: u64) -> usize {
    let ftime = time as f64;
    let discriminant = (ftime * ftime) - 4.0 * (record_distance + 1) as f64;
    let sqrt_discriminant = discriminant.sqrt();
    let hold_time_min = ((ftime - sqrt_discriminant) / 2.0).ceil() as u64;
    let hold_time_max = ((ftime + sqrt_discriminant) / 2.0).floor() as u64;
    (hold_time_max - hold_time_min + 1) as usize
}

fn part1(mut input: Lines) -> String {
    let times = parse_numbers(input.next().expect("missing first line"));
    let distances = parse_numbers(input.next().expect("missing second line"));
    times.into_iter()
        .zip(distances)
        .map(|(t, d)| count_ways_to_win(t, d))
        .product::<usize>()
        .to_string()
}

fn parse_kerned_numbers(line: &str) -> u64 {
    let numbers = line.split_once(':').unwrap().1;
    numbers.replace(' ', "").parse::<u64>().unwrap()
}

fn part2(mut input: Lines) -> String {
    let time = parse_kerned_numbers(input.next().expect("missing first line"));
    let distance = parse_kerned_numbers(input.next().expect("missing second line"));
    count_ways_to_win(time, distance).to_string()
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
    fn test_count_ways_to_win() {
        assert_eq!(count_ways_to_win(7, 9), 4);
        assert_eq!(count_ways_to_win(15, 40), 8);
        assert_eq!(count_ways_to_win(30, 200), 9);
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "288");
        verify!(part2, input, "71503");
    }
}
