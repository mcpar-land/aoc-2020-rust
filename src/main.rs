use std::collections::HashMap;

pub mod util;
pub mod days {
	pub mod day1;
	pub mod day2;
	pub mod day3;
	pub mod day4;
	pub mod day5;
	pub mod day6;
	pub mod day7;
	pub mod day8;
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let mut funcs: HashMap<u16, fn(args: Vec<String>) -> ()> = HashMap::new();

	funcs.insert(1, crate::days::day1::day1);
	funcs.insert(2, crate::days::day2::day2);
	funcs.insert(3, crate::days::day3::day3);
	funcs.insert(4, crate::days::day4::day4);
	funcs.insert(5, crate::days::day5::day5);
	funcs.insert(6, crate::days::day6::day6);
	funcs.insert(7, crate::days::day7::day7);
	funcs.insert(8, crate::days::day8::day8);

	let day_picked: u16 = args.get(1).unwrap().parse().unwrap();
	println!("Running Advent of Code: Day {}", day_picked);
	funcs.get(&day_picked).unwrap()(args);
}
