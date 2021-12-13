/// Solution to an Advent of Code problem, day 13, 2021
/// https://adventofcode.com/2021/day/13
use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord)]
struct Dot {
    x: u64,
    y: u64,
}

struct ParseDotError {}
impl FromStr for Dot {
    type Err = ParseDotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(',')
            .map(|(str_x, str_y)| Dot {
                x: str_x.parse::<u64>().unwrap(),
                y: str_y.parse::<u64>().unwrap(),
            })
            .ok_or(ParseDotError {})
    }
}

#[derive(Clone, Copy, Debug)]
enum FoldInstruction {
    FoldAlongX(u64),
    FoldAlongY(u64),
}
struct ParseFoldInstructionError {}
impl FromStr for FoldInstruction {
    type Err = ParseFoldInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once('=')
            .map(|(fold_along_cmd, v)| match fold_along_cmd {
                "fold along y" => FoldInstruction::FoldAlongY(v.parse::<u64>().unwrap()),
                _ => FoldInstruction::FoldAlongX(v.parse::<u64>().unwrap()),
            })
            .ok_or(ParseFoldInstructionError {})
    }
}

#[derive(Debug)]
struct Problem {
    dots: Vec<Dot>,
    fold_instructions: Vec<FoldInstruction>,
}

#[derive(Debug)]
struct ParseProblemError {}
impl FromStr for Problem {
    type Err = ParseProblemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Problem {
            dots: s
                .lines()
                .filter_map(|line| line.parse::<Dot>().ok())
                .collect(),
            fold_instructions: s
                .lines()
                .filter_map(|line| line.parse::<FoldInstruction>().ok())
                .collect(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let mut problem = contents.parse::<Problem>().unwrap();
    //println!("{:?}", problem);

    for (i, &fold_instruction) in problem.fold_instructions.iter().enumerate() {
        match fold_instruction {
            FoldInstruction::FoldAlongX(fold_line_x) => {
                problem.dots = problem
                    .dots
                    .iter()
                    .map(|dot| Dot {
                        x: if dot.x < fold_line_x {
                            dot.x
                        } else {
                            2 * fold_line_x - dot.x
                        },
                        y: dot.y,
                    })
                    .collect();
            }
            FoldInstruction::FoldAlongY(fold_line_y) => {
                problem.dots = problem
                    .dots
                    .iter()
                    .map(|dot| Dot {
                        x: dot.x,
                        y: if dot.y < fold_line_y {
                            dot.y
                        } else {
                            2 * fold_line_y - dot.y
                        },
                    })
                    .collect();
            }
        }

        if i == 0 {
            problem.dots.sort();
            problem.dots.dedup();
            println!(
                "[part 1] Remaining dots after one fold: {}",
                problem.dots.len()
            );
        }
    }
    
    println!("[part 2] After done folding:");
    let max_x = problem.dots.iter().map(|dot| dot.x).max().unwrap();
    let max_y = problem.dots.iter().map(|dot| dot.y).max().unwrap();
    let mut field: Vec<Vec<bool>> = vec![vec![false; (max_x + 1) as usize]; (max_y + 1) as usize];
    for dot in problem.dots {
        field[dot.y as usize][dot.x as usize] = true;
    }

    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            if field[y as usize][x as usize] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
