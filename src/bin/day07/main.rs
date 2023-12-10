use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::{str::Lines, fmt::Debug};

struct Rules {
    card_values: &'static str,
    wild: Option<u8>,
}

impl Rules {
    fn parse_card(&self, card: char) -> u8 {
        self.card_values.find(card).expect("bad card value") as u8
    }

    fn count_cards(&self, cards: &[u8]) -> [u8; 5] {
        let mut sorted_cards = cards.to_vec();
        sorted_cards.sort();
        let mut counts = [0u8; 5];
        let mut wild = 0;
        for (num, card) in sorted_cards.into_iter().dedup_with_count() {
            if self.wild.is_some_and(|w| w == card) {
                wild = num
            } else {
                counts[counts.len()-num] += 1;
            }
        }

        // if there is a wild, find the highest count and add the wilds to it
        if wild > 0 {
            if let Some((pos, count)) = counts.iter_mut().find_position(|count| count > &&mut 0u8) {
                *count -= 1;
                counts[pos - wild] += 1;
            } else {
                // handle all wilds case
                counts[0] = 1;
            }
        }
        counts
    }

    fn cards_str(&self, cards: &[u8]) -> String {
        cards.iter().map(|&c| self.card_values.as_bytes()[c as usize] as char).collect::<String>()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    counts: [u8; 5],
    cards: Vec<u8>,
}

impl Hand {
    fn new(hand: &str, rules: &Rules) -> Self {
        let cards = hand.chars().map(|c| rules.parse_card(c)).collect_vec();
        let counts = rules.count_cards(&cards);
        Self { counts, cards }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HandBid {
    hand: Hand,
    bid: usize,
}

impl HandBid {
    fn new(line: &str, rules: &Rules) -> Self {
        let (hand, bid) = line.split_once(' ').expect("space");
        Self {
            hand: Hand::new(hand, rules),
            bid: bid.parse::<usize>().expect("bid")
        }
    }
}

const RULES_PART1: Rules = Rules {
    card_values: "23456789TJQKA",
    wild: None
};


fn part1(input: Lines) -> String {
    let mut hands = input.map(|line| HandBid::new(line, &RULES_PART1)).collect_vec();
    hands.sort();
    hands.into_iter()
        .enumerate()
        .map(|(i, hb)| (i+1) * hb.bid)
        .sum::<usize>()
        .to_string()
}

const RULES_PART2: Rules = Rules {
    card_values: "J23456789TQKA",
    wild: Some(0)
};

fn part2(input: Lines) -> String {
    let mut hands = input.map(|line| HandBid::new(line, &RULES_PART2)).collect_vec();
    hands.sort();
    for hb in hands.iter() {
        println!("hand={} cards={:?} counts={:?} bid={}", RULES_PART2.cards_str(&hb.hand.cards), hb.hand.cards, hb.hand.counts, hb.bid)
    }
    hands.into_iter()
        .enumerate()
        .map(|(i, hb)| (i+1) * hb.bid)
        .sum::<usize>()
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
        verify!(part1, input, "6440");
        verify!(part2, input, "5905");
    }
}
