/// Solution to an Advent of Code problem, day 14, 2021
/// https://adventofcode.com/2021/day/14
use std::env;
use std::fs;

fn difference_between_most_and_least_common_element(pair_frequencies: [[u64; 256]; 256]) -> u64 {
    let mut min = u64::MAX;
    let mut max = u64::MIN;

    let mut character_frequencies: [u64; 256] = [0; 256];
    for left in 0..256 {
        for right in 0..256 {
            character_frequencies[right as usize] +=
                pair_frequencies[left as usize][right as usize];
        }
    }

    for f in character_frequencies {
        if f < min && f > 0 {
            min = f;
        }
        if f > max {
            max = f;
        }
    }

    max - min
}

fn to_pair_frequencies(polymer_template: Vec<u8>) -> [[u64; 256]; 256] {
    let mut result = [[0; 256]; 256];
    for i in 0..(polymer_template.len() - 1) {
        let left = polymer_template[i];
        let right = polymer_template[i + 1];
        result[left as usize][right as usize] += 1;
    }

    // add a virtual pair [0][first character],
    // because difference_between_most_and_least_common_element will only count right characters
    result[0][polymer_template[0] as usize] = 1;

    result
}

/// Runs a polymerization step according to insertion_rules
fn polymerization_step(
    pair_frequencies: [[u64; 256]; 256],
    insertion_rules: [[Option<u8>; 256]; 256],
) -> [[u64; 256]; 256] {
    let mut result = [[0; 256]; 256];
    for left in 0..256 {
        for right in 0..256 {
            if let Some(insertion) = insertion_rules[left][right] {
                // Insertion of element 'insertion' between 'left' and 'right'
                // results in two new character pairs: 'left', 'insertion' AND 'insertion', 'left'
                result[left][insertion as usize] += pair_frequencies[left][right];
                result[insertion as usize][right] += pair_frequencies[left][right];
            } else {
                result[left][right] = pair_frequencies[left][right];
            }
        }
    }

    result
}

fn skip_until_ascii_uppercase<I: Iterator<Item = u8>>(i: &mut std::iter::Peekable<I>) {
    while i.next_if(|c| !c.is_ascii_uppercase()).is_some() {}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut bytes = fs::read(filename)
        .expect("Cannot read file")
        .into_iter()
        .peekable();
    let mut polymer_template: Vec<u8> = Vec::new();
    for chr in &mut bytes {
        if chr.is_ascii_uppercase() {
            polymer_template.push(chr);
        } else {
            break;
        }
    }
    skip_until_ascii_uppercase(&mut bytes);

    let mut insertion_rules: [[Option<u8>; 256]; 256] = [[None; 256]; 256];
    while bytes.peek().is_some() {
        let left = bytes.next().unwrap();
        let right = bytes.next().unwrap();
        skip_until_ascii_uppercase(&mut bytes);
        let insert = bytes.next().unwrap();
        insertion_rules[left as usize][right as usize] = Some(insert);
        skip_until_ascii_uppercase(&mut bytes);
    }

    let mut pair_frequencies = to_pair_frequencies(polymer_template);

    for _ in 0..10 {
        pair_frequencies = polymerization_step(pair_frequencies, insertion_rules);
    }

    println!(
        "[part 1]: Difference in quantities of most and least common elements after step 10: {}",
        difference_between_most_and_least_common_element(pair_frequencies)
    );
    for _ in 10..40 {
        pair_frequencies = polymerization_step(pair_frequencies, insertion_rules);
    }
    println!(
        "[part 2]: Difference in quantities of most and least common elements after step 40: {}",
        difference_between_most_and_least_common_element(pair_frequencies)
    );
}
