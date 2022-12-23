use std::fs::File;
use utils::read_lines;

#[derive(Debug, Clone, Copy)]
struct Move {
    quantity: i64,
    from: usize,
    to: usize,
}

impl Move {
    pub fn new(input_line: &String) -> Self {
        let digits: Vec<i64> = input_line
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(|c| c.parse::<i64>().unwrap())
            .collect();
        if let [quantity, from, to] = digits[..] {
            Move {
                quantity,
                from: from as usize - 1usize,
                to: to as usize - 1usize,
            }
        } else {
            panic!()
        }
    }
}

#[derive(Debug)]
struct SupplyStack {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

impl SupplyStack {
    fn stack_from_input(stack_input: &[String]) -> Vec<Vec<char>> {
        // 0, 1, 2, 3, 4, 5
        //             [  X
        // [  N  ]     [  C
        // 1 -> stack 0 -> 0 * 4 + 1
        // 5 -> stack 1 -> 1 * 4 + 1

        let total_stacks = ((stack_input.last().unwrap().len() - 1) / 4) + 1;
        let mut stacks = vec![vec!['X'; 0]; total_stacks];

        for line in stack_input.iter().rev().skip(1) {
            for (index, c) in line
                .chars()
                .enumerate()
                .skip(1)
                .step_by(4)
                .filter(|(_, c)| *c != ' ')
            {
                let stack_index = (index - 1usize) / 4;
                stacks[stack_index].push(c);
            }
        }

        stacks
    }

    fn moves_from_input(moves_input: &[String]) -> Vec<Move> {
        moves_input.iter().map(|line| Move::new(line)).collect()
    }

    pub fn from_input(input: &Vec<String>) -> Self {
        let stack_move_split = input.iter().position(|s| s == "").unwrap();
        SupplyStack {
            stacks: SupplyStack::stack_from_input(&input[..stack_move_split]),
            moves: SupplyStack::moves_from_input(&input[(stack_move_split + 1usize)..]),
        }
    }

    pub fn stack_top(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last())
            .filter_map(|x| x)
            .collect()
    }
}

trait CrateMover {
    fn execute_moves(self, supply_stack: &mut SupplyStack) -> ();
}

struct CrateMover9000 {}

impl CrateMover for CrateMover9000 {
    fn execute_moves(self, supply_stack: &mut SupplyStack) -> () {
        for m in &supply_stack.moves {
            for _ in 0..m.quantity {
                let c = supply_stack.stacks[m.from].pop().unwrap();
                supply_stack.stacks[m.to].push(c);
            }
        }
    }
}

struct CrateMover9001 {}

impl CrateMover for CrateMover9001 {
    fn execute_moves(self, supply_stack: &mut SupplyStack) -> () {
        for m in &supply_stack.moves {
            let len = supply_stack.stacks[m.from].len();
            let mut crates = supply_stack.stacks[m.from][(len - m.quantity as usize)..].to_vec();
            supply_stack.stacks[m.to].append(&mut crates);
            for _ in 0..m.quantity {
                supply_stack.stacks[m.from].pop().unwrap();
            }
        }
    }
}

fn main() {
    let file = File::open("supply_stacks/input/input.txt").unwrap();
    let lines = read_lines(file);
    let mut supply_stack = SupplyStack::from_input(&lines);

    let crate_mover_9000 = CrateMover9000 {};
    crate_mover_9000.execute_moves(&mut supply_stack);
    println!(
        "Part 1 - crates at top of the stack: {:?}",
        supply_stack.stack_top()
    );

    let mut supply_stack = SupplyStack::from_input(&lines);
    let crate_mover_9001 = CrateMover9001 {};
    crate_mover_9001.execute_moves(&mut supply_stack);
    println!(
        "Part 2 - crates at top of the stack with CrateMover9001: {:?}",
        supply_stack.stack_top()
    );
}
