/// Solution to an Advent of Code problem, day 1, 2021
/// https://adventofcode.com/2021/day/1
use std::env;
use std::fs;

fn count_increases<I: Iterator<Item = i64> + Clone>(it: I) -> u64 {
    let successors = it.clone().skip(1);
    it.zip(successors)
        .fold(0, |count, (n, next)| count + if next > n { 1 } else { 0 })
}

fn part_1_times_water_got_deeper<I: std::iter::Iterator<Item = i64> + Clone>(numbers: I) -> u64 {
    count_increases(numbers)
}

fn part_2_times_water_got_deeper_windowed<I: std::iter::Iterator<Item = i64> + Clone>(
    numbers: I,
) -> u64 {
    let windows = numbers
        .clone()
        .zip(numbers.clone().skip(1))
        .zip(numbers.skip(2));
    let window_sums = windows.map(|((a, b), c)| a + b + c);

    count_increases(window_sums)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let numbers = contents.lines().map(|v| v.parse::<i64>().unwrap());
    println!(
        "[part 1] Times water got deeper: {}",
        part_1_times_water_got_deeper(numbers.clone())
    );
    println!(
        "[part 2] Times water got deeper (averaged depths): {}",
        part_2_times_water_got_deeper_windowed(numbers)
    );
}
