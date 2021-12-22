/// Solution to an Advent of Code problem, day 22, 2021
/// https://adventofcode.com/2021/day/22
use std::cmp::max;
use std::cmp::min;
use std::env;
use std::fs;
use std::ops::Range;

#[derive(Debug, Copy, Clone)]
struct Cuboid {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
}

#[derive(Debug)]
struct Step {
    on_off: bool,
    cuboid: Cuboid,
}

fn clamp(x: i64, min_x: i64, max_x: i64) -> i64 {
    min(max_x, max(min_x, x))
}

fn clamp_range(range: &Range<i64>, min_x: i64, max_x: i64) -> Range<i64> {
    Range::<i64> {
        start: clamp(range.start, min_x, max_x),
        end: clamp(range.end, min_x, max_x),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();

    let steps = lines
        .map(|line| {
            let (on_off, ranges_) = line.split_once(' ').unwrap();
            let mut ranges = ranges_.split(',').map(|range_str| {
                let (from, to) = range_str
                    .split_once('=')
                    .unwrap()
                    .1
                    .split_once("..")
                    .unwrap();
                (from.parse::<i64>().unwrap(), to.parse::<i64>().unwrap())
            });
            let (x1, x2) = ranges.next().unwrap();
            let (y1, y2) = ranges.next().unwrap();
            let (z1, z2) = ranges.next().unwrap();
            Step {
                on_off: on_off == "on",
                cuboid: Cuboid {
                    x1,
                    x2,
                    y1,
                    y2,
                    z1,
                    z2,
                },
            }
        })
        .collect::<Vec<Step>>();

    let mut reactor: [[[bool; 101]; 101]; 101] = [[[false; 101]; 101]; 101];
    for step in steps.iter() {
        let x_range = clamp_range(&(step.cuboid.x1..step.cuboid.x2 + 1), -50, 51);
        let y_range = clamp_range(&(step.cuboid.y1..step.cuboid.y2 + 1), -50, 51);
        let z_range = clamp_range(&(step.cuboid.z1..step.cuboid.z2 + 1), -50, 51);
        for x in x_range.clone() {
            for y in y_range.clone() {
                for z in z_range.clone() {
                    let i = x + 50;
                    let j = y + 50;
                    let k = z + 50;
                    reactor[i as usize][j as usize][k as usize] = step.on_off;
                }
            }
        }
    }

    let mut on_count = 0;
    #[allow(clippy::needless_range_loop)]
    for i in 0..101 {
        for j in 0..101 {
            for k in 0..101 {
                if reactor[i][j][k] {
                    on_count += 1;
                }
            }
        }
    }
    println!(
        "[part 1]: ON Cuboids count (within -50..50 region): {}",
        on_count
    );

    let mut x_splits: Vec<i64> = vec![];
    let mut y_splits: Vec<i64> = vec![];
    let mut z_splits: Vec<i64> = vec![];
    for step in steps.iter() {
        x_splits.push(step.cuboid.x1 - 1);
        x_splits.push(step.cuboid.x1);
        x_splits.push(step.cuboid.x2);
        x_splits.push(step.cuboid.x2 + 1);
        y_splits.push(step.cuboid.y1 - 1);
        y_splits.push(step.cuboid.y1);
        y_splits.push(step.cuboid.y2);
        y_splits.push(step.cuboid.y2 + 1);
        z_splits.push(step.cuboid.z1 - 1);
        z_splits.push(step.cuboid.z1);
        z_splits.push(step.cuboid.z2);
        z_splits.push(step.cuboid.z2 + 1);
    }
    x_splits.sort_unstable();
    y_splits.sort_unstable();
    z_splits.sort_unstable();
    x_splits.dedup();
    y_splits.dedup();
    z_splits.dedup();

    let mut landscape = vec![vec![vec![false; x_splits.len()]; y_splits.len()]; z_splits.len()];
    for (step_index, step) in steps.iter().enumerate() {
        println!("Step {}", step_index + 1);

        let i1 = x_splits.binary_search(&step.cuboid.x1).unwrap();
        let i2 = x_splits.binary_search(&step.cuboid.x2).unwrap();
        let j1 = y_splits.binary_search(&step.cuboid.y1).unwrap();
        let j2 = y_splits.binary_search(&step.cuboid.y2).unwrap();
        let k1 = z_splits.binary_search(&step.cuboid.z1).unwrap();
        let k2 = z_splits.binary_search(&step.cuboid.z2).unwrap();
        #[allow(clippy::needless_range_loop)]
        for k in k1..(k2 + 1) {
            for j in j1..(j2 + 1) {
                for i in i1..(i2 + 1) {
                    landscape[k][j][i] = step.on_off;
                }
            }
        }
    }

    let mut total_count: u64 = 0;
    for k in 0..z_splits.len() - 1 {
        let dz = (z_splits[k + 1] - z_splits[k]) as u64;
        for j in 0..y_splits.len() - 1 {
            let dy = (y_splits[j + 1] - y_splits[j]) as u64;
            for i in 0..x_splits.len() - 1 {
                let dx = (x_splits[i + 1] - x_splits[i]) as u64;
                if landscape[k][j][i] {
                    total_count += dx * dy * dz;
                }
            }
        }
    }

    println!("[part 2]: Total count: {}", total_count);
}
