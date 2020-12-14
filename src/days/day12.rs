use nom::{
	character::complete::{alpha1, digit1},
	combinator::map,
	IResult,
};
use std::fmt;

pub fn day12(_: Vec<String>) {
	let input_raw = crate::util::read_input("day12.txt");
	let instructions = input_raw
		.split("\n")
		.map(|input| Instruction::from_str(input).unwrap().1)
		.collect::<Vec<Instruction>>();

	let mut ship = Ship::new();
	for ins in &instructions {
		ship.do_instruction(&ins);
	}
	println!("Part 1 - Manhattan distance: {}", ship.manhattan_distance());

	let mut ship2 = WaypointShip::new();
	for ins in &instructions {
		ship2.do_instruction(&ins);
	}
	println!(
		"Part 2 - Manhattan distance: {}",
		ship2.manhattan_distance()
	);
}

pub struct Ship {
	pub x: i32,
	pub y: i32,
	pub rot: i32,
}

impl Ship {
	pub fn new() -> Self {
		Self {
			x: 0,
			y: 0,
			rot: 90,
		}
	}
	pub fn add_rot(&mut self, val: i32) {
		self.rot += val;
		while self.rot < 0 {
			self.rot += 360;
		}
		self.rot = self.rot % 360;
	}
	pub fn do_instruction(&mut self, ins: &Instruction) {
		let v = ins.value as i32;
		match ins.action {
			Action::North => self.y += v,
			Action::South => self.y -= v,
			Action::East => self.x += v,
			Action::West => self.x -= v,
			Action::Left => self.add_rot(v * -1),
			Action::Right => self.add_rot(v),
			Action::Forward => match self.rot {
				0 => self.y += v,
				90 => self.x += v,
				180 => self.y -= v,
				270 => self.x -= v,
				_ => unreachable!(),
			},
		}
	}
	pub fn manhattan_distance(&self) -> i32 {
		(self.x.abs() + self.y.abs()) as i32
	}
}

pub struct WaypointShip {
	pub x: i32,
	pub y: i32,
	pub wx: i32,
	pub wy: i32,
}

impl WaypointShip {
	pub fn new() -> Self {
		Self {
			x: 0,
			y: 0,
			wx: 10,
			wy: 1,
		}
	}
	pub fn add_rot(&mut self, val: i32) {
		let val = val as f32;
		let t_cos = val.to_radians().cos();
		let t_sin = val.to_radians().sin();
		let (x, y) = (self.wx as f32, self.wy as f32);
		self.wx = (x * t_cos - y * t_sin).round() as i32;
		self.wy = (x * t_sin + y * t_cos).round() as i32;
	}

	pub fn do_instruction(&mut self, ins: &Instruction) {
		let v = ins.value as i32;
		match ins.action {
			Action::North => self.wy += v,
			Action::South => self.wy -= v,
			Action::East => self.wx += v,
			Action::West => self.wx -= v,
			Action::Left => self.add_rot(v),
			Action::Right => self.add_rot(v * -1),
			Action::Forward => {
				self.x += self.wx * v;
				self.y += self.wy * v;
			}
		}
		println!(
			"{:?} pos: {}n {}e, waypoint: {}n {}e",
			ins, self.y, self.x, self.wy, self.wx
		);
	}
	pub fn manhattan_distance(&self) -> i32 {
		(self.x.abs() + self.y.abs()) as i32
	}
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Action {
	North,
	South,
	East,
	West,
	Left,
	Right,
	Forward,
}

impl From<&str> for Action {
	fn from(input: &str) -> Self {
		match input {
			"N" => Self::North,
			"S" => Self::South,
			"E" => Self::East,
			"W" => Self::West,
			"L" => Self::Left,
			"R" => Self::Right,
			"F" => Self::Forward,
			_ => unreachable!(),
		}
	}
}

impl fmt::Display for Action {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Action::North => "N",
				Action::South => "S",
				Action::East => "E",
				Action::West => "W",
				Action::Left => "L",
				Action::Right => "R",
				Action::Forward => "F",
			}
		)
	}
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Instruction {
	action: Action,
	value: u32,
}

impl Instruction {
	pub fn from_str(input: &str) -> IResult<&str, Self> {
		let (input, action) = map(alpha1, |s: &str| Action::from(s))(input)?;
		let (input, value) =
			map(digit1, |s: &str| s.parse::<u32>().unwrap())(input)?;
		Ok((input, Self { action, value }))
	}
}
