use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::{get_lines, part_end, part_start};

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = part_start(1);
    println!("Result: {}", calc_shortest_galaxy_paths_sum("d11/input", 2));
    part_end(start);
}

fn part_two() {
    let start = part_start(2);
    println!("Result: {}", calc_shortest_galaxy_paths_sum("d11/input", 1_000_000));
    part_end(start);
}

fn calc_shortest_galaxy_paths_sum(file_path: &str, growth_factor: usize) -> usize {
    let galaxy_coords = parse_input_and_expand(file_path, growth_factor);
    let mut sum = 0;
    for (i, &(ri1, ci1)) in galaxy_coords.iter().enumerate() {
        for &(ri2, ci2) in galaxy_coords.iter().skip(i + 1) {
            sum += ri1.abs_diff(ri2) + ci1.abs_diff(ci2);
        }
    }
    sum
}

fn parse_input_and_expand(file_path: &str, growth_factor: usize) -> Vec<(usize, usize)> {
    let mut empty_row_add = 0;
    let lines = get_lines(file_path);

    let mut galaxy_coords = lines
        .iter()
        .enumerate()
        .flat_map(|(ri, line)| {
            let row_gal_coords = line
                .chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(|(ci, _)| (ri + empty_row_add * (growth_factor - 1), ci))
                .collect_vec();
            // In case this row is empty, next galaxy rows are moved by +1
            empty_row_add += row_gal_coords.is_empty() as usize;
            row_gal_coords
        })
        .collect_vec();

    // Determine which column had how many empty columns in front of it...
    let col_count = lines.first().unwrap().len();
    let empty_col_add_by_col = (0..col_count).fold(HashMap::new(), |mut acc, ci| {
        let empty = galaxy_coords.iter().all(|&(_, gci)| gci != ci);
        let prev = if ci == 0 { 0 } else { *acc.get(&(ci - 1)).unwrap() };
        acc.insert(ci, prev + empty as usize);
        acc
    });

    // ...and then expand column indices accordingly
    galaxy_coords.iter_mut().for_each(|x| {
        x.1 += empty_col_add_by_col.get(&x.1).unwrap() * (growth_factor - 1);
    });

    galaxy_coords
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_part_one() {
        let res = super::calc_shortest_galaxy_paths_sum("d11/example", 2);
        assert_eq!(374, res);
    }

    #[test]
    fn example_part_two() {
        let mut res = super::calc_shortest_galaxy_paths_sum("d11/example", 10);
        assert_eq!(1030, res);
        res = super::calc_shortest_galaxy_paths_sum("d11/example", 100);
        assert_eq!(8410, res);
    }
}
