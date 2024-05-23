use std::str::FromStr;

use num::abs;

use crate::d18::Direction::{EAST, NORTH, SOUTH, WEST};
use crate::utils::{get_lines, part_end, part_start};

enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(NORTH),
            "R" => Ok(EAST),
            "D" => Ok(SOUTH),
            "L" => Ok(WEST),
            _ => Err(format!("Invalid direction string {s}")),
        }
    }
}

impl From<u8> for Direction {
    fn from(v: u8) -> Self {
        match v {
            0 => EAST,
            1 => SOUTH,
            2 => WEST,
            3 => NORTH,
            _ => panic!("Invalid direction index {v}"),
        }
    }
}

struct Step {
    direction: Direction,
    count: u64,
}

impl Step {
    fn v_steps(&self) -> i64 {
        match self.direction {
            NORTH => -1 * self.count as i64,
            SOUTH => 1 * self.count as i64,
            _ => 0,
        }
    }
    fn h_steps(&self) -> i64 {
        match self.direction {
            EAST => 1 * self.count as i64,
            WEST => -1 * self.count as i64,
            _ => 0,
        }
    }
}

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = part_start(1);
    println!("Result: {}", calc_cubic_meters("d18/input", false));
    part_end(start);
}

fn part_two() {
    let start = part_start(2);
    println!("Result: {}", calc_cubic_meters("d18/input", true));
    part_end(start);
}

fn calc_cubic_meters(file_path: &str, use_hex: bool) -> u64 {
    let steps = parse_input(file_path, use_hex);
    let mut cur = (0i64, 0i64);
    let mut edges = vec![];

    let mut outer_points = 0;
    for step in steps {
        outer_points += step.count;
        cur = (cur.0 + step.v_steps(), cur.1 + step.h_steps());
        edges.push(cur);
    }

    // Shoelace formula: https://en.wikipedia.org/wiki/Shoelace_formula
    let sum1 = (0..(edges.len() - 1))
        .into_iter()
        .map(|i| edges.get(i).unwrap().1 * edges.get(i + 1).unwrap().0)
        .sum::<i64>();
    let sum2 = (1..edges.len())
        .into_iter()
        .map(|i| edges.get(i).unwrap().1 * edges.get(i - 1).unwrap().0)
        .sum::<i64>();
    let inner_area = (0.5 * abs(sum1 - sum2) as f64).round() as u64;

    // Pick's theorem: https://en.wikipedia.org/wiki/Pick%27s_theorem
    // Reordered from "A = I + (B/2) - 1" to "I + B = A + (B/2) + 1"
    // A being the area, I being the number of internal points, B being the number of boundary points
    inner_area + (0.5 * outer_points as f64).round() as u64 + 1
}

fn parse_input(file_path: &str, use_hex: bool) -> Vec<Step> {
    return get_lines(file_path)
        .iter()
        .map(|line| line.split_whitespace())
        .map(|mut parts| {
            let dir = parts.next().unwrap();
            let count = parts.next().unwrap();
            let hex = parts.next().unwrap();

            if use_hex {
                let dir = hex.chars().nth(7).unwrap().to_digit(10).unwrap() as u8;
                let count = u64::from_str_radix(&hex[2..7], 16).unwrap();
                Step {
                    direction: dir.into(),
                    count,
                }
            } else {
                Step {
                    direction: dir.parse().unwrap(),
                    count: count.parse().unwrap(),
                }
            }
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use crate::d18::calc_cubic_meters;

    #[test]
    fn example_part_one() {
        let result = calc_cubic_meters("d18/example", false);
        assert_eq!(62, result);
    }

    #[test]
    fn example_part_two() {
        let result = calc_cubic_meters("d18/example", true);
        assert_eq!(952408144115, result);
    }
}
