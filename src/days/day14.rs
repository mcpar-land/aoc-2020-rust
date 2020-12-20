use bitvec::prelude::*;
use nom::{
	bytes::complete::tag,
	character::complete::{digit1, one_of},
	combinator::map,
	multi::{many1, separated_list1},
	IResult,
};
use std::collections::HashMap;

pub fn day14(_: Vec<String>) {
	let input = crate::util::read_input("day14.txt");

	let mut program: HashMap<usize, u64> = HashMap::new();
	let commands: Vec<DockingMask> =
		separated_list1(tag("\n"), DockingMask::parse)(input.as_str())
			.unwrap()
			.1;

	for DockingMask { mask, mem } in &commands {
		for (i, oldval) in mem {
			let mut val = *oldval;
			let oldval_bytes = val.view_bits_mut::<Msb0>();
			for (i, cmd) in mask.iter().enumerate() {
				match cmd {
					MaskCommand::None => {}
					MaskCommand::Zero => oldval_bytes.set(i, false),
					MaskCommand::One => oldval_bytes.set(i, true),
				}
			}
			program.insert(*i, val);
		}
	}
	let total = program.iter().fold(0u64, |acc, (_, v)| acc + v);
	println!("Part 1 Total: {}", total);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MaskCommand {
	None,
	Zero,
	One,
}

impl MaskCommand {
	pub fn parse(input: &str) -> IResult<&str, MaskCommand> {
		map(one_of("X01"), |v| match v {
			'X' => MaskCommand::None,
			'0' => MaskCommand::Zero,
			'1' => MaskCommand::One,
			_ => unreachable!(),
		})(input)
	}
}

#[derive(Clone, Debug)]
pub struct DockingMask {
	mask: Vec<MaskCommand>,
	mem: Vec<(usize, u64)>,
}

impl DockingMask {
	pub fn new() -> Self {
		DockingMask {
			mask: Vec::new(),
			mem: Vec::new(),
		}
	}
	pub fn parse(input: &str) -> IResult<&str, DockingMask> {
		let (input, _) = tag("mask = ")(input)?;
		let (input, mut mask_vals): (&str, Vec<MaskCommand>) =
			map(many1(MaskCommand::parse), |v| v)(input)?;
		let mut mask: Vec<MaskCommand> =
			[MaskCommand::None; 28].iter().cloned().collect();
		mask.append(&mut mask_vals);
		let (input, _) = tag("\n")(input)?;
		let (input, mem) =
			map(separated_list1(tag("\n"), Self::parse_mem_set), |v| {
				v.into_iter().collect()
			})(input)?;
		Ok((input, DockingMask { mask, mem }))
	}
	fn parse_mem_set(input: &str) -> IResult<&str, (usize, u64)> {
		let (input, _) = tag("mem[")(input)?;
		let (input, i) = map(digit1, |v: &str| v.parse().unwrap())(input)?;
		let (input, _) = tag("] = ")(input)?;
		let (input, val) = map(digit1, |v: &str| v.parse().unwrap())(input)?;
		Ok((input, (i, val)))
	}
}
