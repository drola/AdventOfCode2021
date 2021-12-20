/// Solution to an Advent of Code problem, day 12, 2021
/// https://adventofcode.com/2021/day/12
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn is_small_cave(name: &str) -> bool {
    name.chars().next().unwrap().is_ascii_lowercase()
}

fn count_paths_from_cave(
    adjacent_caves: &HashMap<&str, HashSet<&str>>,
    cave: &str,
    visited_small_caves: &HashSet<&str>,
    allow_visit_to_one_small_cave_again: bool,
) -> usize {
    match cave {
        "end" => 1,
        _ => {
            let mut visited_small_caves_updated = visited_small_caves.clone();
            if is_small_cave(cave) {
                visited_small_caves_updated.insert(cave);
            }
            adjacent_caves
                .get(cave)
                .unwrap()
                .iter()
                .filter_map(|&next| {
                    if !visited_small_caves_updated.contains(next) {
                        Some((next, allow_visit_to_one_small_cave_again))
                    } else if allow_visit_to_one_small_cave_again && next != "start" {
                        Some((next, false))
                    } else {
                        None
                    }
                })
                .map(|(next, allow_visit_to_one_small_cave_again)| {
                    count_paths_from_cave(
                        adjacent_caves,
                        next,
                        &visited_small_caves_updated,
                        allow_visit_to_one_small_cave_again,
                    )
                })
                .sum::<usize>()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();

    let mut adjacent_caves: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in lines {
        if let Some((from, to)) = line.split_once('-') {
            if !adjacent_caves.contains_key(from) {
                adjacent_caves.insert(from, HashSet::new());
            }
            if !adjacent_caves.contains_key(to) {
                adjacent_caves.insert(to, HashSet::new());
            }
            adjacent_caves.get_mut(from).unwrap().insert(to);
            adjacent_caves.get_mut(to).unwrap().insert(from);
        }
    }

    println!(
        "[part 1] Path count (no repeated visits to small caves): {}",
        count_paths_from_cave(&adjacent_caves, "start", &HashSet::<&str>::new(), false)
    );
    println!(
        "[part 2] Path count (allow 1 repeated visit to 1 small cave): {}",
        count_paths_from_cave(&adjacent_caves, "start", &HashSet::<&str>::new(), true)
    );
}
