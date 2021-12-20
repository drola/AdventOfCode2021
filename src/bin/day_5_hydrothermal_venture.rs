/// Solution to an Advent of Code problem, day 5, 2021
/// https://adventofcode.com/2021/day/5
use std::cmp::max;
use std::cmp::min;
use std::env;
use std::fs;

struct Line {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

impl Line {
    fn from_str(s: &str) -> Self {
        let from_to: [i64; 4] = s
            .replace(" -> ", ",")
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();

        Line {
            x1: from_to[0],
            y1: from_to[1],
            x2: from_to[2],
            y2: from_to[3],
        }
    }

    fn into_points(self) -> Box<dyn std::iter::Iterator<Item = (i64, i64)>> {
        let y_min = min(self.y1, self.y2);
        let y_max = max(self.y1, self.y2);
        let x_min = min(self.x1, self.x2);
        let x_max = max(self.x1, self.x2);

        if self.x1 == self.x2 {
            let x = self.x1;
            Box::new((y_min..(y_max + 1)).map(move |y| (x, y)))
        } else if self.y1 == self.y2 {
            let y = self.y1;
            Box::new((x_min..(x_max + 1)).map(move |x| (x, y)))
        } else if (y_max - y_min) == (x_max - x_min) {
            let x_direction: i64 = if self.x1 <= self.x2 { 1 } else { -1 };
            let y_direction: i64 = if self.y1 <= self.y2 { 1 } else { -1 };
            let k = x_direction * y_direction;
            let y_start = if k < 0 { y_max } else { y_min };
            Box::new((x_min..(x_max + 1)).map(move |x| (x, y_start + k * (x - x_min))))
        } else {
            Box::new(std::iter::empty())
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines().map(Line::from_str);

    let x_max = lines.clone().map(|l| max(l.x1, l.x2)).max().unwrap();
    let y_max = lines.clone().map(|l| max(l.y1, l.y2)).max().unwrap();
    let capacity: usize = ((y_max + 1) * (x_max + 1)).try_into().unwrap();
    let mut is_marked: Vec<bool> = vec![false; capacity];
    let mut is_marked_more_than_once: Vec<bool> = vec![false; capacity];

    for (x, y) in lines.flat_map(|line| line.into_points()) {
        let index: usize = (y * (x_max + 1) + x).try_into().unwrap();
        if !is_marked[index] {
            is_marked[index] = true;
        } else {
            is_marked_more_than_once[index] = true;
        }
    }

    let result = is_marked_more_than_once.into_iter().filter(|v| *v).count();

    println!("Result: {}", result);
}
