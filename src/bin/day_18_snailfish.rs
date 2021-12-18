/// Solution to an Advent of Code problem, day 18, 2021
/// https://adventofcode.com/2021/day/18
use std::env;
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Open,
    Close,
    Next,
    Number(u64),
}

#[derive(Debug, PartialEq, Clone)]
struct SnailfishNumber {
    tokens: Vec<Token>,
}

impl FromStr for SnailfishNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SnailfishNumber {
            tokens: s
                .chars()
                .filter_map(|c| match c {
                    '[' => Some(Token::Open),
                    ']' => Some(Token::Close),
                    ',' => Some(Token::Next),
                    n => Some(Token::Number(n.to_digit(10)? as u64)),
                })
                .collect(),
        })
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for token in self.tokens.iter() {
            match token {
                Token::Open => write!(f, "[")?,
                Token::Close => write!(f, "]")?,
                Token::Next => write!(f, ",")?,
                Token::Number(n) => write!(f, "{}", n)?,
            };
        }

        Ok(())
    }
}

impl SnailfishNumber {
    fn magnitude(&self) -> u64 {
        self.tokens
            .iter()
            .fold((0, 1), |(sum, factor), token| match token {
                Token::Open => (sum, factor * 3),
                Token::Next => (sum, factor / 3 * 2),
                Token::Close => (sum, factor / 2),
                Token::Number(n) => (sum + factor * n, factor),
            })
            .0
    }

    fn reduce(&mut self) -> &mut Self {
        loop {
            // explode?
            if let Some(exploding_pair_start) = self
                .tokens
                .iter()
                .scan(0, |nesting_level, &token| {
                    match token {
                        Token::Open => *nesting_level += 1,
                        Token::Close => *nesting_level -= 1,
                        _ => {}
                    };
                    Some(*nesting_level)
                })
                .position(|nesting_level| nesting_level == 5)
            {
                let exploding_pair_end = exploding_pair_start + 4; // Number, Next, Number, Close
                if let (Token::Number(left_number), Token::Number(right_number)) = (
                    self.tokens[exploding_pair_start + 1],
                    self.tokens[exploding_pair_end - 1],
                ) {
                    for token in self.tokens[..exploding_pair_start].iter_mut().rev() {
                        if let Token::Number(first_number_to_the_left) = token {
                            *first_number_to_the_left += left_number;
                            break;
                        }
                    }
                    for token in self.tokens[exploding_pair_end..].iter_mut() {
                        if let Token::Number(first_number_to_the_right) = token {
                            *first_number_to_the_right += right_number;
                            break;
                        }
                    }
                    self.tokens.splice(
                        exploding_pair_start..exploding_pair_end + 1,
                        std::iter::once(Token::Number(0)),
                    );
                }

                continue;
            }

            // split?
            if let Some((number_to_split_index, Token::Number(number_to_split))) = self
                .tokens
                .iter()
                .enumerate()
                .find(|(_, &token)| matches!(token, Token::Number(n) if n >= 10))
            {
                let l = number_to_split / 2;
                let r = number_to_split - l;
                self.tokens.splice(
                    number_to_split_index..number_to_split_index + 1,
                    [
                        Token::Open,
                        Token::Number(l),
                        Token::Next,
                        Token::Number(r),
                        Token::Close,
                    ],
                );
                continue;
            }

            break;
        }
        self
    }

