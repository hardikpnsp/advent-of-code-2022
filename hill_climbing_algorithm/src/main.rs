use std::fs::File;
use utils::read_lines;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn neighbors(&self, width: usize, height: usize) -> Vec<Position> {
        let mut neighbors = vec![];

        if self.x > 0 {
            neighbors.push(Position {
                y: self.y,
                x: self.x - 1,
            });
        }
        if self.y > 0 {
            neighbors.push(Position {
                y: self.y - 1,
                x: self.x,
            });
        }

        if self.x + 1 < width {
            neighbors.push(Position {
                y: self.y,
                x: self.x + 1,
            });
        }
        if self.y + 1 < height {
            neighbors.push(Position {
                y: self.y + 1,
                x: self.x,
            });
        }

        neighbors
    }
}

struct RecursiveSolver {
    cost: Vec<Vec<i64>>,
    width: usize,
    height: usize,
}

impl RecursiveSolver {
    pub fn process(&mut self, current: &Position, map: &Vec<Vec<i64>>) {
        current
            .neighbors(self.width, self.height)
            .iter()
            .for_each(|next| {
                let elevation = map[next.y][next.x] - map[current.y][current.x];
                if elevation <= 1 && self.cost[next.y][next.x] > self.cost[current.y][current.x] + 1
                {
                    self.cost[next.y][next.x] = self.cost[current.y][current.x] + 1;
                    self.process(next, map);
                }
            });
    }
}

#[derive(Debug)]
struct Hill {
    map: Vec<Vec<i64>>,
    start: Position,
    end: Position,
}

impl Hill {
    pub fn find_position(field: &Vec<Vec<char>>, c: char) -> Position {
        for y in 0..field.len() {
            for x in 0..field[y].len() {
                if field[y][x] == c {
                    return Position { x, y };
                }
            }
        }
        panic!()
    }

    pub fn new(field: &Vec<Vec<char>>) -> Self {
        let map = field
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| match c {
                        'S' => 1,
                        'E' => 26,
                        &c => (c as i64) - ('a' as i64) + 1,
                    })
                    .collect()
            })
            .collect();

        let start = Hill::find_position(field, 'S');
        let end = Hill::find_position(field, 'E');

        Hill { map, start, end }
    }

    pub fn part1(&self) -> i64 {
        let height = self.map.len();
        let width = self.map[0].len();
        let mut cost = vec![vec![i64::MAX; self.map[0].len()]; self.map.len()];
        cost[self.start.y][self.start.x] = 0;

        let mut solver = RecursiveSolver {
            cost,
            width,
            height,
        };

        solver.process(&self.start, &self.map);

        solver.cost[self.end.y][self.end.x]
    }

    pub fn part2(&self) -> i64 {
        let height = self.map.len();
        let width = self.map[0].len();

        let start_positions: Vec<Position> = self
            .map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, elevation)| **elevation == 1)
                    .map(|(x, _)| Position { x, y })
                    .collect()
            })
            .flat_map(|x: Vec<Position>| x)
            .collect();

        println!("No of start positions: {}", start_positions.len());
        start_positions
            .iter()
            .map(|start| {
                let mut cost = vec![vec![i64::MAX; self.map[0].len()]; self.map.len()];
                cost[start.y][start.x] = 0;

                let mut solver = RecursiveSolver {
                    cost,
                    width,
                    height,
                };

                solver.process(&start, &self.map);

                solver.cost[self.end.y][self.end.x]
            })
            .min()
            .unwrap()
    }
}

// Note: For part 2, running with --release flag gives quicker result
// Might be able to optimize the solution by sharing the cost vector between multiple start_position runs
fn main() {
    let file = File::open("hill_climbing_algorithm/inputs/inputs.txt").unwrap();
    let lines = read_lines(file);

    let field = lines.iter().map(|line| line.chars().collect()).collect();

    let hill = Hill::new(&field);

    println!("Part 1 - minimum steps to reach goal - {}", hill.part1());
    println!(
        "Part 2 - minimum steps to reach goal from any starting point - {}",
        hill.part2()
    );
}
