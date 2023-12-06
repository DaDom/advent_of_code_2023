use std::iter::zip;

use itertools::Itertools;

use crate::utils::{get_lines, part_end, part_start};

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = part_start(1);
    println!("Result: {}", calc_win_options_product("d06/input"));
    part_end(start);
}

fn part_two() {
    let start = part_start(2);
    println!(
        "Result: {}",
        calc_win_options_product_single_race("d06/input")
    );
    part_end(start);
}

fn calc_win_options_product(file_path: &str) -> i64 {
    let parsed = get_lines(file_path)
        .iter()
        .take(2)
        .map(|line| line.split_once(':').unwrap().1)
        .map(|nums| split_nums(nums))
        .collect_vec();
    let races = zip(&parsed[0], &parsed[1]).collect_vec();

    races
        .iter()
        .map(|(&t, &d)| calc_win_options_count(t, d))
        .product()
}

fn calc_win_options_product_single_race(file_path: &str) -> i64 {
    let parsed = get_lines(file_path)
        .iter()
        .take(2)
        .map(|line| {
            line.chars()
                .filter(|c| c.is_digit(10))
                .join("")
                .parse::<i64>()
                .unwrap()
        })
        .collect_vec();

    calc_win_options_count(parsed[0], parsed[1])
}

fn calc_win_options_count(time: i64, distance: i64) -> i64 {
    // The question we try to answer is: When is x * (t - x) > d ?
    // Leading to quadratic formula: -x^2 + xt > d or -x^2 + xt - d > 0
    // Equality is reached for: x = (-t +/- sqrt(t^2 - 4d)) / -2
    // Or: (t +/- sqrt(t^2 - 4d)) / 2
    let t = time as f64;
    let d = distance as f64;
    let sol_1 = (t - f64::sqrt(t * t - 4f64 * d)) / 2f64;
    let sol_2 = (t + f64::sqrt(t * t - 4f64 * d)) / 2f64;
    let mut lower = f64::ceil(f64::min(sol_1, sol_2)) as i64;
    let mut upper = f64::floor(f64::max(sol_1, sol_2)) as i64;

    // In case our solutions lead to the exact record, but don't beat it, we have to adjust
    if lower * (time - lower) == distance {
        lower += 1;
    }
    if upper * (time - upper) == distance {
        upper -= 1;
    }

    upper - lower + 1
}

fn split_nums(target: &str) -> Vec<i64> {
    target
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_part_one() {
        let res = super::calc_win_options_product("d06/example");
        assert_eq!(288, res);
    }

    #[test]
    fn example_part_two() {
        let res = super::calc_win_options_product_single_race("d06/example");
        assert_eq!(71503, res);
    }
}
