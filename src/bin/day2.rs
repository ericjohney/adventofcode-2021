use adventofcode2021::utils::{file, get_answerer, BoxResult};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
#[display("{direction} {amount}")]
struct Command {
	direction: String,
	amount: usize,
}

fn main() -> BoxResult<()> {
	let commands = file::lines::<Command>("inputs/day2.txt")?;

	let mut answer = get_answerer();
	answer(part1(&commands));
	answer(part2(&commands));

	Ok(())
}

fn part1(commands: &Vec<Command>) -> BoxResult<usize> {
	struct Position {
		horizontal: usize,
		depth: usize,
	}

	let mut pos = Position {
		horizontal: 0,
		depth: 0,
	};

	for command in commands {
		match command.direction.as_str() {
			"forward" => pos.horizontal += command.amount,
			"down" => pos.depth += command.amount,
			"up" => pos.depth -= command.amount,
			_ => (),
		};
	}
	Ok(pos.horizontal * pos.depth)
}

fn part2(commands: &Vec<Command>) -> BoxResult<usize> {
	struct Position {
		horizontal: usize,
		depth: usize,
		aim: usize,
	}

	let mut pos = Position {
		horizontal: 0,
		depth: 0,
		aim: 0,
	};

	for command in commands {
		match command.direction.as_str() {
			"forward" => {
				pos.horizontal += command.amount;
				pos.depth += pos.aim * command.amount;
			}
			"down" => pos.aim += command.amount,
			"up" => pos.aim -= command.amount,
			_ => (),
		};
	}
	Ok(pos.horizontal * pos.depth)
}
