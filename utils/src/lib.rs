use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(file: File) -> Vec<String> {
    let br = BufReader::new(file);
    let lines: Vec<String> = br.lines().map(|line| line.unwrap()).collect();
    lines
}
