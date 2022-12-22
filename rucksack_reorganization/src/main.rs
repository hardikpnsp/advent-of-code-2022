use std::collections::HashSet;
use std::fs::File;
use utils::read_lines;

fn priority(item: char) -> i64 {
    if item <= 'Z' {
        item as i64 - 'A' as i64 + 27
    } else {
        item as i64 - 'a' as i64 + 1
    }
}

fn main() {
    let file = File::open("./rucksack_reorganization/input/input.txt").unwrap();
    let lines = read_lines(file);

    let common_chars = lines.iter().map(|line| {
        let (left, right) = line.split_at(line.len() / 2);
        let left: HashSet<char> = left.chars().collect();
        let right: HashSet<char> = right.chars().collect();
        let common_char = left.intersection(&right).next().unwrap();
        common_char.clone()
    });

    let total: i64 = common_chars.map(|item| priority(item)).sum();
    println!("Part 1: sum of the priorities of common items {:?}", total);

    let badges = lines.chunks(3).map(|chunks| {
        let common_char = chunks
            .iter()
            .map(|chunk| chunk.chars().collect::<HashSet<char>>())
            .reduce(|acc, element| acc.intersection(&element).cloned().collect())
            .unwrap();

        let common_char = common_char.iter().next().unwrap();
        return common_char.clone();
    });

    let badges_total: i64 = badges.map(|item| priority(item)).sum();
    println!(
        "Part 2: sum of the priorities of common items {:?}",
        badges_total
    );
}
