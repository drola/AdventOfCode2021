use std::cmp::min;
/// Solution to an Advent of Code problem, day 15, 2021
/// https://adventofcode.com/2021/day/15
use std::env;
use std::fs;

fn enlarge_map(risk_levels: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let width = risk_levels[0].len();
    let height = risk_levels.len();
    assert_eq!(width, height);
    let size = width;

    let mut enlarged_map: Vec<Vec<u32>> = vec![vec![0; 5 * size]; 5 * size];

    for i in 0..5 {
        for j in 0..5 {
            for x in 0..size {
                for y in 0..size {
                    enlarged_map[size * j + y][size * i + x] =
                        (risk_levels[y][x] + (i as u32) + (j as u32) - 1) % 9 + 1;
                }
            }
        }
    }

    enlarged_map
}

fn optimal_path_bellman_ford(map: &[Vec<u32>]) -> u32 {
    let width = map[0].len();
    let height = map.len();
    assert_eq!(width, height);
    let size = width;
    let mut shortest_paths: Vec<Vec<u32>> = vec![vec![u32::MAX / 2; width]; height];
    shortest_paths[0][0] = 0;

    for _ in 0..size * size {
        for x in 0..size {
            for y in 0..size {
                if x > 0 {
                    shortest_paths[y][x] =
                        min(shortest_paths[y][x], map[y][x] + shortest_paths[y][x - 1]);
                }
                if x + 1 < size {
                    shortest_paths[y][x] =
                        min(shortest_paths[y][x], map[y][x] + shortest_paths[y][x + 1]);
                }
                if y > 0 {
                    shortest_paths[y][x] =
                        min(shortest_paths[y][x], map[y][x] + shortest_paths[y - 1][x]);
                }
                if y + 1 < size {
                    shortest_paths[y][x] =
                        min(shortest_paths[y][x], map[y][x] + shortest_paths[y + 1][x]);
                }
            }
        }
    }

    shortest_paths[size - 1][size - 1]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let risk_levels = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    println!("[part 1]: {}", optimal_path_bellman_ford(&risk_levels));
    let enlarged_map = enlarge_map(&risk_levels);
    println!("[part 2]: {}", optimal_path_bellman_ford(&enlarged_map));
}
