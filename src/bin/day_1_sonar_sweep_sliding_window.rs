use std::env;
use std::fs;

fn count_increases<I: Iterator<Item = i64> + Clone>(it: I) -> u64 {
    let successors = it.clone().skip(1);
    it.zip(successors)
        .fold(0, |count, (n, next)| count + if next > n { 1 } else { 0 })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");

    let numbers = contents.lines().map(|v| v.parse::<i64>().unwrap());
    let windows = numbers
        .clone()
        .zip(numbers.clone().skip(1))
        .zip(numbers.skip(2));
    let window_sums = windows.map(|((a, b), c)| a + b + c);

    let count = count_increases(window_sums);
    println!("Times water got deeper (windowed sums): {}", count);
}
