/// Solution to an Advent of Code problem, day 25, 2021
/// https://adventofcode.com/2021/day/25
use std::env;
use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum SeaCucumber {
    Eastbound,
    Southbound,
}

impl TryFrom<char> for SeaCucumber {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '>' => Ok(SeaCucumber::Eastbound),
            'v' => Ok(SeaCucumber::Southbound),
            _ => Err(()),
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();

    let mut map = lines
        .map(|line| {
            line.chars()
                .map(|c| c.try_into().ok())
                .collect::<Vec<Option<SeaCucumber>>>()
        })
        .collect::<Vec<Vec<Option<SeaCucumber>>>>();

    let map_w = map[0].len();
    let map_h = map.len();

    let mut step_index = 0;

    loop {
        let mut next: Vec<Vec<Option<SeaCucumber>>> = vec![vec![None; map_w]; map_h];
        let mut any_moved = false;
        step_index += 1;
        println!("Step {}", step_index);

        // eastbound go first
        for y in 0..map_h {
            for x in 0..map_w {
                let dest_x = (x + 1) % map_w;
                if map[y][x] == Some(SeaCucumber::Eastbound) {
                    if map[y][dest_x].is_none() {
                        next[y][dest_x] = Some(SeaCucumber::Eastbound);
                        any_moved = true;
                    } else {
                        next[y][x] = Some(SeaCucumber::Eastbound);
                    }
                }
            }
        }
        // southbound
        for y in 0..map_h {
            for x in 0..map_w {
                let dest_y = (y + 1) % map_h;
                if map[y][x] == Some(SeaCucumber::Southbound) {
                    if next[dest_y][x].is_none() && map[dest_y][x] != Some(SeaCucumber::Southbound)
                    {
                        next[dest_y][x] = Some(SeaCucumber::Southbound);
                        any_moved = true;
                    } else {
                        next[y][x] = Some(SeaCucumber::Southbound);
                    }
                }
            }
        }

        //print_map(&next);
        //break;

        if !any_moved {
            println!("Stopped moving after {} steps", step_index);
            break;
        }

        map = next;
    }
}
