/// Solution to an Advent of Code problem, day 4, 2021
/// https://adventofcode.com/2021/day/4
use std::convert::TryInto;
use std::env;
use std::fs;

#[derive(Debug)]
struct BingoBoard {
    board: [[u64; 5]; 5],
    marks: [[bool; 5]; 5],
    has_won: bool,
}

impl BingoBoard {
    fn parse<'a, I: std::iter::Iterator<Item = &'a &'a str>>(lines: I) -> BingoBoard {
        BingoBoard {
            board: lines
                .map(|line| {
                    line.split(' ')
                        .filter(|v| !v.is_empty())
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                        .as_slice()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[u64; 5]>>()
                .as_slice()
                .try_into()
                .unwrap(),
            marks: Default::default(),
            has_won: false,
        }
    }

    fn mark(&mut self, drawn_number: u64) {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == drawn_number {
                    self.marks[i][j] = true;
                }
            }
        }

        self.has_won = self._calculate_has_won();
    }

    fn _calculate_has_won(&self) -> bool {
        // by row
        for i in 0..5 {
            let mut all_in_row_marked = true;
            for j in 0..5 {
                all_in_row_marked = all_in_row_marked && self.marks[i][j];
            }
            if all_in_row_marked {
                return true;
            }
        }

        // by column
        for j in 0..5 {
            let mut all_in_column_marked = true;
            for i in 0..5 {
                all_in_column_marked = all_in_column_marked && self.marks[i][j];
            }
            if all_in_column_marked {
                return true;
            }
        }

        false
    }

    fn has_won(&self) -> bool {
        self.has_won
    }

    fn sum_of_unmarked_numbers(&self) -> u64 {
        let mut sum: u64 = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marks[i][j] {
                    sum += self.board[i][j];
                }
            }
        }
        sum
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let mut lines = contents.lines();

    let drawn_numbers: Vec<u64> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<u64>().unwrap())
        .collect();

    let mut bingo_boards = lines
        .collect::<Vec<&str>>()
        .chunks_exact(6)
        .map(|v| BingoBoard::parse(v.iter().skip(1)))
        .enumerate()
        .collect::<Vec<(usize, BingoBoard)>>();

    println!("Bingo results:");
    for (round, drawn_number) in drawn_numbers.iter().enumerate() {
        bingo_boards
            .iter_mut()
            .for_each(|(_i, b)| b.mark(*drawn_number));

        let boards_won_in_this_round = bingo_boards.iter().filter(|(_i, b)| b.has_won());
        for (i, b) in boards_won_in_this_round {
            println!(
                "Round: {}, Board: {}, Score: {}",
                round + 1,
                *i + 1,
                b.sum_of_unmarked_numbers() * drawn_number
            );
        }
        bingo_boards.retain(|(_i, b)| !b.has_won());
    }
}
