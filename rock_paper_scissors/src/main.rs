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
    pub fn new(shape: char) -> Self {
        match shape {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissor,
            _ => panic!(),
        }
    }
}

struct Game {
    player: Shape,
    opponent: Shape,
}

impl Game {
    pub fn new(game_string: String) -> Game {
        let (opponent, player) = game_string.split_once(' ').unwrap();
        return Game {
            player: Shape::new(player.parse().unwrap()),
            opponent: Shape::new(opponent.parse().unwrap()),
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
    let score: i64 = lines
        .iter()
        .map(|line| Game::new(line.parse().unwrap()).score())
        .sum();
    println!("Part 1: highest calories in elves = {:?}", score);
}
