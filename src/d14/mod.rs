use std::collections::HashMap;

use itertools::Itertools;

use crate::d14::Direction::{EAST, NORTH, SOUTH, WEST};
use crate::utils::{part_end, part_start};

const LOOSE: u8 = b'O';
const FIXED: u8 = b'#';
const EMPTY: u8 = b'.';

#[derive(PartialEq, Eq)]
enum Direction {
    NORTH,
    WEST,
    SOUTH,
    EAST,
}

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = part_start(1);
    let mut bytes = include_bytes!("input").clone();
    println!("Result: {}", compute_load_after_single_north_tilt(&mut bytes));
    part_end(start);
}

fn part_two() {
    let start = part_start(2);
    let mut bytes = include_bytes!("input").clone();
    println!("Result: {}", compute_load_after_billion_cycles(&mut bytes));
    part_end(start);
}

fn compute_load_after_single_north_tilt(bytes: &mut [u8]) -> usize {
    let (row_count, col_count) = size(bytes);
    tilt(bytes, row_count, col_count, NORTH);
    calc_north_load(bytes, row_count, col_count)
}

fn compute_load_after_billion_cycles(bytes: &mut [u8]) -> usize {
    let (row_count, col_count) = size(bytes);
    let mut cache: HashMap<String, usize> = HashMap::new();
    let mut i = 0;
    let mut cycle_found = false;
    while i < 1_000_000_000 {
        tilt(bytes, row_count, col_count, NORTH);
        tilt(bytes, row_count, col_count, WEST);
        tilt(bytes, row_count, col_count, SOUTH);
        tilt(bytes, row_count, col_count, EAST);
        let cache_key = bytes.iter().map(|&b| b as char).join("");
        if cycle_found {
            i += 1;
        } else if cache.contains_key(&cache_key) {
            let cycle_start = cache.get(&cache_key).unwrap();
            let cycle_len = i - cycle_start;
            let div = (1_000_000_000 - cycle_start) / cycle_len;
            i = cycle_start + div * cycle_len + 1;
            cycle_found = true;
        } else {
            cache.insert(cache_key, i);
            i += 1;
        }
    }

    calc_north_load(bytes, row_count, col_count)
}

fn tilt(bytes: &mut [u8], row_count: usize, col_count: usize, dir: Direction) {
    let mut cur_fixed;
    let horizontal = dir == EAST || dir == WEST;
    let inc_fixed = dir == NORTH || dir == WEST;

    let mut process_char = |ri: usize, ci: usize, cur: &mut usize| {
        let c = bytes[ri * (col_count + 1) + ci];
        match c {
            LOOSE => {
                bytes[ri * (col_count + 1) + ci] = EMPTY;
                let next_ri = if horizontal { ri } else { *cur };
                let next_ci = if horizontal { *cur } else { ci };
                bytes[next_ri * (col_count + 1) + next_ci] = LOOSE;
                *cur = if inc_fixed { *cur + 1 } else { (*cur).max(1) - 1 };
            }
            FIXED => {
                let base = if horizontal { ci } else { ri };
                *cur = if inc_fixed { base + 1 } else { base.max(1) - 1 };
            }
            EMPTY => (),
            _ => unreachable!("Impossible char: {c}"),
        }
    };

    match dir {
        NORTH => {
            for ci in 0..col_count {
                cur_fixed = 0;
                for ri in 0..row_count {
                    process_char(ri, ci, &mut cur_fixed);
                }
            }
        }
        WEST => {
            for ri in 0..row_count {
                cur_fixed = 0;
                for ci in 0..col_count {
                    process_char(ri, ci, &mut cur_fixed);
                }
            }
        }
        SOUTH => {
            for ci in 0..col_count {
                cur_fixed = row_count - 1;
                for ri in (0..row_count).rev() {
                    process_char(ri, ci, &mut cur_fixed);
                }
            }
        }
        EAST => {
            for ri in 0..row_count {
                cur_fixed = col_count - 1;
                for ci in (0..col_count).rev() {
                    process_char(ri, ci, &mut cur_fixed);
                }
            }
        }
    }
}

fn calc_north_load(bytes: &[u8], row_count: usize, col_count: usize) -> usize {
    let mut res = 0usize;
    for ri in 0..row_count {
        for ci in 0..col_count {
            if bytes[ri * (col_count + 1) + ci] == LOOSE {
                res += row_count - ri;
            }
        }
    }

    res
}

fn size(bytes: &[u8]) -> (usize, usize) {
    let col_count = bytes.iter().position(|&b| b == b'\n').unwrap();
    let row_count = bytes.iter().filter(|&&b| b == b'\n').count() + 1;
    (row_count, col_count)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_part_one() {
        let mut bytes = include_bytes!("example").clone();
        let res = super::compute_load_after_single_north_tilt(&mut bytes);
        assert_eq!(136, res);
    }

    #[test]
    fn example_part_two() {
        let mut bytes = include_bytes!("example").clone();
        let res = super::compute_load_after_billion_cycles(&mut bytes);
        assert_eq!(64, res);
    }
}
