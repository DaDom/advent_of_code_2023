use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::{get_lines, part_end, part_start};

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = part_start(1);
    println!("Result: {}", count_steps_to_zzz("d08/input"));
    part_end(start);
}

fn part_two() {
    let start = part_start(2);
    println!("Result: {}", count_steps_to_ending_with_z("d08/input"));
    part_end(start);
}

fn count_steps_to_zzz(file_path: &str) -> i64 {
    let (instr, maps) = parse_input(file_path);
    count_steps_to_end_with(&instr, &maps, "AAA", "ZZZ")
}

fn count_steps_to_ending_with_z(file_path: &str) -> i64 {
    let (instr, maps) = parse_input(file_path);
    maps.keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| count_steps_to_end_with(&instr, &maps, &k, "Z"))
        .fold(1, num_integer::lcm)
}

fn count_steps_to_end_with(
    instr: &Vec<bool>,
    maps: &HashMap<String, [String; 2]>,
    start: &str,
    end_with: &str,
) -> i64 {
    let instr_count = instr.len() as i64;
    let mut i = 0i64;
    let mut cur = start;
    while !cur.ends_with(end_with) {
        let take_right = *instr.get((i % instr_count) as usize).unwrap();
        cur = maps.get(cur).unwrap()[take_right as usize].as_str();
        i += 1;
    }

    i
}

fn parse_input(file_path: &str) -> (Vec<bool>, HashMap<String, [String; 2]>) {
    let lines = get_lines(file_path);
    let instr = lines
        .first()
        .unwrap()
        .chars()
        .map(|c| c == 'R')
        .collect_vec();

    let maps = lines
        .iter()
        .skip(2)
        .map(|l| {
            (
                l[0..3].to_string(),
                [l[7..10].to_string(), l[12..15].to_string()],
            )
        })
        .collect::<_>();

    (instr, maps)
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples_part_one() {
        let mut res = super::count_steps_to_zzz("d08/example1");
        assert_eq!(2, res);
        res = super::count_steps_to_zzz("d08/example2");
        assert_eq!(6, res);
    }

    #[test]
    fn examples_part_two() {
        let res = super::count_steps_to_ending_with_z("d08/example3");
        assert_eq!(6, res);
    }
}
