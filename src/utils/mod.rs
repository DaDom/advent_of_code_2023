use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

pub fn get_input_file(file_path: &str) -> File {
    let path_string = format!("./src/{}", &file_path);
    let path = Path::new(&path_string);
    return File::open(path).unwrap();
}

pub fn get_lines(day: &str) -> Vec<String> {
    let file = get_input_file(day);
    let reader = BufReader::new(file);
    return reader.lines().map(|x| x.unwrap()).collect::<Vec<_>>();
}

pub fn part_start(num: i32) -> Instant {
    println!("**** PART {} ****", num);
    Instant::now()
}

pub fn part_end(start: Instant) {
    println!("Finished in {:.2?}", start.elapsed());
}
