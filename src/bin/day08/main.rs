use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use num::integer::lcm;
use std::str::Lines;
use lazy_regex::regex_captures;

fn parse_step(step: char) -> bool {
    match step {
        'L' => false,
        'R' => true,
        _ => panic!("unexpected step char {}", step)
    }
}

fn parse_steps(line: &str) -> Vec<bool> {
    line.chars().map(parse_step).collect_vec()
}

const ID_CHARS: usize = 36;

fn parse_node_char(c: char) -> u16 {
    c.to_digit(ID_CHARS as u32).expect("a-z") as u16
}

fn parse_node_id(node_id: &str) -> u16 {
    node_id.chars().fold(0u16, |out, c| {
        out * ID_CHARS as u16 + parse_node_char(c)
    })
}

fn node_ends_with(node: u16, c: u16) -> bool {
    node % ID_CHARS as u16 == c
}

fn format_node_id(node: u16) -> String {
    let digit2 = node % ID_CHARS as u16;
    let node = node / ID_CHARS as u16;
    let digit1 = node % ID_CHARS as u16;
    let node = node / ID_CHARS as u16;
    let digit0 = node % ID_CHARS as u16;
    [digit0, digit1, digit2]
        .map(|d| std::char::from_digit(d as u32, ID_CHARS as u32).unwrap())
        .map(|c| c.to_ascii_uppercase())
        .into_iter()
        .collect::<String>()
    
}

const NUM_IDS: usize = ID_CHARS * ID_CHARS * ID_CHARS;

struct Network {
    nodes: [Option<(u16, u16)>; NUM_IDS]
}

impl Network {
    fn parse_node(line: &str) -> (u16, (u16, u16)) {
        let (_, node, left, right) = regex_captures!(r"([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)", line).expect("match");
        let node = parse_node_id(node);
        let left = parse_node_id(left);
        let right = parse_node_id(right);
        (node, (left, right))
    }

    fn new(input: Lines) -> Self {
        let mut nodes: [Option<(u16, u16)>; NUM_IDS] = [None; NUM_IDS];
        for (node, dest) in input.map(Self::parse_node) {
            nodes[node as usize] = Some(dest);
        }
        Self{ nodes }
    }

    fn start_nodes(&self, end: char) -> Vec<u16> {
        let end = parse_node_char(end);
        self.nodes.iter().enumerate().filter_map(|(i, n)| {
            if n.is_some() && node_ends_with(i as u16, end) {
                Some(i as u16)
            } else {
                None
            }
        }).collect_vec()
    }

    fn choices(&self, node: u16) -> (u16, u16) {
        if let Some(choices) = self.nodes[node as usize] {
            choices
        } else {
            panic!("node {} does not exist", node)
        }
    }

    fn next(&self, node: u16, right: bool) -> u16 {
        let choices = self.choices(node);
        match right {
            false => choices.0,
            true => choices.1,
        }
    }

    fn steps_to_end<'a, I, F>(&self, start_node: u16, steps: &mut I, is_end: F) -> (usize, u16)
    where
        I: Iterator<Item = &'a bool>,
        F: Fn(u16) -> bool,
    {
        let mut node = start_node;
        let mut num_steps = 0usize;
        while !is_end(node) {
            node = self.next(node, *steps.next().unwrap());
            num_steps += 1;
        }
        (num_steps, node)
    }
}

fn part1(mut input: Lines) -> String {
    let steps = parse_steps(input.next().expect("first line"));
    let network = Network::new(input.dropping(1));
    let start_node = parse_node_id("AAA");
    let end_node = parse_node_id("ZZZ");
    let is_end = |n| n == end_node;
    network.steps_to_end(start_node, &mut steps.iter().cycle(), is_end).0.to_string()
}

fn part2(mut input: Lines) -> String {
    let steps = parse_steps(input.next().expect("first line"));
    let network = Network::new(input.dropping(1));
    let start_nodes = network.start_nodes('A');
    let end_node_digit = parse_node_char('Z');
    let is_end = |n| node_ends_with(n, end_node_digit);

    start_nodes.into_iter().map(|start_node| {
        let mut node_steps = steps.iter().cycle();
        // find first node ending in Z
        let (end_steps, end_node ) = network.steps_to_end(start_node, &mut node_steps, is_end);
        // continue until reaching another node ending in Z
        let cycle_start_node = network.next(end_node, *node_steps.next().unwrap());
        let (cycle_steps, cycle_node) = network.steps_to_end(cycle_start_node, &mut node_steps, is_end);
        // logging shows some surprising things:
        // 1. end_node == cycle_node
        // 2. end_steps == cycle_steps + 1
        println!("start_node={}({}) end_steps={} end_node={}({}) cycle_steps={} cycle_node={}({})",
            start_node, format_node_id(start_node),
            end_steps, end_node, format_node_id(end_node),
            cycle_steps, cycle_node, format_node_id(cycle_node)
        );
        end_steps
    }).reduce(lcm).unwrap().to_string()
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
        let input2 = include_str!("example2.txt");
        verify!(part1, input, "2");
        verify!(part1, input2, "6");
        let input3 = include_str!("example3.txt");
        verify!(part2, input3, "6");
    }
}
