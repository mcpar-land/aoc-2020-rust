use std::collections::HashMap;

pub mod util;
pub mod days {
	pub mod day01;
	pub mod day02;
	pub mod day03;
	pub mod day04;
	pub mod day05;
	pub mod day06;
	pub mod day07;
	pub mod day08;
	pub mod day09;
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let mut funcs: HashMap<u16, fn(args: Vec<String>) -> ()> = HashMap::new();

	funcs.insert(1, crate::days::day01::day01);
	funcs.insert(2, crate::days::day02::day02);
	funcs.insert(3, crate::days::day03::day03);
	funcs.insert(4, crate::days::day04::day04);
	funcs.insert(5, crate::days::day05::day05);
	funcs.insert(6, crate::days::day06::day06);
	funcs.insert(7, crate::days::day07::day07);
	funcs.insert(8, crate::days::day08::day08);
	funcs.insert(9, crate::days::day09::day09);

	let day_picked: u16 = args.get(1).unwrap().parse().unwrap();
	println!("Running Advent of Code: Day {}", day_picked);
	funcs.get(&day_picked).unwrap()(args);
}
