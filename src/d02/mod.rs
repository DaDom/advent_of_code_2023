use std::cmp::max;
use std::vec::IntoIter;

use crate::utils;

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = utils::part_start(1);
    println!("Result: {}", calc_valid_game_sum("d02/input"));
    utils::part_end(start);
}

fn part_two() {
    let start = utils::part_start(2);
    println!("Result: {}", calc_min_power_sum("d02/input"));
    utils::part_end(start);
}

fn calc_valid_game_sum(file_path: &str) -> i32 {
    // red, green, blue
    let max = (12, 13, 14);

    parse_lines(file_path)
        .filter(|(_, draws)| {
            draws
                .iter()
                .map(|draw| draw.split_once(" ").unwrap())
                .all(|(num_str, col)| {
                    let num = num_str.parse::<i32>().unwrap();
                    match col {
                        "red" => num <= max.0,
                        "green" => num <= max.1,
                        "blue" => num <= max.2,
                        _ => unreachable!("Invalid color: {}", col),
                    }
                })
        })
        .map(|(i, _)| i)
        .sum()
}

fn calc_min_power_sum(file_path: &str) -> i32 {
    parse_lines(file_path)
        .map(|(_, draws)| {
            draws.iter().map(|draw| draw.split_once(" ").unwrap()).fold(
                (0, 0, 0),
                |acc, (num_str, col)| {
                    let num = num_str.parse::<i32>().unwrap();
                    match col {
                        "red" => (max(num, acc.0), acc.1, acc.2),
                        "green" => (acc.0, max(num, acc.1), acc.2),
                        "blue" => (acc.0, acc.1, max(num, acc.2)),
                        _ => unreachable!("Invalid color: {}", col),
                    }
                },
            )
        })
        .map(|(a, b, c)| a * b * c)
        .sum()
}

fn parse_lines(file_path: &str) -> IntoIter<(i32, Vec<String>)> {
    let res = utils::get_lines(file_path)
        .iter()
        .map(|line| line.split_once(":").unwrap())
        .map(|(game, draws)| {
            (
                game.chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap(),
                draws
                    .split(|c| c == ';' || c == ',')
                    .map(|draw| draw.trim().to_string())
                    .collect::<Vec<String>>(),
            )
        })
        .collect::<Vec<(i32, Vec<String>)>>();

    res.into_iter()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one_example() {
        let res = super::calc_valid_game_sum("d02/example");
        assert_eq!(8, res);
    }

    #[test]
    fn part_two_example() {
        let res = super::calc_min_power_sum("d02/example");
        assert_eq!(2286, res);
    }
}
