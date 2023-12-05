use std::cmp::min;

use itertools::Itertools;
use num::pow;

use crate::utils;

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = utils::part_start(1);
    println!("Result: {}", get_card_point_sum("d04/input"));
    utils::part_end(start);
}

fn part_two() {
    let start = utils::part_start(2);
    println!("Result: {}", get_won_card_count("d04/input"));
    utils::part_end(start);
}

fn get_won_card_count(file_path: &str) -> i32 {
    let cards = get_card_values(file_path);
    let mut counts: Vec<i32> = vec![1; cards.len()];
    for (i, (wins, _)) in cards.iter().enumerate() {
        let win_from = i + 1;
        let win_to = min(i + *wins as usize, counts.len() - 1);
        for j in win_from..=win_to {
            counts[j] += counts[i];
        }
    }

    counts.iter().sum()
}

fn get_card_point_sum(file_path: &str) -> i32 {
    get_card_values(file_path).iter().map(|vals| vals.1).sum()
}

fn get_card_values(file_path: &str) -> Vec<(i32, i32)> {
    utils::get_lines(file_path)
        .iter()
        .map(|line| line.split_once(':').unwrap().1.trim())
        .map(|line| line.split_once('|').unwrap())
        .map(|(win, mine)| (split_nums(win), split_nums(mine)))
        .map(|(win, mine)| win.iter().filter(|&n| mine.contains(n)).count())
        .map(|n| (n as i32, if n <= 0 { 0 } else { pow(2, n - 1) }))
        .collect_vec()
}

fn split_nums(target: &str) -> Vec<i32> {
    target
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_part_one() {
        let res = super::get_card_point_sum("d04/example");
        assert_eq!(13, res);
    }

    #[test]
    fn example_part_two() {
        let res = super::get_won_card_count("d04/example");
        assert_eq!(30, res);
    }
}
