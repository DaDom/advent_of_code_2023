use itertools::Itertools;

use crate::utils::{get_lines, part_end, part_start, split_whitespace};

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = part_start(1);
    println!("Result: {}", calc_value_sum("d09/input", true));
    part_end(start);
}

fn part_two() {
    let start = part_start(2);
    println!("Result: {}", calc_value_sum("d09/input", false));
    part_end(start);
}

fn calc_value_sum(file_path: &str, next: bool) -> i64 {
    get_lines(file_path)
        .iter()
        .map(|line| split_whitespace::<i64>(line.as_str()))
        .map(|nums| extrapolate_value(&nums, next))
        .sum()
}

fn extrapolate_value(nums: &Vec<i64>, next: bool) -> i64 {
    if nums.is_empty() {
        panic!("Cannot determine next value for empty series");
    }

    let first = *nums.first().unwrap();
    let last = *nums.last().unwrap();
    if nums.iter().all(|&n| n == first) {
        return first;
    }

    let diffs = nums
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, n)| n - nums.get(i - 1).unwrap())
        .collect_vec();
    let value_for_diffs = extrapolate_value(&diffs, next);
    return if next { last + value_for_diffs } else { first - value_for_diffs };
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_part_one() {
        let res = super::calc_value_sum("d09/example", true);
        assert_eq!(114, res);
    }

    #[test]
    fn example_part_two() {
        let res = super::calc_value_sum("d09/example", false);
        assert_eq!(2, res);
    }

    #[test]
    fn should_calc_correct_next_value() {
        let res = super::extrapolate_value(&vec![10, 13, 16, 21, 30, 45], true);
        assert_eq!(68, res);
    }

    #[test]
    fn should_calc_correct_first_value() {
        let res = super::extrapolate_value(&vec![10, 13, 16, 21, 30, 45], false);
        assert_eq!(5, res);
    }
}
