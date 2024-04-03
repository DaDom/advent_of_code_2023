use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;

use crate::d17::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::utils::{get_lines, part_end, part_start};

type Grid = Vec<Vec<u8>>;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
impl Direction {
    fn is_opposite(&self, dir: &Direction) -> bool {
        (*self == UP && *dir == DOWN)
            || (*self == DOWN && *dir == UP)
            || (*self == LEFT && *dir == RIGHT)
            || (*self == RIGHT && *dir == LEFT)
    }
}

#[derive(Eq, PartialEq)]
struct Pos {
    cum_heat_loss: u64,
    ri: usize,
    ci: usize,
    direction: Direction,
    dir_count: usize,
}
impl Pos {
    fn step(&self, grid: &Grid, dir: Direction, max_same_dir_moves: usize) -> Option<Self> {
        if self.direction.is_opposite(&dir) {
            return None;
        }
        if (self.ri == 0 && dir == UP) || (self.ci == 0 && dir == LEFT) {
            return None;
        }

        let next_ri = match dir {
            UP => self.ri - 1,
            DOWN => self.ri + 1,
            _ => self.ri,
        };
        let next_ci = match dir {
            LEFT => self.ci - 1,
            RIGHT => self.ci + 1,
            _ => self.ci,
        };
        let next_dir_count = if self.direction == dir { self.dir_count + 1 } else { 1 };
        let mut result = Pos {
            cum_heat_loss: self.cum_heat_loss,
            ri: next_ri,
            ci: next_ci,
            direction: dir,
            dir_count: next_dir_count,
        };

        if result.valid(grid, max_same_dir_moves) {
            result.cum_heat_loss += *grid.get(next_ri).unwrap().get(next_ci).unwrap() as u64;
            Some(result)
        } else {
            None
        }
    }

    fn done(&self, grid: &Grid, min_same_dir_moves: usize) -> bool {
        self.dir_count >= min_same_dir_moves
            && self.ri == grid.len() - 1
            && self.ci == grid.get(self.ri).unwrap().len() - 1
    }

    fn valid(&self, grid: &Grid, max_same_dir_moves: usize) -> bool {
        self.dir_count <= max_same_dir_moves && self.ri < grid.len() && self.ci < grid.get(self.ri).unwrap().len()
    }
}
impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        // Note that order of `other` and `self` is flipped
        // This turns the max-heap into a min-heap
        other.cum_heat_loss.cmp(&self.cum_heat_loss)
    }
}
impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = part_start(1);
    println!("Result: {}", compute_min_heat_loss("d17/input", 0, 3));
    part_end(start);
}

fn part_two() {
    let start = part_start(2);
    println!("Result: {}", compute_min_heat_loss("d17/input", 4, 10));
    part_end(start);
}

fn compute_min_heat_loss(file_path: &str, min_same_dir_moves: usize, max_same_dir_moves: usize) -> u64 {
    let grid = parse_input(file_path);
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    heap.push(Pos {
        cum_heat_loss: 0,
        ri: 0,
        ci: 0,
        direction: DOWN,
        dir_count: 0,
    });

    while let Some(cur_min) = heap.pop() {
        let dir_options = if cur_min.dir_count > 0 && cur_min.dir_count < min_same_dir_moves {
            vec![cur_min.direction.clone()]
        } else {
            vec![RIGHT, DOWN, LEFT, UP]
        };

        for dir in dir_options {
            if let Some(next) = cur_min.step(&grid, dir, max_same_dir_moves) {
                let seen_key = (next.ri, next.ci, next.direction.clone(), next.dir_count);
                if seen.contains(&seen_key) {
                    continue;
                }
                seen.insert(seen_key);

                if next.done(&grid, min_same_dir_moves) {
                    return next.cum_heat_loss;
                }

                heap.push(next);
            }
        }
    }

    panic!("Could not find solution")
}

fn parse_input(file_path: &str) -> Grid {
    get_lines(file_path)
        .iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as u8).collect_vec())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_part_one() {
        let res = super::compute_min_heat_loss("d17/example", 0, 3);
        assert_eq!(102, res);
    }

    #[test]
    fn example_part_two() {
        let res = super::compute_min_heat_loss("d17/example", 4, 10);
        assert_eq!(94, res);
    }
}
