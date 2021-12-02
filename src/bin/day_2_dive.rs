use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

struct Move {
	dx: i64,
	ddepth: i64,
}

impl FromStr for Move {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let command_and_arg: Vec<&str> = s.split(" ").collect();
		let command = command_and_arg[0];
		let arg: i64 = command_and_arg[1].parse()?;
		match command {
			"up" => Ok(Move {
				dx: 0,
				ddepth: -arg,
			}),
			"down" => Ok(Move { dx: 0, ddepth: arg }),
			"forward" => Ok(Move { dx: arg, ddepth: 0 }),
			_ => Ok(Move { dx: 0, ddepth: 0 }),
		}
	}
}

struct Position {
	x: i64,
	depth: i64,
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let contents = fs::read_to_string(filename).expect("Cannot read file");
	let commands = contents.lines().map(|l| l.parse::<Move>().unwrap());
	let resulting_position =
		commands.fold(Position { x: 0, depth: 0 }, |position, move_| Position {
			x: position.x + move_.dx,
			depth: position.depth + move_.ddepth,
		});

	println!(
		"x * depth = {}",
		resulting_position.x * resulting_position.depth
	);
}
