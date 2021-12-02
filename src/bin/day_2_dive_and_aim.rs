use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

enum Move {
	Up(i64),
	Down(i64),
	Forward(i64),
}

#[derive(Debug)]
struct ParseMoveError {}

impl FromStr for Move {
	type Err = ParseMoveError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let command_and_arg: Vec<&str> = s.split(" ").collect();
		let command = command_and_arg[0];
		let arg: Result<i64, ParseIntError> = command_and_arg[1].parse();
		match (command, arg) {
			("up", Ok(arg)) => Ok(Move::Up(arg)),
			("down", Ok(arg)) => Ok(Move::Down(arg)),
			("forward", Ok(arg)) => Ok(Move::Forward(arg)),
			_ => Result::Err(ParseMoveError {}),
		}
	}
}

struct Position {
	aim: i64,
	x: i64,
	depth: i64,
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let contents = fs::read_to_string(filename).expect("Cannot read file");
	let commands = contents.lines().map(|l| l.parse::<Move>().unwrap());
	let resulting_position = commands.fold(
		Position {
			aim: 0,
			x: 0,
			depth: 0,
		},
		|position, move_| match move_ {
			Move::Up(v) => Position {
				aim: position.aim - v,
				x: position.x,
				depth: position.depth,
			},
			Move::Down(v) => Position {
				aim: position.aim + v,
				x: position.x,
				depth: position.depth,
			},
			Move::Forward(v) => Position {
				aim: position.aim,
				x: position.x + v,
				depth: position.depth + v * position.aim,
			},
		},
	);

	println!(
		"x * depth = {}",
		resulting_position.x * resulting_position.depth
	);
}
