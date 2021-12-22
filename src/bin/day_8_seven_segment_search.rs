/// Solution to an Advent of Code problem, day 8, 2021
/// https://adventofcode.com/2021/day/8
use std::env;
use std::fs;
use std::vec::Vec;

/************************************
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
**************************************/

struct Problem {
    test_patterns: [u8; 10],
    result_patterns: [u8; 4],
}

fn parse_pattern(pattern: &str) -> u8 {
    let mut p: u8 = 0;
    if pattern.contains('a') {
        p |= 0b10000000;
    }
    if pattern.contains('b') {
        p |= 0b01000000;
    }
    if pattern.contains('c') {
        p |= 0b00100000;
    }
    if pattern.contains('d') {
        p |= 0b00010000;
    }
    if pattern.contains('e') {
        p |= 0b00001000;
    }
    if pattern.contains('f') {
        p |= 0b00000100;
    }
    if pattern.contains('g') {
        p |= 0b00000010;
    }

    p
}

fn first_with_x_segments(i: &[u8], x: u32) -> u8 {
    *i.iter().find(|p| p.count_ones() == x).unwrap()
}

fn all_with_x_segments(i: &[u8], x: u32) -> Vec<u8> {
    i.iter().filter(|p| p.count_ones() == x).cloned().collect()
}

fn intersection(i: &[u8]) -> u8 {
    i.iter().cloned().reduce(|l, r| l & r).unwrap()
}

#[allow(clippy::many_single_char_names)]
fn resolve(p: Problem) -> u64 {
    let one = first_with_x_segments(&p.test_patterns, 2);
    let seven = first_with_x_segments(&p.test_patterns, 3);
    let four = first_with_x_segments(&p.test_patterns, 4);
    let eight = first_with_x_segments(&p.test_patterns, 7);
    let six_nine_zero = all_with_x_segments(&p.test_patterns, 6);
    let two_three_five = all_with_x_segments(&p.test_patterns, 5);

    let a = seven ^ one;
    let horizontal_segments = intersection(&two_three_five);
    let t: Vec<u8> = six_nine_zero
        .iter()
        .map(|p| p & (!horizontal_segments) & (!one))
        .collect();
    let b = first_with_x_segments(&t, 1);
    let e = all_with_x_segments(&t, 2)
        .iter()
        .map(|p| p & (!b))
        .next()
        .unwrap();
    let d = four & (!one) & (!b);
    let _g = horizontal_segments & (!a) & (!d);

    let six_nine_without_horizontal_segments = all_with_x_segments(
        &(six_nine_zero
            .iter()
            .cloned()
            .map(|p| p & (!horizontal_segments))
            .collect::<Vec<u8>>()),
        3,
    );

    let f = intersection(&six_nine_without_horizontal_segments) & (!b);
    let c = one & (!f);

    // map pattern -> decoded number
    let mut map: [u8; 256] = [0; 256];
    map[one as usize] = 1;
    map[(horizontal_segments | c | e) as usize] = 2;
    map[(horizontal_segments | one) as usize] = 3;
    map[four as usize] = 4;
    map[(horizontal_segments | b | f) as usize] = 5;
    map[(eight & (!c)) as usize] = 6;
    map[seven as usize] = 7;
    map[eight as usize] = 8;
    map[(eight & (!e)) as usize] = 9;

    let mut result: u64 = 0;

    result += map[p.result_patterns[0] as usize] as u64 * 1000;
    result += map[p.result_patterns[1] as usize] as u64 * 100;
    result += map[p.result_patterns[2] as usize] as u64 * 10;
    result += map[p.result_patterns[3] as usize] as u64;

    result
}

fn parse_line(line: &str) -> Problem {
    let lr: [&str; 2] = line.split(" | ").collect::<Vec<&str>>().try_into().unwrap();
    Problem {
        test_patterns: lr[0]
            .split(' ')
            .map(parse_pattern)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap(),
        result_patterns: lr[1]
            .split(' ')
            .map(parse_pattern)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let problems = contents.lines().map(parse_line);

    let part1: usize = problems
        .clone()
        .map(|problem| {
            problem
                .result_patterns
                .iter()
                .filter(|p| {
                    p.count_ones() == 2
                        || p.count_ones() == 3
                        || p.count_ones() == 4
                        || p.count_ones() == 7
                })
                .count()
        })
        .sum();
    println!("Simple digit count (part 1): {}", part1);

    let part2: u64 = problems.map(resolve).sum();
    println!("Sum (part 2): {}", part2);
}
