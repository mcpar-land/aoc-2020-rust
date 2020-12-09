pub fn day9(_: Vec<String>) {
	let data =
		XmasData::from_str(crate::util::read_input("day9.txt").as_str(), 25);

	println!("{:?}", data.cipher_values(0));
	println!("First invalid value: {:?}", data.invalids()[0].1);
	println!("Weakness sum range: {:?}", data.weakness_sum());
	println!("weakness: {}", data.weakness());
}

pub struct XmasData {
	values: Vec<u128>,
	sample: usize,
}

impl XmasData {
	pub fn from_str(input: &str, size: usize) -> Self {
		Self::new(
			input
				.split("\n")
				.map(|i| i.parse::<u128>().unwrap())
				.collect::<Vec<u128>>(),
			size,
		)
	}

	pub fn new(values: Vec<u128>, size: usize) -> Self {
		Self {
			values,
			sample: size,
		}
	}

	pub fn cipher_values(&self, i: usize) -> &[u128] {
		&self.values[i..i + self.sample]
	}

	pub fn valid(&self, index: usize) -> bool {
		let val = self[index];
		let cvals = self.cipher_values(index);
		for i in 0..cvals.len() {
			for j in i..cvals.len() {
				if cvals[i] + cvals[j] == val {
					return true;
				}
			}
		}
		return false;
	}

	pub fn invalids(&self) -> Vec<(usize, u128)> {
		self
			.values
			.iter()
			.skip(self.sample)
			.enumerate()
			.filter(|(i, _)| !self.valid(*i))
			.map(|(i, v)| (i, *v))
			.collect()
	}

	pub fn values_len(&self) -> usize {
		self.values.len() - self.sample
	}

	pub fn weakness_sum(&self) -> &[u128] {
		let (invalid_index, invalid_value) = self.invalids()[0];

		for len in 2..invalid_index {
			for i in 0..self.values_len() - len {
				let sum_sample = &self[i..i + len];
				let sum = sum_sample.iter().fold(0, |acc, v| acc + v);
				if sum == invalid_value {
					return sum_sample;
				}
			}
		}
		panic!();
	}

	pub fn weakness(&self) -> u128 {
		let weakness_sum = self.weakness_sum();
		*weakness_sum.iter().min().unwrap() + *weakness_sum.iter().max().unwrap()
	}
}

impl std::ops::Index<usize> for XmasData {
	type Output = u128;

	fn index(&self, index: usize) -> &Self::Output {
		&self.values[index + self.sample]
	}
}

impl std::ops::Index<std::ops::Range<usize>> for XmasData {
	type Output = [u128];

	fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
		&self.values[index.start + self.sample..index.end + self.sample]
	}
}
