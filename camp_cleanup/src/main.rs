use std::fs::File;
use utils::read_lines;

struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    pub fn from(interval_string: &str) -> Self {
        let (left, right) = interval_string.split_once("-").unwrap();
        return Interval {
            start: left.parse().unwrap(),
            end: right.parse().unwrap(),
        }
    }

    pub fn contains(&self, interval: &Interval) -> bool {
        interval.start >= self.start && interval.end <= self.end
    }

    pub fn overlaps(&self, interval: &Interval) -> bool {
        (self.start <= interval.start && interval.start <= self.end) || (self.start <= interval.end && interval.end <= self.end) || interval.contains(self)
    }
}

fn main() {
    let file = File::open("./camp_cleanup/input/input.txt").unwrap();
    let lines = read_lines(file);
    let fully_contained = lines.iter().map(|line| {
        let (i1, i2) = line.split_once(",").unwrap();
        let (i1, i2) = (Interval::from(i1), Interval::from(i2));
        i1.contains(&i2) || i2.contains(&i1)
    }).filter(|contained| *contained).count();

    println!("Part 1 - fully contained intervals: {:?}", fully_contained);

    let ovelapped = lines.iter().map(|line| {
        let (i1, i2) = line.split_once(",").unwrap();
        let (i1, i2) = (Interval::from(i1), Interval::from(i2));
        i1.overlaps(&i2)
    }).filter(|contained| *contained).count();

    println!("Part 2 - overlapped intervals: {:?}", ovelapped);
}
