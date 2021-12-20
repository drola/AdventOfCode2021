/// Solution to an Advent of Code problem, day 9, 2021
/// https://adventofcode.com/2021/day/9
use std::env;
use std::fs;

fn neighbor_coordinates(
    field: &[Vec<u32>],
    (x, y): (usize, usize),
) -> impl std::iter::Iterator<Item = (usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::with_capacity(4);

    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < field[y].len() - 1 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < field.len() - 1 {
        neighbors.push((x, y + 1));
    }

    neighbors.into_iter()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let height_map = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let point_coordinates = (0..height_map.len())
        .flat_map(|y| (0..height_map[y].len()).map(move |x| (x, y)))
        .collect::<Vec<(usize, usize)>>();

    let low_points = point_coordinates
        .iter()
        .cloned()
        .filter(|&point| {
            neighbor_coordinates(&height_map, point).all(|(neighbor_x, neighbor_y)| {
                height_map[neighbor_y][neighbor_x] > height_map[point.1][point.0]
            })
        })
        .collect::<Vec<(usize, usize)>>();

    let risk_levels = low_points.iter().map(|&(x, y)| height_map[y][x] + 1);
    println!("Sum of risk levels (part 1): {}", risk_levels.sum::<u32>());

    let mut basins: Vec<Vec<Option<usize>>> = (0..height_map.len())
        .map(|y| vec![None; height_map[y].len()])
        .collect();

    for (basin_index, &(x, y)) in low_points.iter().enumerate() {
        basins[y][x] = Some(basin_index);
    }

    let mut basin_sizes = low_points.iter().map(|_| 1).collect::<Vec<u64>>();

    let mut has_grown = true;
    while has_grown {
        has_grown = false;
        for &(x, y) in point_coordinates.iter() {
            if let Some(basin_index) = basins[y][x] {
                neighbor_coordinates(&height_map, (x, y)).for_each(
                    |(neighbor_x, neighbor_y)| {
                        if height_map[neighbor_y][neighbor_x] < 9
                            && basins[neighbor_y][neighbor_x].is_none()
                        {
                            basins[neighbor_y][neighbor_x] = Some(basin_index);
                            has_grown = true;
                            basin_sizes[basin_index] += 1;
                        }
                    },
                );
            };
        }
    }
    basin_sizes.sort_unstable();
    println!(
        "Product of three largest basin sizes (part 2): {}",
        basin_sizes.iter().rev().take(3).product::<u64>()
    );
}
