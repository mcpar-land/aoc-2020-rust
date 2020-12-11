use nom::{
	bytes::complete::tag,
	character::complete::{alpha1, digit1, newline, space1},
	multi::separated_list1,
	IResult,
};

pub fn day02(_: Vec<String>) -> () {
	let input_raw = crate::util::read_input("day02.txt");
	let (_, list) = password_list(&input_raw).unwrap();
	println!("Parsed {} passwords", list.len());
	let valid_count_one = list
		.iter()
		.filter(|p| p.is_valid_one())
		.collect::<Vec<&Password>>()
		.len();
	println!("(problem 1) Number of valid passwords: {}", valid_count_one);
	let valid_count_two = list
		.iter()
		.filter(|p| p.is_valid_two())
		.collect::<Vec<&Password>>()
		.len();
	println!("(problem 2) Number of valid passwords: {}", valid_count_two);
}

fn password_list(input: &str) -> IResult<&str, Vec<Password>> {
	separated_list1(newline, password)(input)
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Password {
	pub min: u32,
	pub max: u32,
	pub character: char,
	pub password: String,
}

impl Password {
	pub fn is_valid_one(&self) -> bool {
		let res = self
			.password
			.chars()
			.filter(|c| c == &self.character)
			.collect::<Vec<char>>()
			.len() as u32;
		res >= self.min && res <= self.max
	}
	pub fn is_valid_two(&self) -> bool {
		let char_a = self.password.chars().nth(self.min as usize - 1);
		let char_b = self.password.chars().nth(self.max as usize - 1);
		let pos_a: bool = char_a == Some(self.character);
		let pos_b: bool = char_b == Some(self.character);

		let res = (pos_a && !pos_b) || (!pos_a && pos_b);

		res
	}
}

fn password(input: &str) -> IResult<&str, Password> {
	let (input, min) = digit1(input)?;
	let (input, _) = tag("-")(input)?;
	let (input, max) = digit1(input)?;
	let (input, _) = space1(input)?;
	let (input, character) = alpha1(input)?;
	let (input, _) = tag(": ")(input)?;
	let (input, password) = alpha1(input)?;

	Ok((
		input,
		Password {
			min: min.parse().unwrap(),
			max: max.parse().unwrap(),
			character: character.parse().unwrap(),
			password: password.to_string(),
		},
	))
}