    fn add(&self, other: &SnailfishNumber) -> Self {
        let mut new_tokens = vec![Token::Open];
        new_tokens.extend(&self.tokens);
        new_tokens.push(Token::Next);
        new_tokens.extend(&other.tokens);
        new_tokens.push(Token::Close);

        let mut new_number = SnailfishNumber { tokens: new_tokens };
        new_number.reduce();
        new_number
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let numbers = contents
        .lines()
        .map(|line| SnailfishNumber::from_str(line).unwrap())
        .collect::<Vec<SnailfishNumber>>();

    let sum = numbers
        .iter()
        .cloned()
        .reduce(|left, right| left.add(&right))
        .unwrap();

    println!("[part 1] magnitude of the sum: {}", sum.magnitude());

    let max_magnitude_of_pair_of_added_numbers = (0..numbers.len())
        .flat_map(|i| (0..numbers.len()).map(move |j| (i, j)))
        .filter(|(i, j)| i != j)
        .map(|(i, j)| numbers[i].add(&numbers[j]).magnitude())
        .max()
        .unwrap();
    println!(
        "[part 2]: max magnitude of a pair of added numbers: {}",
        max_magnitude_of_pair_of_added_numbers
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            SnailfishNumber {
                tokens: vec![
                    Token::Open,
                    Token::Number(1),
                    Token::Next,
                    Token::Number(2),
                    Token::Close
                ]
            },
            SnailfishNumber::from_str("[1,2]").unwrap()
        );
        assert_eq!(
            SnailfishNumber {
                tokens: vec![
                    Token::Open,
                    Token::Open,
                    Token::Open,
                    Token::Open,
                    Token::Number(1),
                    Token::Next,
                    Token::Number(2),
                    Token::Close,
                    Token::Next,
                    Token::Open,
                    Token::Number(3),
                    Token::Next,
                    Token::Number(4),
                    Token::Close,
                    Token::Close,
                    Token::Next,
                    Token::Open,
                    Token::Open,
                    Token::Number(5),
                    Token::Next,
                    Token::Number(6),
                    Token::Close,
                    Token::Next,
                    Token::Open,
                    Token::Number(7),
                    Token::Next,
                    Token::Number(8),
                    Token::Close,
                    Token::Close,
                    Token::Close,
                    Token::Next,
                    Token::Number(9),
                    Token::Close
                ]
            },
            SnailfishNumber::from_str("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]").unwrap()
        );
    }

    #[test]
    fn fmt() {
        assert_eq!(
            "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
            SnailfishNumber {
                tokens: vec![
                    Token::Open,
                    Token::Open,
                    Token::Open,
                    Token::Open,
                    Token::Number(1),
                    Token::Next,
                    Token::Number(2),
                    Token::Close,
                    Token::Next,
                    Token::Open,
                    Token::Number(3),
                    Token::Next,
                    Token::Number(4),
                    Token::Close,
                    Token::Close,
                    Token::Next,
                    Token::Open,
                    Token::Open,
                    Token::Number(5),
                    Token::Next,
                    Token::Number(6),
                    Token::Close,
                    Token::Next,
                    Token::Open,
                    Token::Number(7),
                    Token::Next,
                    Token::Number(8),
                    Token::Close,
                    Token::Close,
                    Token::Close,
                    Token::Next,
                    Token::Number(9),
                    Token::Close
                ]
            }
            .to_string(),
        );
    }

    #[test]
    fn magnitude() {
        assert_eq!(29, SnailfishNumber::from_str("[9,1]").unwrap().magnitude());
        assert_eq!(
            129,
            SnailfishNumber::from_str("[[9,1],[1,9]]")
                .unwrap()
                .magnitude()
        );
        assert_eq!(
            143,
            SnailfishNumber::from_str("[[1,2],[[3,4],5]]")
                .unwrap()
                .magnitude()
        );
        assert_eq!(
            1384,
            SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
                .unwrap()
                .magnitude()
        );
        assert_eq!(
            3488,
            SnailfishNumber::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .unwrap()
                .magnitude()
        );
    }

    #[test]
    fn reduce() {
        assert_eq!(
            "[[[[0,9],2],3],4]",
            SnailfishNumber::from_str("[[[[[9,8],1],2],3],4]")
                .unwrap()
                .reduce()
                .to_string()
        );
        assert_eq!(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            SnailfishNumber::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
                .unwrap()
                .reduce()
                .to_string()
        );
    }

    #[test]
    fn add() {
        assert_eq!(
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            SnailfishNumber::from_str("[1,1]")
                .unwrap()
                .add(&SnailfishNumber::from_str("[2,2]").unwrap())
                .add(&SnailfishNumber::from_str("[3,3]").unwrap())
                .add(&SnailfishNumber::from_str("[4,4]").unwrap())
                .to_string()
        );
        assert_eq!(
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
            SnailfishNumber::from_str("[1,1]")
                .unwrap()
                .add(&SnailfishNumber::from_str("[2,2]").unwrap())
                .add(&SnailfishNumber::from_str("[3,3]").unwrap())
                .add(&SnailfishNumber::from_str("[4,4]").unwrap())
                .add(&SnailfishNumber::from_str("[5,5]").unwrap())
                .add(&SnailfishNumber::from_str("[6,6]").unwrap())
                .to_string()
        );
    }
}
