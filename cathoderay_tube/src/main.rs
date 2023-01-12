use std::fs::File;
use utils::read_lines;

enum Instruction {
    Noop,
    Addx(i64),
}

impl Instruction {
    pub fn new(line: &str) -> Self {
        let mut split = line.split_whitespace();
        match split.next().unwrap() {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(split.next().unwrap().parse().unwrap()),
            _ => {
                panic!()
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct CpuState {
    cycle: i64,
    register_x: i64,
}

impl CpuState {
    pub fn next_instruction_cycles(&self, instruction: &Instruction) -> Vec<Self> {
        match instruction {
            Instruction::Noop => {
                vec![CpuState {
                    cycle: self.cycle + 1,
                    register_x: self.register_x,
                }]
            }
            Instruction::Addx(value) => {
                vec![
                    CpuState {
                        cycle: self.cycle + 1,
                        register_x: self.register_x,
                    },
                    CpuState {
                        cycle: self.cycle + 2,
                        register_x: self.register_x + value,
                    },
                ]
            }
        }
    }

    pub fn signal_strength(&self) -> i64 {
        self.register_x * self.cycle
    }
}

struct System {
    index: usize,
    cpu_state: CpuState,
    instructions: Vec<Instruction>,
}

impl System {
    pub fn new(lines: Vec<String>) -> Self {
        let instructions = lines.iter().map(|line| Instruction::new(line)).collect();
        let cpu_state = CpuState {
            cycle: 1,
            register_x: 1,
        };
        System {
            index: 0usize,
            cpu_state,
            instructions,
        }
    }
}

impl Iterator for System {
    type Item = Vec<CpuState>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.instructions.len() {
            None
        } else {
            let next_cpu_states = self
                .cpu_state
                .next_instruction_cycles(&self.instructions[self.index]);
            self.cpu_state = *next_cpu_states.last().unwrap();
            self.index += 1;
            Some(next_cpu_states)
        }
    }
}

fn part1(lines: Vec<String>) -> i64 {
    let system = System::new(lines);

    let mut cpu_states = system.into_iter().flat_map(|state| state);

    let every_40th_cycle = cpu_states.skip(18).step_by(40).take(6);

    let signal_strength: i64 = every_40th_cycle
        .map(|state| {
            println!("{:?}", state);
            state.signal_strength()
        })
        .sum();
    signal_strength
}

fn main() {
    let file = File::open("cathoderay_tube/inputs/input.txt").unwrap();
    let lines = read_lines(file);

    println!("Part 1: signal strength {:?}", part1(lines));
}
