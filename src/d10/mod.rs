use std::collections::HashSet;

use itertools::Itertools;

use crate::d10::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::utils::{get_lines, part_end, part_start};

type Coord = (i32, i32);
type Grid = Vec<Vec<char>>;

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let start = part_start(1);
    println!("Result: {}", find_farthest_loop_tile_distance("d10/input"));
    part_end(start);
}

fn part_two() {
    let start = part_start(1);
    println!("Result: {}", find_enclosed_tile_count("d10/input"));
    part_end(start);
}

fn find_farthest_loop_tile_distance(file_path: &str) -> i32 {
    let (start, grid) = parse_input(file_path);
    let connected = find_connected_tiles(&grid, &start);
    let mut cur_coord = connected.into_iter().next().unwrap();
    let mut cur_char = get_unchecked(&grid, &cur_coord.0);
    let mut steps = 1;

    while cur_char != 'S' {
        cur_coord = next(&grid, &cur_coord.0, &cur_coord.1);
        cur_char = get_unchecked(&grid, &cur_coord.0);
        steps += 1;
    }

    steps / 2
}

fn find_enclosed_tile_count(file_path: &str) -> i32 {
    let (start, grid) = parse_input(file_path);
    let connected = find_connected_tiles(&grid, &start);
    let start_acts_as = determine_pipe_type(&connected);
    let mut cur_coord = connected.into_iter().next().unwrap();
    let mut cur_char = get_unchecked(&grid, &cur_coord.0);
    let mut tiles: HashSet<Coord> = HashSet::new();
    tiles.insert(start);

    while cur_char != 'S' {
        tiles.insert(cur_coord.0);
        cur_coord = next(&grid, &cur_coord.0, &cur_coord.1);
        cur_char = get_unchecked(&grid, &cur_coord.0);
    }

    grid.iter()
        .enumerate()
        .map(|(ri, row)| {
            let mut inside = false;
            row.iter()
                // We need to process the start pipe with its exact role in the given loop structure
                .map(|&c| if c == 'S' { start_acts_as } else { c })
                .enumerate()
                .fold(0, |acc, (ci, c)| {
                    if !tiles.contains(&(ri as i32, ci as i32)) {
                        return acc + inside as i32;
                    }

                    // We determine the amount of enclosed tiles by observing horizontal crossing
                    // of the loop. Some patterns require special attention, for example:
                    // - F-7 and L-J are not crossing the loop boundaries
                    // - F-J or L-7 however are
                    // We also need to process 'S' for the exact role it plays in the given structure.
                    if ['|'].contains(&c) {
                        inside = !inside;
                    } else if ['7', 'J'].contains(&c) {
                        let open_char = (0..ci)
                            .rev()
                            .map(|ci| row.get(ci).unwrap())
                            .map(|&c| if c == 'S' { start_acts_as } else { c })
                            .find(|&c| c != '-')
                            .unwrap();
                        if c == '7' && open_char == 'L' {
                            inside = !inside;
                        } else if c == 'J' && open_char == 'F' {
                            inside = !inside;
                        }
                    }

                    acc
                })
        })
        .sum()
}

fn get(grid: &Grid, coord: &Coord) -> Option<char> {
    if coord.0 < 0 || coord.1 < 0 {
        return None;
    }
    grid.get(coord.0 as usize)
        .and_then(|row| row.get(coord.1 as usize))
        .map(|&c| c)
}

fn get_unchecked(grid: &Grid, coord: &Coord) -> char {
    get(grid, coord).unwrap()
}

