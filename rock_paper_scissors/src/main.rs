use std::fs::File;
use utils::read_lines;

#[derive(Copy, Clone)]
enum GameResult {
    Victory = 6,
    Draw = 3,
    Loss = 0,
}

#[derive(Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Shape {
    pub fn new(shape: &str) -> Self {
        match shape {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissor,
            _ => panic!(),
        }
    }

    fn win(opponent: Shape) -> Shape {
        match opponent {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape::Rock,
        }
    }

    fn lose(opponent: Shape) -> Shape {
        match opponent {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        }
    }

    fn draw(opponent: Shape) -> Shape {
        opponent
    }

    pub fn from_strategy(strategy: &str, opponent: Shape) -> Self {
        match strategy {
            "X" => Shape::lose(opponent),
            "Y" => Shape::draw(opponent),
            "Z" => Shape::win(opponent),
            _ => panic!(),
        }
    }
}

struct Game {
    player: Shape,
    opponent: Shape,
}

impl Game {
    pub fn new(game_string: &String) -> Game {
        let (opponent, player) = game_string.split_once(' ').unwrap();
        return Game {
            player: Shape::new(player),
            opponent: Shape::new(opponent),
        };
    }

    pub fn new_part_2(game_string: &String) -> Game {
        let (opponent, strategy) = game_string.split_once(' ').unwrap();
        let opponent = Shape::new(opponent);
        return Game {
            player: Shape::from_strategy(strategy, opponent),
            opponent,
        };
    }

    pub fn score(self) -> i64 {
        self.player as i64 + self.play() as i64
    }

    fn play(&self) -> GameResult {
        match self.opponent {
            Shape::Rock => match self.player {
                Shape::Rock => GameResult::Draw,
                Shape::Paper => GameResult::Victory,
                Shape::Scissor => GameResult::Loss,
            },
            Shape::Paper => match self.player {
                Shape::Rock => GameResult::Loss,
                Shape::Paper => GameResult::Draw,
                Shape::Scissor => GameResult::Victory,
            },
            Shape::Scissor => match self.player {
                Shape::Rock => GameResult::Victory,
                Shape::Paper => GameResult::Loss,
                Shape::Scissor => GameResult::Draw,
            },
        }
    }
}

fn main() {
    let lines = read_lines(File::open("rock_paper_scissors/input/input.txt").unwrap());
    let score: i64 = lines.iter().map(|line| Game::new(line).score()).sum();
    println!("Part 1: highest calories in elves = {:?}", score);
    let score: i64 = lines
        .iter()
        .map(|line| Game::new_part_2(line).score())
        .sum();
    println!("Part 1: highest calories in elves = {:?}", score);
}
