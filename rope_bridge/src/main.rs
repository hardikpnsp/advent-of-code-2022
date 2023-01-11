use std::cell::RefCell;
use std::collections::HashSet;
use std::fs::File;
use std::rc::Rc;
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

#[derive(Debug)]
struct Knot {
    position: Position,
    next: Option<Rc<RefCell<Knot>>>,
}

impl Knot {
    pub fn up(&mut self) {
        self.position.up();
        if let Some(next) = &self.next {
            let mut next = next.borrow_mut();
            if (self.position.y - next.position.y).abs() == 2 {
                next.position.x = self.position.x;
                next.up();
            } else if (self.position.y - next.position.y).abs() > 2 {
                panic!()
            }
        }
    }

    pub fn down(&mut self) {
        self.position.down();
        if let Some(next) = &self.next {
            let mut next = next.borrow_mut();
            if (self.position.y - next.position.y).abs() == 2 {
                next.position.x = self.position.x;
                next.down();
            } else if (self.position.y - next.position.y).abs() > 2 {
                panic!()
            }
        }
    }

    pub fn left(&mut self) {
        self.position.left();
        if let Some(next) = &self.next {
            let mut next = next.borrow_mut();
            if (self.position.x - next.position.x).abs() == 2 {
                next.position.y = self.position.y;
                next.left();
            } else if (self.position.x - next.position.x).abs() > 2 {
                panic!()
            }
        }
    }

    pub fn right(&mut self) {
        self.position.right();
        if let Some(next) = &self.next {
            let mut next = next.borrow_mut();
            if (self.position.x - next.position.x).abs() == 2 {
                next.position.y = self.position.y;
                next.right();
            } else if (self.position.x - next.position.x).abs() > 2 {
                panic!()
            }
        }
    }

    pub fn position(&self) -> Position {
        Position {
            x: self.position.x,
            y: self.position.y,
        }
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
    let tail = Rc::new(RefCell::new(Knot {
        position: Position { x: 0, y: 0 },
        next: None,
    }));
    let head = Knot {
        position: Position { x: 0, y: 0 },
        next: Some(tail.clone()),
    };
    positions_visited_by_tail(lines, tail, head)
}

fn positions_visited_by_tail(lines: Vec<(Direction, i64)>, tail: Rc<RefCell<Knot>>, mut head: Knot) -> usize {
    let mut visited = HashSet::new();
    visited.insert(tail.borrow().position());

    lines.iter().for_each(|(direction, step)| {
        for _ in 0..*step {
            match direction {
                Direction::Up => {
                    head.up();
                }
                Direction::Down => {
                    head.down();
                }
                Direction::Right => {
                    head.right();
                }
                Direction::Left => {
                    head.left();
                }
            }
            visited.insert(tail.borrow().position());
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
