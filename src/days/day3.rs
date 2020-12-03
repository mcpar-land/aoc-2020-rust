pub fn day3(_: Vec<String>) {
	let input_raw = crate::util::read_input("day3.txt");
	let field = TreeField::new(&input_raw);
	let slopes: Vec<(usize, usize)> =
		vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
	let counts = slopes
		.iter()
		.map(|(x, y)| field.trees_for_slope(*x, *y))
		.collect::<Vec<u32>>();

	for ((x, y), count) in slopes.iter().zip(counts.iter()) {
		println!("trees in slope ({}, {}): {}", x, y, count);
	}

	let mul_result = counts.iter().fold(1, |sum, val| sum * val);
	println!("multiplicative result: {}", mul_result);
}

struct TreeField(Vec<Vec<bool>>);

impl TreeField {
	pub fn new(input: &str) -> Self {
		let mut data: Vec<Vec<bool>> = vec![vec![]];
		for c in input.chars() {
			match c {
				'.' => {
					data.last_mut().unwrap().push(false);
				}
				'#' => {
					data.last_mut().unwrap().push(true);
				}
				'\n' => {
					data.push(vec![]);
				}
				_ => {}
			}
		}
		Self(data)
	}
	pub fn get(&self, x: usize, y: usize) -> Option<bool> {
		self
			.0
			.get(y)
			.map(|row| row.get(x % row.len()))
			.flatten()
			.copied()
	}
	pub fn trees_for_slope(&self, x: usize, y: usize) -> u32 {
		let mut tree_count: u32 = 0;
		let mut x_pos: usize = x;
		let mut y_pos: usize = y;
		loop {
			if let Some(tree) = self.get(x_pos, y_pos) {
				if tree {
					tree_count += 1;
				}
				x_pos += x;
				y_pos += y;
			} else {
				break;
			}
		}
		tree_count
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_tree_field() {
		let field = TreeField::new("###...###...###...\n.#.#.#.#.#.#.#.#.#");
		assert_eq!(field.get(0, 0), Some(true));
		assert_eq!(field.get(1, 0), Some(true));
		assert_eq!(field.get(2, 0), Some(true));
		assert_eq!(field.get(3, 0), Some(false));
		assert_eq!(field.get(17, 0), Some(false));
		assert_eq!(field.get(18, 0), Some(true));
	}
}
