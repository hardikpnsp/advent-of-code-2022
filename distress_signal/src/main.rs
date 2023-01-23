use std::fs::File;
use utils::read_lines;

#[derive(Debug)]
enum SignalValue {
    Value(i64),
    List(Vec<SignalValue>),
}

struct PacketParser {
    value: String,
    index: usize,
}

impl PacketParser {
    pub fn new(value: &str) -> Self {
        PacketParser {
            value: value.to_string(),
            index: 1,
        }
    }

    pub fn parse(&mut self) -> SignalValue {
        let mut signal_values: Vec<SignalValue> = vec![];
        while self.index < self.value.len() {
            match self.value.chars().nth(self.index).unwrap() {
                ']' => {
                    self.index += 1;
                    return SignalValue::List(signal_values);
                }
                '[' => {
                    self.index += 1;
                    signal_values.push(self.parse());
                }
                ',' => self.index += 1,
                _ => {
                    let mut value = String::new();
                    while self.index < self.value.len()
                        && self.value.chars().nth(self.index).unwrap() != ','
                        && self.value.chars().nth(self.index).unwrap() != ']'
                    {
                        value.push(self.value.chars().nth(self.index).unwrap());
                        self.index += 1;
                    }
                    signal_values.push(SignalValue::Value(value.parse().unwrap()));
                }
            }
        }
        return SignalValue::List(signal_values);
    }
}

fn main() {
    let file = File::open("distress_signal/inputs/input.txt").unwrap();
    let lines = read_lines(file);

    lines.chunks(3).enumerate().for_each(|(index, packets)| {
        println!("{}", index);
        for i in 0..2 {
            let mut parser = PacketParser::new(packets[i].as_str());
            let signal_value = parser.parse();
            println!("{:?}", signal_value);
        }
        println!();
    });
}
