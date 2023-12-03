use itertools::Itertools;

use crate::utils;

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = utils::part_start(1);
    println!("Result: {}", find_included_number_sum("d03/input"));
    utils::part_end(start);
}

fn part_two() {
    let start = utils::part_start(2);
    println!("Result: {}", find_gear_ratio_sum("d03/input"));
    utils::part_end(start);
}

fn find_included_number_sum(file_path: &str) -> i32 {
    let lines = utils::get_lines(file_path);
    let nums = find_numbers_with_positions(&lines);
    let included_pos = find_included_positions(&lines);
    nums.iter()
        .filter(|(_, num_pos)| overlap(&included_pos, num_pos))
        .map(|(num, _)| *num)
        .sum()
}

fn find_gear_ratio_sum(file_path: &str) -> i32 {
    let lines = utils::get_lines(file_path);
    let nums = find_numbers_with_positions(&lines);
    lines
        .iter()
        .enumerate()
        .flat_map(|(ri, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '*')
                .map(|(ci, _)| {
                    let adj_pos = get_adjacent_positions(ri, ci, lines.len(), line.len());
                    nums.iter()
                        .filter(|(_, pos)| overlap(pos, &adj_pos))
                        .map(|(num, _)| *num)
                        .collect_vec()
                })
                .collect_vec()
        })
        .filter(|n| n.len() == 2)
        .map(|n| n.iter().product::<i32>())
        .sum()
}

fn find_numbers_with_positions(lines: &Vec<String>) -> Vec<(i32, Vec<(usize, usize)>)> {
    let mut nums: Vec<(i32, Vec<(usize, usize)>)> = Vec::new();
    let mut cur_num: Option<(String, Vec<(usize, usize)>)> = None;
    for (ri, line) in lines.iter().enumerate() {
        for (ci, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                cur_num = cur_num.or_else(|| Some((String::new(), Vec::new())));
                cur_num.as_mut().unwrap().0.push(c);
                cur_num.as_mut().unwrap().1.push((ri, ci));
            }
            if cur_num.is_some() && (ci == line.len() - 1 || !c.is_digit(10)) {
                let next = cur_num.unwrap();
                nums.push((next.0.parse::<i32>().unwrap(), next.1));
                cur_num = None;
            }
        }
    }

    nums
}

fn find_included_positions(lines: &Vec<String>) -> Vec<(usize, usize)> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(ri, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.' && !c.is_digit(10))
                .map(|(ci, _)| (ri, ci))
                .flat_map(|(ri, ci)| get_adjacent_positions(ri, ci, lines.len(), line.len()))
                .collect_vec()
        })
        .collect_vec()
}

fn get_adjacent_positions(
    center_ri: usize,
    center_ci: usize,
    max_ri: usize,
    max_ci: usize,
) -> Vec<(usize, usize)> {
    let ri = center_ri as i32;
    let ci = center_ci as i32;
    vec![
        (ri - 1, ci - 1),
        (ri - 1, ci),
        (ri - 1, ci + 1),
        (ri, ci - 1),
        (ri, ci + 1),
        (ri + 1, ci - 1),
        (ri + 1, ci),
        (ri + 1, ci + 1),
    ]
    .iter()
    .filter(|(ri, ci)| *ri >= 0 && *ci >= 0 && *ri < max_ri as i32 && *ci < max_ci as i32)
    .map(|(ri, ci)| (*ri as usize, *ci as usize))
    .collect_vec()
}

fn overlap<T>(a: &Vec<T>, b: &Vec<T>) -> bool
where
    T: PartialEq,
{
    return a.iter().any(|el| b.iter().contains(el));
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_part_one() {
        let res = super::find_included_number_sum("d03/example");
        assert_eq!(4361, res);
    }

    #[test]
    fn example_part_two() {
        let res = super::find_gear_ratio_sum("d03/example");
        assert_eq!(467835, res);
    }
}
