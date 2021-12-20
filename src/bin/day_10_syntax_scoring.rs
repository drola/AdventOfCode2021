/// Solution to an Advent of Code problem, day 10, 2021
/// https://adventofcode.com/2021/day/10
use std::env;
use std::fs;

// https://en.wikipedia.org/wiki/Bracket
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(clippy::enum_variant_names)]
enum Bracket {
    LeftParenthesis,
    RightParenthesis,
    LeftChevron,
    RightChevron,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
}

#[derive(Debug)]
pub struct BracketTryFromError(());

impl TryFrom<char> for Bracket {
    type Error = BracketTryFromError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Bracket::LeftParenthesis),
            '[' => Ok(Bracket::LeftBracket),
            '{' => Ok(Bracket::LeftBrace),
            '<' => Ok(Bracket::LeftChevron),
            ')' => Ok(Bracket::RightParenthesis),
            ']' => Ok(Bracket::RightBracket),
            '}' => Ok(Bracket::RightBrace),
            '>' => Ok(Bracket::RightChevron),
            _ => Err(BracketTryFromError(())),
        }
    }
}

impl Bracket {
    fn is_opening(&self) -> bool {
        matches!(self, Bracket::LeftParenthesis
            | Bracket::LeftChevron
            | Bracket::LeftBracket
            | Bracket::LeftBrace)
    }

    fn complement(&self) -> Bracket {
        match self {
            Bracket::LeftParenthesis => Bracket::RightParenthesis,
            Bracket::RightParenthesis => Bracket::LeftParenthesis,
            Bracket::LeftChevron => Bracket::RightChevron,
            Bracket::RightChevron => Bracket::LeftChevron,
            Bracket::LeftBracket => Bracket::RightBracket,
            Bracket::RightBracket => Bracket::LeftBracket,
            Bracket::LeftBrace => Bracket::RightBrace,
            Bracket::RightBrace => Bracket::LeftBrace,
        }
    }
}

fn wrong_closing_score(b: Bracket) -> Option<u64> {
    match b {
        Bracket::RightParenthesis => Some(3),
        Bracket::RightBracket => Some(57),
        Bracket::RightBrace => Some(1197),
        Bracket::RightChevron => Some(25137),
        _ => None,
    }
}

fn part_1_syntax_error_score_if_corrupted(line: &[Bracket]) -> Option<u64> {
    let mut stack: Vec<Bracket> = Vec::with_capacity(line.len());
    for &b in line {
        if b.is_opening() {
            stack.push(b);
        } else if stack.pop().unwrap().complement() != b {
            return wrong_closing_score(b);
        }
    }
    None
}

fn part_2_completion_string_if_incomplete(line: &[Bracket]) -> Option<Vec<Bracket>> {
    let mut stack: Vec<Bracket> = Vec::with_capacity(line.len());
    for &b in line {
        if b.is_opening() {
            stack.push(b);
        } else if stack.pop().unwrap().complement() != b {
            return None;
        }
    }

    match stack.len() {
        0 => None,
        _ => Some(
            stack
                .into_iter()
                .rev()
                .map(|b| b.complement())
                .collect::<Vec<Bracket>>(),
        ),
    }
}

fn completion_string_score(b: Vec<Bracket>) -> u64 {
    b.into_iter().fold(0, |prev_score, b| {
        prev_score * 5
            + match b {
                Bracket::RightParenthesis => 1,
                Bracket::RightBracket => 2,
                Bracket::RightBrace => 3,
                Bracket::RightChevron => 4,
                _ => 0,
            }
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Bracket::try_from(c).unwrap())
                .collect::<Vec<Bracket>>()
        })
        .collect::<Vec<Vec<Bracket>>>();

    println!(
        "Sum of syntax error scores (corrupted lines): {}",
        lines
            .iter()
            .map(|line| part_1_syntax_error_score_if_corrupted(line))
            .flatten()
            .sum::<u64>()
    );

    let mut scores_for_incomplete_strings = lines
        .iter()
        .map(|line| part_2_completion_string_if_incomplete(line))
        .flatten()
        .map(completion_string_score)
        .collect::<Vec<u64>>();
    scores_for_incomplete_strings.sort_unstable();
    println!(
        "Middle score of the incomplete strings: {}",
        scores_for_incomplete_strings[scores_for_incomplete_strings.len() / 2]
    );
}
