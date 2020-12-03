use std::collections::HashMap;

pub mod util;
pub mod days {
	pub mod day1;
	pub mod day2;
	pub mod day3;
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let mut funcs: HashMap<u16, fn(args: Vec<String>) -> ()> = HashMap::new();

	funcs.insert(1, crate::days::day1::day1);
	funcs.insert(2, crate::days::day2::day2);
	funcs.insert(3, crate::days::day3::day3);

	let day_picked: u16 = args.get(1).unwrap().parse().unwrap();
	println!("Running Advent of Code: Day {}", day_picked);
	funcs.get(&day_picked).unwrap()(args);
}
