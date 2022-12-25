use std::collections::{HashSet, VecDeque};
use std::fs::File;
use utils::read_lines;

struct Buffer {
    chars: VecDeque<char>,
    size: usize,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        Buffer {
            chars: VecDeque::new(),
            size,
        }
    }

    pub fn read(&mut self, c: char) {
        self.chars.push_back(c);
        if self.chars.len() > self.size {
            self.chars.pop_front();
        }
    }

    pub fn start_of_packet_marker(&self) -> bool {
        let set: HashSet<char> = HashSet::from_iter(self.chars.iter().cloned());
        set.len() == self.size
    }
}

fn main() {
    let file = File::open("tuning_trouble/input/input.txt").unwrap();
    let lines = read_lines(file);
    let line = lines.first().unwrap();

    let mut buffer = Buffer::new(4);

    for (index, c) in line.chars().enumerate() {
        buffer.read(c);
        if buffer.start_of_packet_marker() {
            println!("Part 1 - start of packet marker: {:?}", index + 1usize);
            break;
        }
    }
}
