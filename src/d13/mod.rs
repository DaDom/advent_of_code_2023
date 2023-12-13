use itertools::Itertools;

use crate::utils::{get_lines, part_end, part_start};

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = part_start(1);
    println!("Result: {}", calc_grid_points("d13/input", 0));
    part_end(start);
}

fn part_two() {
    let start = part_start(2);
    println!("Result: {}", calc_grid_points("d13/input", 1));
    part_end(start);
}

fn calc_grid_points(file_path: &str, expected_err: usize) -> usize {
    parse_input(file_path)
        .iter()
        .map(|grid| {
            let col_count = grid.first().unwrap().len();
            let col_points = (1..col_count)
                .find(|&i| is_symmetric_by_cols(grid, i, expected_err))
                .unwrap_or(0);
            let row_points = (1..grid.len())
                .find(|&i| is_symmetric_by_rows(grid, i, expected_err))
                .map(|p| p * 100usize)
                .unwrap_or(0);
            col_points + row_points
        })
        .sum()
}

fn is_symmetric_by_cols(grid: &Vec<Vec<char>>, col: usize, expected_err: usize) -> bool {
    let col_count = grid.first().unwrap().len();
    let radius = col.min(col_count - col);
    let mut errors = 0usize;
    for line in grid {
        for ci in 1..=radius {
            errors += (line[col - ci] != line[col + ci - 1]) as usize;
            if errors > expected_err {
                return false;
            }
        }
    }

    errors == expected_err
}

fn is_symmetric_by_rows(grid: &Vec<Vec<char>>, row: usize, expected_err: usize) -> bool {
    let radius = row.min(grid.len() - row);
    let mut errors = 0usize;
    for ri in 1..=radius {
        let row1 = grid.get(row - ri).unwrap();
        let row2 = grid.get(row + ri - 1).unwrap();
        for (ci, c) in row1.iter().enumerate() {
            errors += (c != row2.get(ci).unwrap()) as usize;
            if errors > expected_err {
                return false;
            }
        }
    }

    errors == expected_err
}

fn parse_input(file_path: &str) -> Vec<Vec<Vec<char>>> {
    get_lines(file_path).iter().fold(vec![vec![]], |mut acc, line| {
        if line.is_empty() {
            acc.push(Vec::new());
        } else {
            acc.last_mut().unwrap().push(line.chars().collect_vec());
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_part_one() {
        let res = super::calc_grid_points("d13/example", 0);
        assert_eq!(405, res);
    }

    #[test]
    fn example_part_two() {
        let res = super::calc_grid_points("d13/example", 1);
        assert_eq!(400, res);
    }
}
