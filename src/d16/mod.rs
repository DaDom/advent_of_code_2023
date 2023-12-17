use std::collections::HashSet;

use crate::d16::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::utils::{part_end, part_start};

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

type Beam = (i32, i32, Direction);

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = part_start(1);
    println!(
        "Result: {}",
        compute_energized_tile_count_from_top_left(include_bytes!("input"))
    );
    part_end(start);
}

fn part_two() {
    let start = part_start(2);
    println!(
        "Result: {}",
        compute_max_energized_tile_count_from_any_start(include_bytes!("input"))
    );
    part_end(start);
}

fn compute_energized_tile_count_from_top_left(bytes: &[u8]) -> usize {
    compute_energized_tile_count(bytes, &(0, 0, RIGHT))
}

fn compute_max_energized_tile_count_from_any_start(bytes: &[u8]) -> usize {
    let size = size(bytes);
    let row_count = size.0 as i32;
    let col_count = size.1 as i32;

    let from_left = (0..row_count)
        .map(|ri| compute_energized_tile_count(bytes, &(ri, 0, RIGHT)))
        .max()
        .unwrap();
    let from_right = (0..row_count)
        .map(|ri| compute_energized_tile_count(bytes, &(ri, col_count - 1, LEFT)))
        .max()
        .unwrap();
    let from_top = (0..col_count)
        .map(|ci| compute_energized_tile_count(bytes, &(0, ci, DOWN)))
        .max()
        .unwrap();
    let from_bottom = (0..col_count)
        .map(|ci| compute_energized_tile_count(bytes, &(row_count - 1, ci, UP)))
        .max()
        .unwrap();

    *[from_left, from_right, from_top, from_bottom].iter().max().unwrap()
}

fn compute_energized_tile_count(bytes: &[u8], first_beam: &Beam) -> usize {
    let (row_count, col_count) = size(bytes);
    let mut beams: Vec<Beam> = vec![*first_beam];
    let mut energized: HashSet<(usize, usize)> = HashSet::new();

    // To detect cycles, it is not enough to just check whether an already visited tile is visited
    // again. The direction matters too, e.g. when visiting a '/' from the left, that was already
    // visited before from the right, does not mean that we have a cycle yet.
    // TODO: Visiting '|' from the left or right is equivalent, and so is visiting '-' from top or bottom.
    //       This is not considered in the cycle detection, and hence potentially inefficient.
    let mut visited: HashSet<Beam> = HashSet::new();
    while !beams.is_empty() {
        let mut next_beams = vec![];
        for beam in beams {
            let (new_beams, new_energized) = get_next_beams(&beam, bytes, row_count, col_count);
            energized.extend(new_energized);
            for b in new_beams {
                if !visited.contains(&b) {
                    visited.insert(b.clone());
                    next_beams.push(b.clone());
                }
            }
        }

        beams = next_beams;
    }

    energized.len()
}

// Returns a set of energized tiles on the path of the beam, until hitting bounds or a tile that
// causes direction change. In the latter case, it returns a vector of new beams resulting from that change.
// TODO: Could use some kind of caching here, but didn't find time for that yet.
fn get_next_beams(
    beam: &Beam,
    bytes: &[u8],
    row_count: usize,
    col_count: usize,
) -> (Vec<Beam>, HashSet<(usize, usize)>) {
    let &(mut r, mut c, d) = beam;
    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    energized.insert((r as usize, c as usize));

    while r >= 0 && c >= 0 && r < row_count as i32 && c < col_count as i32 {
        match d {
            UP => r -= 1,
            RIGHT => c += 1,
            DOWN => r += 1,
            LEFT => c -= 1,
        }
        if r < 0 || c < 0 || r >= row_count as i32 || c >= col_count as i32 {
            return (vec![], energized);
        }
        energized.insert((r as usize, c as usize));
        let ch = bytes[r as usize * (col_count + 1) + c as usize];
        match ch {
            b'/' => {
                let dir = match d {
                    RIGHT => UP,
                    LEFT => DOWN,
                    DOWN => LEFT,
                    UP => RIGHT,
                };
                return (vec![(r, c, dir)], energized);
            }
            b'\\' => {
                let dir = match d {
                    RIGHT => DOWN,
                    LEFT => UP,
                    DOWN => RIGHT,
                    UP => LEFT,
                };
                return (vec![(r, c, dir)], energized);
            }
            b'|' => {
                if d == LEFT || d == RIGHT {
                    return (vec![(r, c, UP), (r, c, DOWN)], energized);
                }
            }
            b'-' => {
                if d == DOWN || d == UP {
                    return (vec![(r, c, LEFT), (r, c, RIGHT)], energized);
                }
            }
            _ => (),
        }
    }

    (vec![], energized)
}

fn size(bytes: &[u8]) -> (usize, usize) {
    let col_count = bytes.iter().position(|&b| b == b'\n').unwrap();
    let row_count = bytes.len() / col_count;
    (row_count, col_count)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_part_one() {
        let res = super::compute_energized_tile_count_from_top_left(include_bytes!("example"));
        assert_eq!(46, res);
    }

    #[test]
    fn example_part_two() {
        let res = super::compute_max_energized_tile_count_from_any_start(include_bytes!("example"));
        assert_eq!(51, res);
    }
}
