use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let numbers = contents
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap());

    // Fish internal counter histogram. Valid values: [0..8]
    let mut histogram: Vec<u64> = vec![0; 9];
    for n in numbers {
        histogram[n] = histogram[n] + 1;
    }

    // Simulate
    for day in 1..257 {
        let number_of_fish_with_timer_at_0 = histogram.remove(0);
        histogram[6] = histogram[6] + number_of_fish_with_timer_at_0; // timer of these fish resets to 6
        histogram.push(number_of_fish_with_timer_at_0); // offspring starts their timer at 8

        println!(
            "Lanternfish count (after day {}): {}",
            day,
            histogram.iter().sum::<u64>()
        );
    }
}
