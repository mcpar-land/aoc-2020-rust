pub fn day10(_: Vec<String>) {
	let input_raw = crate::util::read_input("day10.txt");
	let mut input: Vec<u32> =
		input_raw.split("\n").map(|i| i.parse().unwrap()).collect();

	input.sort();

	let mut diffs: [usize; 3] = [1, 0, 1];

	for i in 0..input.len() - 1 {
		diffs[(input[i + 1] - input[i] - 1) as usize] += 1;
	}

	println!("{:?}", input);
	println!(
		"{:?}, {} * {} = {}",
		diffs,
		diffs[0],
		diffs[2],
		diffs[0] * diffs[2]
	);

	// no clue on pulling off the second half of day 10. empty for now...
}

pub fn possible_arrangements(input: Vec<u32>) -> u32 {
	let mut input = input;
	input.sort();
	println!("Sorted {:?}", input);
	input.insert(0, 0);
	input.push(input[input.len() - 1] + 3);
	let mut jumps = vec![];
	let mut i = 0;
	while i < input.len() {
		let val = input[i];
		let mut jump = 0;
		if i != input.len() - 1 {
			for j in i + 1..input.len() {
				if input[j] - val <= 3 {
					jump += 1;
					i += 1;
				} else {
					continue;
				}
			}
		}
		jumps.push(jump);
		i += 1;
	}
	jumps = jumps
		.iter()
		.filter(|v| **v > 0)
		.map(|v| if *v == 3 { 4 } else { *v })
		.collect();
	println!("jumps: {:?}", jumps);
	jumps.iter().fold(1, |acc, val| acc * val)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_possible_arrangements() {
		let input1 = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
		let input2 = vec![
			28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11,
			1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
		];
		assert_eq!(possible_arrangements(input1), 8);
		assert_eq!(possible_arrangements(input2), 19208);
	}
}
