use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::{str::{Lines, FromStr}, ops};

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

#[derive(Clone, PartialEq, Debug)]
struct CategoryRange {
    start: i64,
    length: i64,
}

impl CategoryRange {
    fn new(start: i64, length: i64) -> Self {
        Self { start, length }
    }

    fn new_end(start: i64, end: i64) -> Self {
        Self { start, length: end - start }
    }

    fn end(&self) -> i64 {
        self.start + self.length
    }

    fn is_empty(&self) -> bool {
        self.length <= 0
    }

    fn non_empty(self) -> Option<Self> {
        Some(self).filter(|r| !r.is_empty())
    }

    fn contains(&self, value: i64) -> bool {
        value >= self.start && value < self.end()
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        let start = self.start.max(other.start);
        let end = (self.end()).min(other.end());
        if start < end {
            let length = end - start;
            Some(Self{ start, length })
        } else {
            None
        }
    }
}

impl ops::Add<i64> for CategoryRange {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        Self{ start: self.start + rhs, length: self.length }
    }
}

#[derive(PartialEq, Debug)]
struct CategoryMapEntry {
    source_range: CategoryRange,
    dest_range_start: i64,
}

impl CategoryMapEntry {
 
    fn dest_range(&self) -> CategoryRange {
        CategoryRange { start: self.dest_range_start, length: self.source_range.length }
    }

    fn map(&self, value: i64) -> i64 {
        value - self.source_range.start + self.dest_range_start
    }

    fn map_range(&self, range: &CategoryRange) -> Option<Self> {
        self.source_range
            .intersect(range)
            .map(|source_range| {
                let dest_range_start = self.map(source_range.start);
                Self { source_range, dest_range_start }
            })
    }
}

impl FromStr for CategoryMapEntry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, dest_range_start, source_range_start, range_length) = regex_captures!(r"(\d+)\s+(\d+)\s+(\d+)", s).ok_or(ParseError)?;
        let dest_range_start = dest_range_start.parse::<i64>().map_err(|_| ParseError)?;
        let source_range_start = source_range_start.parse::<i64>().map_err(|_| ParseError)?;
        let range_length = range_length.parse::<i64>().map_err(|_| ParseError)?;
        let source_range = CategoryRange { start: source_range_start, length: range_length };
        Ok(CategoryMapEntry { source_range, dest_range_start })
    }
}

struct CategoryMap {
    entries: Vec<CategoryMapEntry>,
}

impl CategoryMap {
    fn from_entries(mut entries: Vec<CategoryMapEntry>) -> Self {
        entries.sort_by_key(|e| e.source_range.start);
        Self {entries}
    }

    fn lookup(&self, value: i64) -> i64 {
        self.entries
            .iter()
            .find(|e| e.source_range.contains(value))
            .map(|e| e.map(value))
            .unwrap_or(value)
    }

    fn lookup_range(&self, range: &CategoryRange) -> Vec<CategoryRange> {
        let (end, mut output) = self.entries.iter()
            .filter_map(|e| e.map_range(range))
            .fold((range.start, Vec::<CategoryRange>::new()), |(start, mut output), e| {
                output.extend(CategoryRange::new_end(start, e.source_range.start).non_empty());
                output.push(e.dest_range());
                (e.source_range.end(), output)
            });
        output.extend(CategoryRange::new_end(end, range.end()).non_empty());
        output
    }

    fn lookup_ranges(&self, ranges: &[CategoryRange]) -> Vec<CategoryRange> {
        ranges.iter().flat_map(|r| self.lookup_range(r)).collect_vec()
    }
}

fn parse_seeds(seeds: &str) -> Vec<i64> {
    let seeds = seeds.split_once(':').unwrap().1;
    seeds.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect_vec()
}

fn parse_maps(lines: Lines) -> Vec<CategoryMap> {
    lines
        .group_by(|line| !line.is_empty())
        .into_iter()
        .filter_map(|(non_empty, lines)| {
            if non_empty {Some(lines.into_iter().dropping(1))} else {None}
        })
        .map(|lines| {
            CategoryMap::from_entries(lines.map(|line| line.parse::<CategoryMapEntry>().unwrap()).collect_vec())
        })
        .collect_vec()
}

