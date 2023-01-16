use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum Operand {
    New,
    Old,
    Constant(i64),
}

impl Operand {
    pub fn new(s: &str) -> Self {
        match s {
            "new" => Operand::New,
            "old" => Operand::Old,
            s => Operand::Constant(s.parse().unwrap()),
        }
    }

    pub fn exec(&self, old: i64) -> i64 {
        match self {
            Operand::New => {
                panic!()
            }
            Operand::Old => old,
            Operand::Constant(x) => *x,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Sum(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
}

impl Operation {
    pub fn new(p1: &str, p2: &str, operation: &str) -> Self {
        match operation {
            "+" => Operation::Sum(Operand::new(p1), Operand::new(p2)),
            "-" => Operation::Sub(Operand::new(p1), Operand::new(p2)),
            "*" => Operation::Mul(Operand::new(p1), Operand::new(p2)),
            "/" => Operation::Div(Operand::new(p1), Operand::new(p2)),
            _ => {
                panic!()
            }
        }
    }

    pub fn exec(&self, old: i64) -> i64 {
        match self {
            Operation::Sum(p1, p2) => p1.exec(old) + p2.exec(old),
            Operation::Sub(p1, p2) => p1.exec(old) - p2.exec(old),
            Operation::Mul(p1, p2) => p1.exec(old) * p2.exec(old),
            Operation::Div(p1, p2) => p1.exec(old) / p2.exec(old),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: i64,
    items: Vec<i64>,
    divisibility_test: i64,
    true_monkey_id: i64,
    false_monkey_id: i64,
    operation: Operation,
    inspection_count: i64,
}

impl Monkey {
    pub fn new(
        id: &str,
        items: &str,
        divisibility_test: &str,
        true_monkey_id: &str,
        false_monkey_id: &str,
        p1: &str,
        p2: &str,
        operation: &str,
    ) -> Self {
        let id = id.parse().unwrap();
        let items = items
            .split(",")
            .into_iter()
            .map(|s| s.trim().parse().unwrap())
            .collect();
        let divisibility_test = divisibility_test.parse().unwrap();
        let true_monkey_id = true_monkey_id.parse().unwrap();
        let false_monkey_id = false_monkey_id.parse().unwrap();
        let operation = Operation::new(p1, p2, operation);

        Monkey {
            id,
            items,
            divisibility_test,
            true_monkey_id,
            false_monkey_id,
            operation,
            inspection_count: 0,
        }
    }

    pub fn add_item(&mut self, worry_level: i64) {
        self.items.push(worry_level);
    }

    pub fn process(&mut self) -> Vec<(i64, i64)> {
        let item_movement: Vec<(i64, i64)> = self
            .items
            .iter()
            .map(|worry_level| {
                println!(
                    "Monkey Inspects an item with a worry level of {}",
                    worry_level
                );
                self.inspection_count += 1;
                self.operation.exec(*worry_level)
            })
            .map(|worry_level| {
                println!("Worry level is now {}", worry_level);
                worry_level / 3
            })
            .map(|worry_level| {
                println!("Worry level is divided by 3 and is now {}", worry_level);
                if worry_level % self.divisibility_test == 0 {
                    println!("Item is throws to monkey {}", self.true_monkey_id);
                    (self.true_monkey_id, worry_level)
                } else {
                    println!("Item is throws to monkey {}", self.false_monkey_id);
                    (self.false_monkey_id, worry_level)
                }
            })
            .collect();

        self.items = vec![];
        item_movement
    }
}

fn part1(mut monkeys: HashMap<i64, Monkey>) -> i64 {
    let rounds = 20;

    for i in 0..rounds {
        println!("Round {}", i);
        for m in 0..monkeys.len() {
            println!("Monkey {}", m);
            let item_movement = monkeys.get_mut(&(m as i64)).unwrap().process();
            item_movement
                .into_iter()
                .for_each(|(monkey_id, worry_level)| {
                    monkeys.get_mut(&monkey_id).unwrap().add_item(worry_level);
                })
        }
        println!("After Round {}", rounds);
        for m in 0..monkeys.len() {
            println!(
                "Monkey {}: {:?}",
                m,
                monkeys.get(&(m as i64)).unwrap().items
            );
        }
    }

    for m in 0..monkeys.len() {
        println!(
            "Monkey {}: inspected items {} times",
            m,
            monkeys.get(&(m as i64)).unwrap().inspection_count
        );
    }

    let mut inspection_counts: Vec<i64> = monkeys
        .iter()
        .map(|(_, monkey)| monkey.inspection_count)
        .collect();

    inspection_counts.sort();

    inspection_counts.reverse();

    let part1 = inspection_counts[0] * inspection_counts[1];
    part1
}

fn main() {
    let mut file = File::open("monkey_in_the_middle/inputs/input.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    let text = text.as_str();
    let re = Regex::new(
        r#"Monkey (?P<id>\d*):
[ ]*Starting items: (?P<starting_items>[\d, ]+)
[ ]*Operation: (?P<lhs>.*) = (?P<p1>.*) (?P<operation>\+|-|\*|/) (?P<p2>.*)
[ ]*Test: divisible by (?P<divisible_by>\d*)
[ ]*If true: throw to monkey (?P<true_monkey>\d*)
[ ]*If false: throw to monkey (?P<false_monkey>\d*)"#,
    )
    .unwrap();

    let monkeys: HashMap<i64, Monkey> = re
        .captures_iter(text)
        .map(|captures| {
            let monkey_id = captures.name("id").unwrap().as_str();
            let starting_items = captures.name("starting_items").unwrap().as_str();
            let _lhs = captures.name("lhs").unwrap().as_str();
            let p1 = captures.name("p1").unwrap().as_str();
            let operation = captures.name("operation").unwrap().as_str();
            let p2 = captures.name("p2").unwrap().as_str();
            let divisible_by = captures.name("divisible_by").unwrap().as_str();
            let true_monkey = captures.name("true_monkey").unwrap().as_str();
            let false_monkey = captures.name("false_monkey").unwrap().as_str();
            (
                monkey_id.parse::<i64>().unwrap(),
                Monkey::new(
                    monkey_id,
                    starting_items,
                    divisible_by,
                    true_monkey,
                    false_monkey,
                    p1,
                    p2,
                    operation,
                ),
            )
        })
        .collect();

    println!("{:?}", monkeys);

    let part1 = part1(monkeys);

    println!(
        "Part 1: Most active monkeys item count multiplied - {}",
        part1
    );
}
