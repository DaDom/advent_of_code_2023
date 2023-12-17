use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::{get_lines, part_end, part_start};

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = part_start(1);
    println!("Result: {}", compute_hash_sum("d15/input"));
    part_end(start);
}

fn part_two() {
    let start = part_start(2);
    println!("Result: {}", compute_focus_power_sum("d15/input"));
    part_end(start);
}

fn compute_hash_sum(file_path: &str) -> u32 {
    parse_input(file_path).iter().map(|p| hash(p.as_str())).sum()
}

fn compute_focus_power_sum(file_path: &str) -> u32 {
    let ops = parse_input(file_path);
    let mut boxes: HashMap<u8, Vec<(String, u8)>> = HashMap::new();

    for op in ops {
        if op.ends_with('-') {
            let label = &op[..op.len() - 1];
            let b = hash(label) as u8;
            boxes.get_mut(&b).map(|list| list.retain(|(s, _)| s != label));
        } else {
            let label = &op[..op.len() - 2];
            let focal_length = op.chars().last().unwrap().to_digit(10).unwrap() as u8;
            let b = hash(label) as u8;
            if boxes.contains_key(&b) {
                let lenses = boxes.get_mut(&b).unwrap();
                let existing = lenses.iter_mut().find(|(s, _)| s == label);
                if let Some(lense) = existing {
                    lense.1 = focal_length;
                } else {
                    lenses.push((String::from(label), focal_length));
                }
            } else {
                boxes.insert(b, vec![(String::from(label), focal_length)]);
            }
        }
    }

    boxes
        .iter()
        .map(|(&bi, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(li, &(_, fl))| (bi as u32 + 1) * (li as u32 + 1) * fl as u32)
                .sum::<u32>()
        })
        .sum()
}

fn hash(target: &str) -> u32 {
    target.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

fn parse_input(file_path: &str) -> Vec<String> {
    get_lines(file_path)
        .iter()
        .flat_map(|line| line.split(',').map(|x| String::from(x)))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_part_one() {
        let res = super::compute_hash_sum("d15/example");
        assert_eq!(1320, res);
    }

    #[test]
    fn example_part_two() {
        let res = super::compute_focus_power_sum("d15/example");
        assert_eq!(145, res);
    }
}
