use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{alpha1, digit1, newline, one_of, space1},
	combinator::map,
	IResult,
};

pub fn day8(_: Vec<String>) {
	let input = crate::util::read_input("day8.txt");

	let mut computer = Computer::from_str(input.as_str());
	computer.run();
	println!(
		"first solution: {:#?}",
		(computer.position, computer.accumulator)
	);
	println!("Brute-force fixing the program...");

	let base_computer = Computer::from_str(input.as_str());
	for i in 0..computer.commands.len() {
		// don't run if it's an Acc
		// if !vec![Op::Jmp, Op::Nop]
		// 	.contains(&base_computer.commands.get(i).unwrap().0.op)
		// {
		// 	continue;
		// }
		// create clone of base computer
		let mut computer = base_computer.clone();

		// swap jmp and nop
		let mut command = computer.commands[i].0;
		let old_command = command;
		command.op = match command.op {
			Op::Acc => Op::Acc,
			Op::Jmp => Op::Nop,
			Op::Nop => Op::Jmp,
		};
		computer.commands[i].0 = command;

		// run
		let res = computer.run();
		println!(
			"Swapped code {:3}:\t{} => {}\t{:?} {} @ {}",
			i, old_command, command, res, computer.accumulator, computer.position,
		);
		if res == TickError::Done {
			println!("Done! end result: {}", computer.accumulator);
			break;
		}
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Computer {
	pub commands: Vec<(Instruction, Option<i32>)>,
	pub accumulator: i32,
	pub position: usize,
}

impl Computer {
	pub fn from_str(input: &str) -> Self {
		let commands = input
			.split("\n")
			.map(|input| (Instruction::from_str(input).unwrap().1, None))
			.collect::<Vec<(Instruction, Option<i32>)>>();

		Self {
			commands,
			accumulator: 0,
			position: 0,
		}
	}

	pub fn tick(&mut self) -> Result<(Instruction, i32), TickError> {
		let len = self.commands.len();
		let (Instruction { op, arg }, res) = self
			.commands
			.get_mut(self.position)
			.ok_or(TickError::OutOfBounds)?;

		if res.is_some() {
			return Err(TickError::InfiniteLoop);
		}

		match op {
			Op::Acc => {
				self.accumulator += *arg;
				self.position += 1;
			}
			Op::Nop => {
				self.position += 1;
			}
			Op::Jmp => {
				self.position = if arg.is_negative() {
					self.position - arg.abs() as u32 as usize
				} else {
					self.position + *arg as usize
				}
			}
		}
		*res = Some(self.accumulator);

		if self.position == len {
			Err(TickError::Done)
		} else {
			Ok((Instruction { op: *op, arg: *arg }, self.accumulator))
		}
	}

	pub fn run(&mut self) -> TickError {
		loop {
			if let Err(err) = self.tick() {
				return err;
			}
		}
	}
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum TickError {
	InfiniteLoop,
	OutOfBounds,
	Done,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Op {
	Acc, // accumulate
	Jmp, // jump
	Nop, // no operation
}
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Instruction {
	pub op: Op,
	pub arg: i32,
}

impl Instruction {
	pub fn from_str(input: &str) -> IResult<&str, Self> {
		let (input, op) = alpha1(input)?;
		let op = match op {
			"acc" => Op::Acc,
			"jmp" => Op::Jmp,
			"nop" => Op::Nop,
			_ => unreachable!(),
		};
		let (input, _) = space1(input)?;
		let (input, sign) = one_of("+-")(input)?;
		let sign = match sign {
			'+' => 1,
			'-' => -1,
			_ => unreachable!(),
		};
		let (input, arg) = digit1(input)?;
		let arg: i32 = arg.parse::<i32>().unwrap() * sign;

		Ok((input, Instruction { op, arg }))
	}
}

impl std::fmt::Display for Instruction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{} {}{}",
			match self.op {
				Op::Acc => "acc",
				Op::Jmp => "jmp",
				Op::Nop => "nop",
			},
			if self.arg.is_positive() { "+" } else { "" },
			self.arg
		)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_instruction_from_str() {
		assert_eq!(
			Instruction::from_str("acc -12").unwrap().1,
			Instruction {
				op: Op::Acc,
				arg: -12
			}
		);
		assert_eq!(
			Instruction::from_str("jmp +343").unwrap().1,
			Instruction {
				op: Op::Jmp,
				arg: 343
			}
		);
	}
}
