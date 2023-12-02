use cmp::min;
use std::cmp;

use either::Left;
use either::Right;

use crate::utils;

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = utils::part_start(1);
    println!("Result: {}", calc_line_sum("d01/input"));
    utils::part_end(start);
}

fn part_two() {
    let start = utils::part_start(1);
    println!("Result: {}", calc_line_sum_with_written_digits("d01/input"));
    utils::part_end(start);
}

fn calc_line_sum(file_path: &str) -> i32 {
    utils::get_lines(file_path)
        .iter()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect::<String>())
        .map(|num| {
            format!(
                "{}{}",
                num.chars().next().unwrap(),
                num.chars().last().unwrap()
            )
        })
        .map(|num| num.parse::<i32>().unwrap())
        .sum::<i32>()
}

fn calc_line_sum_with_written_digits(file_path: &str) -> i32 {
    utils::get_lines(file_path)
        .iter()
        .map(|line| {
            format!(
                "{}{}",
                find_maybe_written_digit(line, false),
                find_maybe_written_digit(line, true)
            )
        })
        .map(|num| num.parse::<i32>().unwrap())
        .sum::<i32>()
}

fn find_maybe_written_digit(target: &str, reversed: bool) -> i32 {
    let written = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let left = Left(0..target.len());
    let right = Right((0..target.len()).rev());
    let mut range = if reversed { right } else { left };
    range
        .find_map(|i| {
            let c = target.chars().nth(i).unwrap();
            if c.is_digit(10) {
                return Some(c);
            }
            written
                .iter()
                .map(|(w, c)| {
                    if target[i..min(i + w.len(), target.len())] == **w {
                        Some(c)
                    } else {
                        None
                    }
                })
                .find_map(|c| c.map(|c| *c))
        })
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one_example() {
        let res = super::calc_line_sum("d01/example");
        assert_eq!(142, res);
    }

    #[test]
    fn part_two_example() {
        let res = super::calc_line_sum_with_written_digits("d01/example_part_2");
        assert_eq!(281, res);
    }

    #[test]
    fn find_char_digit() {
        let res = super::find_maybe_written_digit("de4jco3kd", false);
        assert_eq!(4, res);
    }

    #[test]
    fn find_char_digit_from_end() {
        let res = super::find_maybe_written_digit("de4jco3kd", true);
        assert_eq!(3, res);
    }

    #[test]
    fn find_written_digit() {
        let res = super::find_maybe_written_digit("dtwonee4jco3kd", false);
        assert_eq!(2, res);
    }

    #[test]
    fn find_written_digit_from_end() {
        let res = super::find_maybe_written_digit("de4jco3ktwoned", true);
        assert_eq!(1, res);
    }
}
