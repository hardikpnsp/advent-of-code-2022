extern crate core;

use std::fs::File;

fn process_input(input: Vec<String>) -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = vec![];

    let mut current_elf: Vec<i64> = vec![];
    for line in input {
        if line == "" {
            result.push(current_elf.to_vec());
            current_elf = vec![];
        } else {
            current_elf.push(line.parse().unwrap())
        }
    }
    result
}

fn main() -> std::io::Result<()> {
    let file = File::open("./calorie_counting/input/input.txt").unwrap();
    let input = utils::read_lines(file);
    let elves = process_input(input);
    let mut total_calories: Vec<i64> = elves.iter().map(|elf| elf.iter().sum()).collect();
    let max = total_calories.iter().max().unwrap();
    println!("Part 1: highest calories in elves = {:?}", max);
    total_calories.sort();
    println!(
        "Part 2: total of top 3 highest calories in elves = {:?}",
        total_calories.iter().rev().take(3).sum::<i64>()
    );
    Ok(())
}
