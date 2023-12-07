use std::cmp::Ordering;

use itertools::Itertools;

use HandType::{FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfKind, TwoPair};

use crate::utils::{get_lines, part_end, part_start};

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Eq, Ord)]
struct Hand {
    cards: String,
    bid: i32,
    hand_type: HandType,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type == other.hand_type {
            return Some(self.cards.cmp(&other.cards));
        }
        Some(self.hand_type.cmp(&other.hand_type))
    }
}

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = part_start(1);
    println!("Result: {}", calc_rank_bid_product_sum("d07/input", false));
    part_end(start);
}

fn part_two() {
    let start = part_start(2);
    println!("Result: {}", calc_rank_bid_product_sum("d07/input", true));
    part_end(start);
}

fn calc_rank_bid_product_sum(file_path: &str, jokers: bool) -> i64 {
    parse_input(file_path, jokers)
        .into_iter()
        .sorted()
        .enumerate()
        .map(|(i, h)| (i as i64 + 1) * h.bid as i64)
        .sum()
}

fn parse_input(file_path: &str, jokers: bool) -> Vec<Hand> {
    get_lines(file_path)
        .iter()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| Hand {
            // These replacements allow us to rely on standard alphanumeric sorting
            cards: hand
                .replace('A', "E")
                .replace('K', "D")
                .replace('Q', "C")
                .replace('J', if jokers { "1" } else { "B" })
                .replace('T', "A"),
            bid: bid.parse::<i32>().unwrap(),
            hand_type: determine_hand_type(hand, jokers),
        })
        .collect_vec()
}

fn determine_hand_type(hand: &str, jokers: bool) -> HandType {
    let joker_count = hand.chars().filter(|&c| jokers && c == 'J').count();
    if joker_count >= hand.len() - 1 {
        return FiveOfKind;
    }

    let mut counts = hand
        .chars()
        .filter(|&c| !jokers || c != 'J')
        .counts()
        .into_iter()
        .collect_vec();

    // Jokers simply increase the count of the most prevalent card
    let count_max = counts.iter().map(|(_, c)| *c).max().unwrap() + joker_count;
    match counts.len() {
        5 => HighCard,
        4 => OnePair,
        3 => return if count_max == 3 { ThreeOfKind } else { TwoPair },
        2 => return if count_max == 4 { FourOfKind } else { FullHouse },
        1 => FiveOfKind,
        _ => unreachable!("Impossible hand card count: {}", counts.len()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        let res = calc_rank_bid_product_sum("d07/example", false);
        assert_eq!(6440, res);
    }

    #[test]
    fn example_part_two() {
        let res = calc_rank_bid_product_sum("d07/example", true);
        assert_eq!(5905, res);
    }

    #[test]
    fn should_determine_correct_hand_types() {
        assert_eq!(HighCard, determine_hand_type("23456", false));
        assert_eq!(OnePair, determine_hand_type("23446", false));
        assert_eq!(TwoPair, determine_hand_type("33446", false));
        assert_eq!(ThreeOfKind, determine_hand_type("23444", false));
        assert_eq!(FullHouse, determine_hand_type("22444", false));
        assert_eq!(FourOfKind, determine_hand_type("24444", false));
        assert_eq!(FiveOfKind, determine_hand_type("44444", false));
        assert_eq!(OnePair, determine_hand_type("1234J", true));
        assert_eq!(ThreeOfKind, determine_hand_type("123JJ", true));
        assert_eq!(FourOfKind, determine_hand_type("12JJJ", true));
        assert_eq!(FiveOfKind, determine_hand_type("22JJJ", true));
        assert_eq!(FiveOfKind, determine_hand_type("JJJJJ", true));
    }
}