// Determines the next tile based on current tile + direction of the step that got us in there
// For example, if we moved into '-' by "moving left", we entered it from the right and
// consequently have to move left next.
fn next(grid: &Grid, pos: &Coord, prev_dir: &Direction) -> (Coord, Direction) {
    let c = get_unchecked(grid, pos);
    match (prev_dir, c) {
        (LEFT, '-') => move_left(pos),
        (LEFT, 'F') => move_down(pos),
        (LEFT, 'L') => move_up(pos),
        (RIGHT, '-') => move_right(pos),
        (RIGHT, '7') => move_down(pos),
        (RIGHT, 'J') => move_up(pos),
        (UP, '|') => move_up(pos),
        (UP, '7') => move_left(pos),
        (UP, 'F') => move_right(pos),
        (DOWN, '|') => move_down(pos),
        (DOWN, 'J') => move_left(pos),
        (DOWN, 'L') => move_right(pos),
        (d, c) => unreachable!("impossible move: {}/{:?}", c, d),
    }
}

// Determines the two connected tiles for a provided pipe position.
// Should return exactly two coordinates and the relative direction from the provided position.
fn find_connected_tiles(grid: &Grid, start: &Coord) -> Vec<(Coord, Direction)> {
    let up_coord = move_up(start);
    let right_coord = move_right(start);
    let down_coord = move_down(start);
    let left_coord = move_left(start);

    let up = get(grid, &up_coord.0)
        .filter(|c| ['|', '7', 'F'].contains(c))
        .map(|_| up_coord);
    let down = get(grid, &down_coord.0)
        .filter(|c| ['|', 'J', 'L'].contains(c))
        .map(|_| down_coord);
    let right = get(grid, &right_coord.0)
        .filter(|c| ['-', 'J', '7'].contains(c))
        .map(|_| right_coord);
    let left = get(grid, &left_coord.0)
        .filter(|c| ['-', 'L', 'F'].contains(c))
        .map(|_| left_coord);

    [up, down, right, left].into_iter().filter_map(|x| x).collect_vec()
}

// Based on the two connected pipes, we can determine what type of pipe it is.
// Necessary for determining the role of the starting point, when calculating enclosed tiles.
fn determine_pipe_type(connected: &Vec<(Coord, Direction)>) -> char {
    let up = connected.iter().any(|(_, d)| *d == UP);
    let right = connected.iter().any(|(_, d)| *d == RIGHT);
    let down = connected.iter().any(|(_, d)| *d == DOWN);
    let left = connected.iter().any(|(_, d)| *d == LEFT);

    match (up, right, down, left) {
        (true, false, true, false) => '|',
        (false, true, false, true) => '-',
        (true, true, false, false) => 'L',
        (true, false, false, true) => 'J',
        (false, false, true, true) => '7',
        (false, true, true, false) => 'F',
        x => unreachable!("impossible start neighbours: {:?}", x),
    }
}

fn move_left(pos: &Coord) -> (Coord, Direction) {
    ((pos.0, pos.1 - 1), LEFT)
}

fn move_right(pos: &Coord) -> (Coord, Direction) {
    ((pos.0, pos.1 + 1), RIGHT)
}

fn move_up(pos: &Coord) -> (Coord, Direction) {
    ((pos.0 - 1, pos.1), UP)
}

fn move_down(pos: &Coord) -> (Coord, Direction) {
    ((pos.0 + 1, pos.1), DOWN)
}

fn parse_input(file_path: &str) -> (Coord, Grid) {
    let mut start: Coord = (0, 0);
    let grid = get_lines(file_path)
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        start = (row as i32, col as i32);
                    }
                    c
                })
                .collect_vec()
        })
        .collect_vec();

    (start, grid)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_part_one() {
        let res = super::find_farthest_loop_tile_distance("d10/example_part1");
        assert_eq!(8, res);
    }

    #[test]
    fn example_part_two_a() {
        let res = super::find_enclosed_tile_count("d10/example_part2a");
        assert_eq!(4, res);
    }

    #[test]
    fn example_part_two_b() {
        let res = super::find_enclosed_tile_count("d10/example_part2b");
        assert_eq!(4, res);
    }

    #[test]
    fn example_part_two_c() {
        let res = super::find_enclosed_tile_count("d10/example_part2c");
        assert_eq!(8, res);
    }

    #[test]
    fn example_part_two_d() {
        let res = super::find_enclosed_tile_count("d10/example_part2d");
        assert_eq!(10, res);
    }
}
