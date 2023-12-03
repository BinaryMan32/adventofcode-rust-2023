use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::str::Lines;

struct Schematic {
    cells: Vec<Vec<char>>,
}

impl Schematic {
    fn parse(input: Lines) -> Self {
        let cells = input.map(|line| line.chars().collect_vec()).collect_vec();
        Schematic { cells }
    }

    fn part_numbers(&self) -> Vec<PartNumber> {
        self.cells.iter().enumerate().flat_map(|(row, row_chars)| {
            let mut part_numbers: Vec<PartNumber> = Vec::new();
            let mut part_number: Option<PartNumber> = None;
            for (col, c) in row_chars.iter().enumerate() {
                part_number = match (part_number, c.to_digit(10)) {
                    (None, Some(digit)) => Some(PartNumber {num: digit, row, col_begin: col, col_end: col}),
                    (None, None) => None,
                    (Some(pn), Some(digit)) =>
                        Some(PartNumber { num: pn.num * 10 + digit, row, col_begin: pn.col_begin, col_end: col }),
                    (Some(pn), None) => {
                        part_numbers.push(pn);
                        None
                    },
                }
            }
            part_numbers.extend(part_number);
            part_numbers
        }).collect_vec()
    }

    fn is_adjacent_to_symbol(&self, part_number: &PartNumber) -> bool {
        let row = part_number.row;
        let cb = part_number.col_begin;
        let cb = if cb > 0 {
            if self.is_symbol_at(row, cb - 1) {return true}
            cb - 1
        } else {
            cb
        };

        let ce = part_number.col_end + 1;
        let ce = if ce < self.cells[row].len() {
            if self.is_symbol_at(row, ce) {return true}
            ce + 1
        } else {
            ce
        };

        if row > 0 && self.cells[row-1][cb..ce].iter().any(|&c| Self::is_symbol(c)) {return true}

        if (row+1) < self.cells.len() && self.cells[row+1][cb..ce].iter().any(|&c| Self::is_symbol(c)) {return true}

        false
    }

    fn is_symbol(c: char) -> bool {
        c != '.' && !c.is_digit(10)
    }

    fn is_symbol_at(&self, row: usize, col: usize) -> bool {
        Self::is_symbol(self.cells[row][col])
    }

    fn find_gears(&self) -> Vec<(usize, usize)> {
        self.cells.iter().enumerate().flat_map(|(row, row_chars)| {
            row_chars.iter().enumerate().filter_map(|(col, &c)| {
                if c == '*' {Some((row, col))} else {None}
            }).collect_vec()
        }).collect_vec()
    }

    fn is_adjacent(&self, part_number: &PartNumber, symbol: (usize, usize)) -> bool {
        let (row, col) = symbol;
        row.abs_diff(part_number.row) <= 1 && (col + 1 >= part_number.col_begin) && (col <= part_number.col_end + 1)
    }
}

struct PartNumber {
    num: u32,
    row: usize,
    col_begin: usize,
    col_end: usize,
}

fn part1(input: Lines) -> String {
    let schematic = Schematic::parse(input);
    schematic.part_numbers()
        .into_iter()
        .filter(|pn| schematic.is_adjacent_to_symbol(pn))
        .map(|pn| pn.num)
        .sum::<u32>()
        .to_string()
}

fn part2(input: Lines) -> String {
    let schematic = Schematic::parse(input);
    let part_numbers = schematic.part_numbers();
    schematic.find_gears()
        .into_iter()
        .map(|gear_pos| {
            part_numbers.iter()
                .filter(|pn| schematic.is_adjacent(pn, gear_pos))
                .collect_vec()
        })
        .filter(|adjacent_part_numbers| adjacent_part_numbers.len() == 2)
        .map(|adjacent_part_numbers| adjacent_part_numbers.iter().map(|pn| pn.num).product::<u32>())
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
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "4361");
        verify!(part2, input, "467835");
    }
}