fn parse_part1(mut lines: Lines) -> (Vec<i64>, Vec<CategoryMap>) {
    let seeds = parse_seeds(lines.next().unwrap());
    let maps = parse_maps(lines);
    (seeds, maps)
}

fn part1(input: Lines) -> String {
    let (seeds, maps) = parse_part1(input);
    seeds.into_iter().map(|s| {
        maps.iter().fold(s, |value, category_map| {
            category_map.lookup(value)
        })
    })
    .min()
    .unwrap()
    .to_string()
}

fn parse_seed_ranges(seeds: &str) -> Vec<CategoryRange> {
    let seeds = seeds.split_once(':').unwrap().1;
    seeds
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|mut c| CategoryRange::new(c.next().unwrap(), c.next().unwrap()))
        .collect_vec()
}

fn parse_part2(mut lines: Lines) -> (Vec<CategoryRange>, Vec<CategoryMap>) {
    let seeds = parse_seed_ranges(lines.next().unwrap());
    let maps = parse_maps(lines);
    (seeds, maps)
}

fn part2(input: Lines) -> String {
    let (seed_ranges, maps) = parse_part2(input);
    maps.iter().fold(seed_ranges, |ranges, category_map| {
        category_map.lookup_ranges(&ranges)
    })
    .into_iter()
    .map(|r| r.start)
    .min()
    .unwrap()
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
    fn test_parse_seed_ranges() {
        assert_eq!(
            parse_seed_ranges("seeds: 79 14 55 13"),
            vec![CategoryRange::new(79, 14), CategoryRange::new(55, 13)]
        )
    }

    #[test]
    fn test_intersect() {
        assert_eq!(
            CategoryRange::new(1, 5).intersect(&CategoryRange::new(2, 3)),
            Some(CategoryRange::new(2, 3))
        );
        assert_eq!(
            CategoryRange::new(2, 3).intersect(&CategoryRange::new(1, 5)),
            Some(CategoryRange::new(2, 3))
        );
        assert_eq!(
            CategoryRange::new(1, 3).intersect(&CategoryRange::new(2, 5)),
            Some(CategoryRange::new(2, 2))
        );
        assert_eq!(
            CategoryRange::new(2, 5).intersect(&CategoryRange::new(1, 3)),
            Some(CategoryRange::new(2, 2))
        );
        assert_eq!(
            CategoryRange::new(1, 2).intersect(&CategoryRange::new(3, 4)),
            None
        );
    }

    #[test]
    fn test_lookup_ranges() {
        let map = CategoryMap{
            entries: vec![
                CategoryMapEntry{ source_range: CategoryRange::new(2, 2), dest_range_start: 10},
                CategoryMapEntry{ source_range: CategoryRange::new(6, 4), dest_range_start: 20},
            ]
        };
        assert_eq!(
            map.lookup_ranges(&[CategoryRange::new(2, 2), CategoryRange::new(4, 2), CategoryRange::new(6, 4)]),
            &[CategoryRange::new(10, 2), CategoryRange::new(4, 2), CategoryRange::new(20, 4)]
        );
        assert_eq!(
            map.lookup_ranges(&[CategoryRange::new(1, 10)]),
            vec![
                CategoryRange::new(1, 1),
                CategoryRange::new(10, 2),
                CategoryRange::new(4, 2),
                CategoryRange::new(20, 4),
                CategoryRange::new(10, 1),
            ]
        );
        assert_eq!(
            map.lookup_ranges(&[CategoryRange::new(7, 2)]),
            &[CategoryRange::new(21, 2)]
        );
        assert_eq!(
            map.lookup_ranges(&[CategoryRange::new(7, 5)]),
            &[CategoryRange::new(21, 3), CategoryRange::new(10, 2)]
        );
        let adjacent_map = CategoryMap{
            entries: vec![
                CategoryMapEntry{ source_range: CategoryRange::new(2, 3), dest_range_start: 10},
                CategoryMapEntry{ source_range: CategoryRange::new(5, 4), dest_range_start: 20},
            ]
        };
        assert_eq!(
            adjacent_map.lookup_ranges(&[CategoryRange::new(3, 5)]),
            &[CategoryRange::new(11, 2), CategoryRange::new(20, 3)]
        );
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "35");
        verify!(part2, input, "46");
    }
}
