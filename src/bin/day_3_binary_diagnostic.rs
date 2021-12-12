/// Solution to an Advent of Code problem, day 3, 2021
/// https://adventofcode.com/2021/day/3
use std::env;
use std::fs;

#[derive(Debug)]
struct BitStatistics {
    zeros_count: usize,
    ones_count: usize,
}

fn bit_statistics(lines: &Vec<Vec<char>>, digit_index: usize) -> BitStatistics {
    let mut ones_count = 0;
    let mut zeros_count = 0;

    for line in lines.iter() {
        if line[digit_index] == '1' {
            ones_count = ones_count + 1;
        } else {
            zeros_count = zeros_count + 1;
        }
    }

    BitStatistics {
        zeros_count: zeros_count,
        ones_count: ones_count,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines: Vec<Vec<char>> = contents
        .lines()
        .map(|line| Vec::<char>::from_iter(line.chars()))
        .collect();

    let diagnostic_length = lines[0].len();

    let mut exp = 1;
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for i in (0..diagnostic_length).rev() {
        let stats = bit_statistics(&lines, i);
        if stats.ones_count > stats.zeros_count {
            gamma_rate = gamma_rate + exp;
        } else {
            epsilon_rate = epsilon_rate + exp;
        }
        exp = exp * 2;
    }

    println!("Power consumption: {}", gamma_rate * epsilon_rate);

    let mut remaining_oxygen_generator_rating = lines.clone();
    let mut remaining_co2_scrubber_rating = lines.clone();

    for digit_index in 0..diagnostic_length {
        if remaining_oxygen_generator_rating.len() > 1 {
            let remaining_oxygen_generator_rating_stats =
                bit_statistics(&remaining_oxygen_generator_rating, digit_index);
            let oxygen_generator_wanted_bit = if remaining_oxygen_generator_rating_stats.zeros_count
                > remaining_oxygen_generator_rating_stats.ones_count
            {
                '0'
            } else {
                '1'
            };

            remaining_oxygen_generator_rating
                .retain(|line| line[digit_index] == oxygen_generator_wanted_bit);
        }
        if remaining_co2_scrubber_rating.len() > 1 {
            let remaining_co2_scrubber_rating_stats =
                bit_statistics(&remaining_co2_scrubber_rating, digit_index);
            let co2_scrubber_wanted_bit = if remaining_co2_scrubber_rating_stats.zeros_count
                > remaining_co2_scrubber_rating_stats.ones_count
            {
                '1'
            } else {
                '0'
            };
            remaining_co2_scrubber_rating
                .retain(|line| line[digit_index] == co2_scrubber_wanted_bit);
        }
    }

    let co2_scrubber_rating_bin = remaining_co2_scrubber_rating.pop().unwrap();
    let oxygen_generator_rating_bin = remaining_oxygen_generator_rating.pop().unwrap();

    exp = 1;
    let mut co2_scrubber_rating = 0;
    let mut oxygen_generator_rating = 0;
    for digit in (0..diagnostic_length).rev() {
        if oxygen_generator_rating_bin[digit] == '1' {
            oxygen_generator_rating = oxygen_generator_rating + exp;
        }
        if co2_scrubber_rating_bin[digit] == '1' {
            co2_scrubber_rating = co2_scrubber_rating + exp;
        }
        exp = exp * 2;
    }

    println!(
        "Life rating: {}",
        co2_scrubber_rating * oxygen_generator_rating
    );
}
