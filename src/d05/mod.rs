use std::cmp::min;

use itertools::Itertools;

use crate::utils::{get_lines, part_end, part_start, split_whitespace};

// (source start, source end inclusive, destination start, destination end inclusive)
type Mappings = Vec<(i64, i64, i64, i64)>;

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = part_start(1);
    println!("Result: {}", get_min_location_for_seeds("d05/input"));
    part_end(start);
}

fn part_two() {
    let start = part_start(2);
    println!("Result: {}", get_min_location_for_seed_ranges("d05/input"));
    part_end(start);
}

fn get_min_location_for_seeds(file_path: &str) -> i64 {
    let (seeds, mappings) = parse_input(file_path);
    seeds
        .iter()
        .map(|&s| {
            mappings.iter().fold(s, |acc, cur| {
                cur.iter()
                    .find(|m| m.0 <= acc && m.1 >= acc)
                    .map(|m| m.2 + acc - m.0)
                    .unwrap_or(acc)
            })
        })
        .min()
        .unwrap()
}

fn get_min_location_for_seed_ranges(file_path: &str) -> i64 {
    let (seeds, mappings) = parse_input(file_path);
    let ranges = seeds
        .chunks(2)
        .map(|c| (c[0], c[0] + c[1] - 1))
        .collect_vec();

    ranges
        .iter()
        .flat_map(|&init_range| {
            mappings.iter().fold(vec![init_range], |acc, ms| {
                acc.iter()
                    .flat_map(|range| {
                        let mut res = Vec::<(i64, i64)>::new();
                        let mut cur_index = range.0;
                        for m in ms.iter().filter(|m| m.0 <= range.1 && m.1 >= range.0) {
                            if m.0 > cur_index {
                                res.push((cur_index, m.0 - 1));
                            }
                            let source_start = cur_index;
                            let source_end = min(m.1, range.1);
                            let dest_start = source_start + m.2 - m.0;
                            let dest_end = source_end + m.3 - m.1;
                            res.push((dest_start, dest_end));
                            cur_index = source_end + 1;
                        }
                        if cur_index <= range.1 {
                            res.push((cur_index, range.1));
                        }
                        res
                    })
                    .collect_vec()
            })
        })
        .map(|r| r.0)
        .min()
        .unwrap()
}

fn parse_input(file_path: &str) -> (Vec<i64>, Vec<Mappings>) {
    let map_order = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    let lines = get_lines(file_path);
    let seeds = split_whitespace::<i64>(lines.first().unwrap().split_once(": ").unwrap().1);

    let mut mapping_index = 0usize;
    let mut mappings: Vec<Mappings> = lines.iter().skip(2).filter(|line| !line.is_empty()).fold(
        (0..map_order.len()).map(|_| Vec::new()).collect_vec(),
        |mut acc, line| {
            if line.ends_with(" map:") {
                mapping_index = map_order.iter().position(|&x| line.starts_with(x)).unwrap();
            } else {
                let n = split_whitespace::<i64>(line);
                acc[mapping_index].push((n[1], n[1] + n[2] - 1, n[0], n[0] + n[2] - 1));
            }
            acc
        },
    );

    // sort mappings by source start
    mappings
        .iter_mut()
        .for_each(|m| m.sort_by(|&a, &b| a.0.cmp(&b.0)));

    (seeds, mappings)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_part_one() {
        let res = super::get_min_location_for_seeds("d05/example");
        assert_eq!(35, res);
    }

    #[test]
    fn example_part_two() {
        let res = super::get_min_location_for_seed_ranges("d05/example");
        assert_eq!(46, res);
    }
}
