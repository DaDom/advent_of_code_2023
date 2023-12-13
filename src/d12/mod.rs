use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::{get_lines, part_end, part_start};

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = part_start(1);
    println!("Result: {}", compute_count_sum("d12/input"));
    part_end(start);
}

fn part_two() {
    let start = part_start(2);
    println!("Result: {}", compute_unfolded_count_sum("d12/input"));
    part_end(start);
}

fn compute_count_sum(file_path: &str) -> usize {
    let mut cache: HashMap<String, usize> = HashMap::new();
    parse_input(file_path)
        .iter()
        .map(|(row, counts)| get_or_compute_for_row(&skip_and_trim(row, 0), counts, &mut cache))
        .sum()
}

fn compute_unfolded_count_sum(file_path: &str) -> usize {
    let mut cache: HashMap<String, usize> = HashMap::new();
    parse_input(file_path)
        .iter()
        .map(|(row, counts)| {
            (
                [row; 5].iter().join("?"),
                [counts; 5].into_iter().flatten().map(|&n| n).collect_vec(),
            )
        })
        .map(|(row, counts)| get_or_compute_for_row(&skip_and_trim(&row, 0), &counts, &mut cache))
        .sum()
}

fn get_or_compute_for_row(r: &str, counts: &Vec<usize>, cache: &mut HashMap<String, usize>) -> usize {
    let cache_key = format!("{}_{}", r, counts.iter().join("_"));
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    let result = compute_for_row(r, counts, cache);
    cache.insert(cache_key, result);
    result
}

fn compute_for_row(r: &str, counts: &Vec<usize>, cache: &mut HashMap<String, usize>) -> usize {
    // When the full row was already processed, there should be no damage count left
    if r.is_empty() {
        return counts.is_empty() as usize;
    }
    // When there is no damage count left, there should only be '.' and '?' (which will be regarded as '.') remaining
    if counts.is_empty() {
        return !r.contains('#') as usize;
    }

    let c = *counts.first().unwrap();
    // This is invalid: "## 3" or "?? 3" or even ".. 3"
    if c > r.len() {
        return 0;
    }
    // There is only a single valid arrangement in case of: "### 3" or "??? 3" or "#?# 3", but not "??. 3" or "##. 3"
    if c == r.len() {
        return (!r.contains('.') && counts.len() == 1) as usize;
    }
    // Early termination in case the counts + single gap exceed row length
    if counts.iter().sum::<usize>() + counts.len() - 1 > r.len() {
        return 0;
    }

    let followed_by_damaged = r.chars().skip(c).next().unwrap() == '#';
    let next_count_has_operational = r[0..c].contains('.');
    // See below in which cases this is relevant
    let cannot_be_damaged = followed_by_damaged || next_count_has_operational;

    let first_char = r.chars().next().unwrap();
    match first_char {
        '#' => {
            // Such cases are invalid: "##. 3" or "#??# 3"
            if cannot_be_damaged {
                return 0;
            }
            let counts_without_first = &counts.into_iter().skip(1).map(|&n| n).collect_vec();
            get_or_compute_for_row(&skip_and_trim(r, c + 1), counts_without_first, cache)
        }
        '?' => {
            let result_when_not_placed = get_or_compute_for_row(&skip_and_trim(r, 1), counts, cache);
            // Next '?' must mean '.' in such cases: "??. 3" or "???# 3"
            if cannot_be_damaged {
                return result_when_not_placed;
            }
            let counts_without_first = &counts.into_iter().skip(1).map(|&n| n).collect_vec();
            let result_when_placed = get_or_compute_for_row(&skip_and_trim(r, c + 1), counts_without_first, cache);

            return result_when_placed + result_when_not_placed;
        }
        _ => unreachable!("Impossible char: {first_char}"),
    }
}

fn skip_and_trim(row: &str, skip: usize) -> String {
    String::from(row[skip..].trim_matches(|c| c == '.'))
}

fn parse_input(file_path: &str) -> Vec<(String, Vec<usize>)> {
    get_lines(file_path)
        .iter()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(s, nums)| {
            (
                String::from(s),
                nums.split(',').map(|n| n.parse::<usize>().unwrap()).collect_vec(),
            )
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn example_part_one() {
        let res = super::compute_count_sum("d12/example");
        assert_eq!(21, res);
    }

    #[test]
    fn example_part_two() {
        let res = super::compute_unfolded_count_sum("d12/example");
        assert_eq!(525152, res);
    }

    // Investigated some examples during debugging, so I kept them:
    #[test]
    fn test_case_1() {
        let res = super::compute_for_row("?#?#?#?#?#?#?#?", &vec![1, 3, 1, 6], &mut HashMap::new());
        assert_eq!(1, res);
    }

    #[test]
    fn test_case_2() {
        let res = super::compute_for_row("????.######..#####.", &vec![1, 6, 5], &mut HashMap::new());
        assert_eq!(4, res);
    }

    #[test]
    fn test_case_3() {
        let res = super::compute_for_row("?###????????", &vec![3, 2, 1], &mut HashMap::new());
        assert_eq!(10, res);
    }

    #[test]
    fn test_case_4() {
        let res = super::compute_for_row("??????#?#?#??", &vec![2, 2, 6], &mut HashMap::new());
        assert_eq!(1, res);
    }

    #[test]
    fn test_case_5() {
        let res = super::compute_for_row("??????.?##?#??#?????", &vec![3, 1, 12], &mut HashMap::new());
        assert_eq!(6, res);
    }

    #[test]
    fn test_case_6() {
        let res = super::compute_for_row("??#?.#??.?#?.???", &vec![2, 2, 2, 1, 1], &mut HashMap::new());
        assert_eq!(4, res);
    }

    #[test]
    fn test_case_7() {
        let res = super::compute_for_row("??#?????#?#?????", &vec![1, 5, 1, 3, 1], &mut HashMap::new());
        assert_eq!(2, res);
    }

    #[test]
    fn test_case_8() {
        let res = super::compute_for_row("?????#??#???##?.?#??", &vec![1, 6, 3, 3], &mut HashMap::new());
        assert_eq!(28, res);
    }

    #[test]
    fn test_case_9() {
        let res = super::compute_for_row("?#..??#.????####?##?", &vec![2, 3, 3, 4, 3], &mut HashMap::new());
        assert_eq!(1, res);
    }

    #[test]
    fn test_case_10() {
        let res = super::compute_for_row("??.?????????", &vec![2, 1, 2, 1], &mut HashMap::new());
        assert_eq!(21, res);
    }

    #[test]
    fn test_case_11() {
        let res = super::compute_for_row("?????????", &vec![1, 2, 1], &mut HashMap::new());
        assert_eq!(20, res);
    }

    #[test]
    fn test_case_12() {
        let res = super::compute_for_row("?.?????????", &vec![2, 1, 2, 1], &mut HashMap::new());
        assert_eq!(1, res);
    }
}
