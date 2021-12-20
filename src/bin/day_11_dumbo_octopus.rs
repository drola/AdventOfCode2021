/// Solution to an Advent of Code problem, day 11, 2021
/// https://adventofcode.com/2021/day/11
use std::env;
use std::fs;

fn neighbor_list(max_x: usize, max_y: usize, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::with_capacity(8);

    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < max_x {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < max_y {
        neighbors.push((x, y + 1));
    }
    if x > 0 && y > 0 {
        neighbors.push((x - 1, y - 1));
    }
    if x > 0 && y < max_y {
        neighbors.push((x - 1, y + 1));
    }
    if x < max_x && y > 0 {
        neighbors.push((x + 1, y - 1));
    }
    if x < max_x && y < max_y {
        neighbors.push((x + 1, y + 1));
    }
    neighbors
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let mut octopus_map = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let max_x = 9;
    let max_y = 9;

    let neighbor_lists = (0..max_y + 1)
        .map(|y| {
            (0..max_x + 1)
                .map(|x| neighbor_list(max_x, max_y, (x, y)))
                .collect::<Vec<Vec<(usize, usize)>>>()
        })
        .collect::<Vec<Vec<Vec<(usize, usize)>>>>();

    let coordinate_list = (0..max_x + 1)
        .flat_map(|x| (0..max_y + 1).map(move |y| (x, y)))
        .collect::<Vec<(usize, usize)>>();

    let mut flash_count: u64 = 0;
    let mut step = 0;
    let mut all_flashed = false;
    while !all_flashed {
        step += 1;

        for &(x, y) in &coordinate_list {
            octopus_map[y][x] += 1;
        }

        let mut more_flashes = true;
        while more_flashes {
            more_flashes = false;

            for &(x, y) in &coordinate_list {
                if octopus_map[y][x] == 10 {
                    octopus_map[y][x] = 11;
                    flash_count += 1;
                    more_flashes = true;

                    for &(neighbor_x, neighbor_y) in &neighbor_lists[y][x] {
                        if octopus_map[neighbor_y][neighbor_x] < 10 {
                            octopus_map[neighbor_y][neighbor_x] += 1;
                        }
                    }
                }
            }
        }

        for &(x, y) in &coordinate_list {
            if octopus_map[y][x] > 10 {
                octopus_map[y][x] = 0;
            }
        }
        // println!("After step {}:", step);
        // for y in 0..max_y + 1 {
        //     println!("{:?}", octopus_map[y]);
        // }
        all_flashed = coordinate_list.iter().all(|&(x, y)| octopus_map[y][x] == 0);
        if all_flashed {
            println!("[part 2] All flashed in step {}", step);
        }

        if step == 100 {
            println!("[part 1] Flash count after 100 steps: {}", flash_count);
        }
    }
}
