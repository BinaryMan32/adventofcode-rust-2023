use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::{str::Lines, ops::Add};

#[derive(PartialEq, Clone, Copy)]
struct Pos {
    row: isize,
    col: isize,
}

impl Pos {
    const fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self{ row: self.row + rhs.row, col: self.col + rhs.col }
    }
}

#[derive(Clone, Copy)]
struct Step {
    pos: Pos,
    from: Direction,
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    const ALL: [Direction; 4] = [Self::North, Self::South, Self::East, Self::West];

    fn clipped_step(self, pos: &Pos, size: &Pos) -> Option<Step> {
        match self {
            Self::North => if pos.row > 0 {Some(Pos::new(pos.row - 1, pos.col))} else {None},
            Self::South => if pos.row+1 < size.row {Some(Pos::new(pos.row + 1, pos.col))} else {None},
            Self::West => if pos.col > 0 {Some(Pos::new(pos.row, pos.col - 1))} else {None},
            Self::East => if pos.col+1 < size.col {Some(Pos::new(pos.row, pos.col + 1))} else {None},
        }.map(|pos| Step{ pos, from: self.flip() })
    }

    fn flip(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("no tile defined for character")
        }
    }

    fn connection_directions(self) -> &'static [Direction] {
        match self {
            Tile::NS => &[Direction::North, Direction::South],
            Tile::EW => &[Direction::East, Direction::West],
            Tile::NE => &[Direction::North, Direction::East],
            Tile::NW => &[Direction::North, Direction::West],
            Tile::SW => &[Direction::South, Direction::West],
            Tile::SE => &[Direction::South, Direction::East],
            _ => &[]
        }
    }

    fn connects(self, dir: Direction) -> bool {
        self.connection_directions().iter().any(|&d| d == dir)
    }

    fn other(self, dir: Direction) -> Direction {
        *self.connection_directions().iter().find(|&&d| d != dir).unwrap()
    }
}

struct Field {
    tiles: Vec<Vec<Tile>>,
    size: Pos,
}

impl Field {
    fn from_lines(input: Lines) -> Self {
        let tiles = input.map(|line| line.chars().map(Tile::from_char).collect_vec()).collect_vec();
        let size = Pos::new(tiles.len() as isize, tiles[0].len() as isize);
        Self { tiles, size }
    }

    fn start_pos(&self) -> Pos {
        self.tiles.iter().enumerate().find_map(|(row, tiles)| {
            tiles.iter().find_position(|&&t| t == Tile::Start).map(|(col, _)| Pos::new(row as isize, col as isize))
        }).expect("start position (S) must exist")
    }

    fn get_tile(&self, pos: &Pos) -> Tile {
        self.tiles[pos.row as usize][pos.col as usize]
    }

    fn start_steps(&self, start: &Pos) -> (Step, Step) {
        let steps = Direction::ALL.iter()
            .filter_map(|dir| {
                dir.clipped_step(start, &self.size).filter(|step| self.get_tile(&step.pos).connects(dir.flip()))
            })
            .collect_vec();
        assert_eq!(steps.len(), 2);
        (steps[0], steps[1])
    }

    fn next(&self, step: &Step) -> Step {
        let tile = self.get_tile(&step.pos);
        let dir = tile.other(step.from);
        dir.clipped_step(&step.pos, &self.size).expect("in bounds")
    }

    fn find_loop(&self) -> (usize, Pos) {
        let start = self.start_pos();
        let (a, b) = self.start_steps(&start);
        let mut a = vec![a];
        let mut b = vec![b];
        while a.last().unwrap().pos != b.last().unwrap().pos {
            a.push(self.next(a.last().unwrap()));
            b.push(self.next(b.last().unwrap()));
        }
        (a.len(), a.last().unwrap().pos)
    }
}

fn part1(input: Lines) -> String {
    Field::from_lines(input).find_loop().0.to_string()
}

fn part2(input: Lines) -> String {
    input.take(0).count().to_string()
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
        verify!(part1, input, "8");
        verify!(part2, input, "0");
    }
}
