use indicatif::ProgressBar;

pub fn day01(_: Vec<String>) -> () {
	println!("Day 1!");
	let input_raw: String = crate::util::read_input("day01.txt");
	let input: Vec<u32> = input_raw
		.split_whitespace()
		.map(|n| n.parse::<u32>().unwrap())
		.collect::<Vec<u32>>();

	let spinner_one = ProgressBar::new_spinner();
	'one: for i in 0..input.len() {
		for j in i..input.len() {
			let a = input[i];
			let b = input[j];
			let res = a + b;
			spinner_one.set_message(&format!("{} + {} = {}", a, b, res));
			spinner_one.tick();
			if res == 2020 {
				spinner_one.finish();
				println!("Result found! {} * {} = {}", a, b, a * b);
				break 'one;
			}
		}
	}
	let spinner_two = ProgressBar::new_spinner();
	'two: for i in 0..input.len() {
		for j in i..input.len() {
			for k in j..input.len() {
				let a = input[i];
				let b = input[j];
				let c = input[k];
				let res = a + b + c;
				spinner_two.set_message(&format!("{} + {} + {} = {}", a, b, c, res));
				if res == 2020 {
					spinner_two.finish();
					println!("Result found! {} * {} * {} = {}", a, b, c, a * b * c);
					break 'two;
				}
			}
		}
	}
}
