use std::env;
use std::fs;

fn cost_part_1<I: std::iter::Iterator<Item = i64>>(target: i64, initial_positions: I) -> i64 {
    initial_positions.map(|v| (target - v).abs()).sum()
}

fn cost_part_2<I: std::iter::Iterator<Item = i64>>(target: i64, initial_positions: I) -> i64 {
    initial_positions
        .map(|v| (target - v).abs())
        // https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF
        // https://en.wikipedia.org/wiki/Triangular_number
        .map(|n| (n * (n + 1)) / 2)
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let numbers = contents
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|v| v.parse::<i64>().unwrap());

    let min = numbers.clone().min().unwrap();
    let max = numbers.clone().max().unwrap();

    let minimal_cost_part_1 = (min..max + 1)
        .map(|target| cost_part_1(target, numbers.clone()))
        .min()
        .unwrap();
    println!("Minimal cost (part 1): {}", minimal_cost_part_1);
    
    let minimal_cost_part_2 = (min..max + 1)
        .map(|target| cost_part_2(target, numbers.clone()))
        .min()
        .unwrap();
    println!("Minimal cost (part 2): {}", minimal_cost_part_2);
}
