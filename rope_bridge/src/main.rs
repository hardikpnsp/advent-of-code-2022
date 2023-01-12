use std::cell::RefCell;
use std::collections::HashSet;
use std::fs::File;
use std::rc::Rc;
use utils::read_lines;

#[derive(Debug)]
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
    id: char,
    position: Position,
    next: Option<Rc<RefCell<Knot>>>,
}

impl Knot {
    pub fn up(&mut self) {
        self.position.up();
    }

    pub fn down(&mut self) {
        self.position.down();
    }

    pub fn left(&mut self) {
        self.position.left();
    }

    pub fn right(&mut self) {
        self.position.right();
    }

    pub fn update(&mut self, position: &Position) {
        if (self.position.x - position.x).abs() == 2 {
            // move left or right
            if self.position.x < position.x {
                self.right();
            } else {
                self.left();
            }

            if (self.position.y - position.y).abs() > 0 {
                // move up or down
                if self.position.y < position.y {
                    self.up();
                } else {
                    self.down();
                }
            }

            if let Some(next) = &self.next {
                next.borrow_mut().update(&self.position);
            }
        } else if (self.position.y - position.y).abs() == 2 {
            // move up or down
            if self.position.y < position.y {
                self.up();
            } else {
                self.down();
            }

            if (self.position.x - position.x).abs() > 0 {
                // move left or right
                if self.position.x < position.x {
                    self.right();
                } else {
                    self.left();
                }
            }

            if let Some(next) = &self.next {
                next.borrow_mut().update(&self.position);
            }
        }
    }

    pub fn position(&self) -> Position {
        self.position.clone()
    }
}

fn process_line(line: &str) -> (Direction, i64) {
    let (direction, steps) = line.split_once(" ").unwrap();
    return (
        Direction::from(direction.chars().nth(0).unwrap()),
        steps.parse().unwrap(),
    );
}

fn find_positions_visited_by_tail(lines: &Vec<(Direction, i64)>) -> usize {
    let tail = Rc::new(RefCell::new(Knot {
        id: 'T',
        position: Position { x: 0, y: 0 },
        next: None,
    }));
    let head = Rc::new(RefCell::new(Knot {
        id: 'H',
        position: Position { x: 0, y: 0 },
        next: Some(tail.clone()),
    }));
    positions_visited_by_tail(lines, tail, head)
}

fn find_positions_visited_by_tail_2(lines: &Vec<(Direction, i64)>) -> usize {
    let tail = Rc::new(RefCell::new(Knot {
        id: 'T',
        position: Position { x: 0, y: 0 },
        next: None,
    }));
    let k8 = Rc::new(RefCell::new(Knot {
        id: '8',
        position: Position { x: 0, y: 0 },
        next: Some(tail.clone()),
    }));
    let k7 = Rc::new(RefCell::new(Knot {
        id: '7',
        position: Position { x: 0, y: 0 },
        next: Some(k8.clone()),
    }));
    let k6 = Rc::new(RefCell::new(Knot {
        id: '6',
        position: Position { x: 0, y: 0 },
        next: Some(k7.clone()),
    }));
    let k5 = Rc::new(RefCell::new(Knot {
        id: '5',
        position: Position { x: 0, y: 0 },
        next: Some(k6.clone()),
    }));
    let k4 = Rc::new(RefCell::new(Knot {
        id: '4',
        position: Position { x: 0, y: 0 },
        next: Some(k5.clone()),
    }));
    let k3 = Rc::new(RefCell::new(Knot {
        id: '3',
        position: Position { x: 0, y: 0 },
        next: Some(k4.clone()),
    }));
    let k2 = Rc::new(RefCell::new(Knot {
        id: '2',
        position: Position { x: 0, y: 0 },
        next: Some(k3.clone()),
    }));
    let k1 = Rc::new(RefCell::new(Knot {
        id: '1',
        position: Position { x: 0, y: 0 },
        next: Some(k2.clone()),
    }));
    let head = Rc::new(RefCell::new(Knot {
        id: 'H',
        position: Position { x: 0, y: 0 },
        next: Some(k1.clone()),
    }));
    positions_visited_by_tail(lines, tail, head)
}

fn print_rope(head: Rc<RefCell<Knot>>) {
    const SIZE: i64 = 15;
    let mut result = vec![vec!['.'; (2 * SIZE) as usize]; (2 * SIZE) as usize];

    let mut current = head.clone();
    loop {
        if result[(SIZE - current.borrow().position.y) as usize]
            [(SIZE + current.borrow().position.x) as usize]
            == '.'
        {
            result[(SIZE - current.borrow().position.y) as usize]
                [(SIZE + current.borrow().position.x) as usize] = current.borrow().id;
        }
        if let Some(next) = current.clone().borrow().next.as_ref() {
            current = next.clone();
        } else {
            break;
        }
    }

    for row in result {
        println!("{:?}", row.iter().collect::<String>());
    }

    println!()
}

fn positions_visited_by_tail(
    lines: &Vec<(Direction, i64)>,
    tail: Rc<RefCell<Knot>>,
    head: Rc<RefCell<Knot>>,
) -> usize {
    let mut visited = HashSet::new();
    visited.insert(tail.borrow().position());
    // print_rope(head.clone());

    lines.iter().for_each(|(direction, step)| {
        for _ in 0..*step {
            match direction {
                Direction::Up => {
                    head.borrow_mut().up();
                    head.borrow()
                        .next
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .update(&(head.borrow().position));
                }
                Direction::Down => {
                    head.borrow_mut().down();
                    head.borrow()
                        .next
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .update(&(head.borrow().position));
                }
                Direction::Right => {
                    head.borrow_mut().right();
                    head.borrow()
                        .next
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .update(&(head.borrow().position));
                }
                Direction::Left => {
                    head.borrow_mut().left();
                    head.borrow()
                        .next
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .update(&(head.borrow().position));
                }
            }
            visited.insert(tail.borrow().position());
        }
        // println!("{:?} {:?}", direction, step);
        // print_rope(head.clone());
    });

    visited.len()
}

fn main() {
    let file = File::open("rope_bridge/inputs/input.txt").unwrap();
    let lines = read_lines(file);
    let lines = lines.iter().map(|line| process_line(line)).collect();
    println!(
        "Part 1: number of positions visited by the tail at least ones - {}",
        find_positions_visited_by_tail(&lines)
    );
    println!(
        "Part 2: number of positions visited by the tail at least ones when there are 10 knots - {}",
        find_positions_visited_by_tail_2(&lines)
    );
}
