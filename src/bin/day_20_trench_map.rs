/// Solution to an Advent of Code problem, day 20, 2021
/// https://adventofcode.com/2021/day/20
use std::env;
use std::fs;

fn pad(image: Vec<Vec<bool>>, padding: usize) -> Vec<Vec<bool>> {
    let mut original = image
        .iter()
        .map(|row| [vec![false; padding], row.to_vec(), vec![false; padding]].concat())
        .collect::<Vec<Vec<bool>>>();

    for _ in 0..padding {
        original.insert(0, vec![false; original[0].len()]);
        original.push(vec![false; original[0].len()]);
    }

    original
}

fn get_pixel_or_0(image: &[Vec<bool>], x_: usize, y_: usize, dx: i64, dy: i64) -> usize {
    let x = x_ as i64 + dx;
    let y = y_ as i64 + dy;

    if x < 0
        || y < 0
        || x >= image[0].len() as i64
        || y >= image.len() as i64
        || !image[y as usize][x as usize]
    {
        0
    } else {
        1
    }
}

fn enhance(algorithm: &[bool], original: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut enhanced = original.clone();

    let w = enhanced[0].len();
    let h = enhanced.len();

    for x in 0..w {
        for y in 0..h {
            enhanced[y][x] = algorithm[[
                get_pixel_or_0(&original, x, y, -1, -1),
                get_pixel_or_0(&original, x, y, 0, -1),
                get_pixel_or_0(&original, x, y, 1, -1),
                get_pixel_or_0(&original, x, y, -1, 0),
                get_pixel_or_0(&original, x, y, 0, 0),
                get_pixel_or_0(&original, x, y, 1, 0),
                get_pixel_or_0(&original, x, y, -1, 1),
                get_pixel_or_0(&original, x, y, 0, 1),
                get_pixel_or_0(&original, x, y, 1, 1),
            ]
            .iter()
            .fold(0, |acc, v| 2 * acc + v)];
        }
    }

    enhanced
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let mut lines = contents
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>());

    let algorithm = lines.next().unwrap();
    lines.next();

    let iterations = 50;

    // pad image to pretend it's infinite
    let mut image = pad(lines.collect::<Vec<Vec<bool>>>(), iterations * 2);
    for iteration in 0..iterations {
        image = enhance(&algorithm, image);
        if iteration + 1 == 2 || iteration + 1 == 50 {
            let w = image[0].len();
            let h = image.len();

            // skip() and take() below are to ignore boundaries with garbage
            // in theory, the image is infinite
            let lit_pixel_count: usize = image
                .iter()
                .skip(iteration)
                .take(h - iteration - iteration)
                .map(|row| {
                    row.iter()
                        .skip(iteration)
                        .take(w - iteration - iteration)
                        .fold(0, |acc, v| acc + (if *v { 1 } else { 0 }))
                })
                .sum();
            println!(
                "Lit pixel count after {} iterations: {}",
                iteration + 1,
                lit_pixel_count
            );
        }
    }
}
