use std::collections::HashSet;
use std::fs::File;
use utils::read_lines;

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn from(c: char) -> Self {
        match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    pub fn up(&mut self) {
        self.y += 1;
    }
    pub fn down(&mut self) {
        self.y -= 1;
    }
    pub fn left(&mut self) {
        self.x -= 1;
    }
    pub fn right(&mut self) {
        self.x += 1;
    }
}

fn process_line(line: &str) -> (Direction, i64) {
    let (direction, steps) = line.split_once(" ").unwrap();
    return (
        Direction::from(direction.chars().nth(0).unwrap()),
        steps.parse().unwrap(),
    );
}

fn find_positions_visited_by_tail(lines: Vec<(Direction, i64)>) -> usize {
    let mut visited = HashSet::new();
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    visited.insert(tail);

    lines.iter().for_each(|(direction, step)| {
        for _ in 0..*step {
            match direction {
                Direction::Up => {
                    head.up();
                    if (head.y - tail.y).abs() == 2 {
                        tail.x = head.x;
                        tail.up();
                    } else if (head.y - tail.y).abs() > 2 {
                        panic!()
                    }
                }
                Direction::Down => {
                    head.down();
                    if (head.y - tail.y).abs() == 2 {
                        tail.x = head.x;
                        tail.down();
                    } else if (head.y - tail.y).abs() > 2 {
                        panic!()
                    }
                }
                Direction::Right => {
                    head.right();
                    if (head.x - tail.x).abs() == 2 {
                        tail.y = head.y;
                        tail.right();
                    } else if (head.x - tail.x).abs() > 2 {
                        panic!()
                    }
                }
                Direction::Left => {
                    head.left();
                    if (head.x - tail.x).abs() == 2 {
                        tail.y = head.y;
                        tail.left();
                    } else if (head.x - tail.x).abs() > 2 {
                        panic!()
                    }
                }
            }
            println!("{:?} {:?}", head, tail);
            visited.insert(tail);
        }
    });

    visited.len()
}

fn main() {
    let file = File::open("rope_bridge/inputs/input.txt").unwrap();
    let lines = read_lines(file);
    let lines = lines.iter().map(|line| process_line(line)).collect();
    println!(
        "Part 1: number of positions visited by the tail at least ones - {}",
        find_positions_visited_by_tail(lines)
    );
}
