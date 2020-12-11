use std::collections::HashSet;

pub fn day06(_: Vec<String>) {
	let input_raw = crate::util::read_input("day06.txt");
	let groups = input_raw
		.split("\n\n")
		.map(|input| input.into())
		.collect::<Vec<Group>>();

	println!("Processed {} groups", groups.len());

	let sum_total = groups.iter().fold(0, |acc, g| acc + g.answers().len());

	println!("Answers anyone answered yes to: {}", sum_total);

	let sum_total_all =
		groups.iter().fold(0, |acc, g| acc + g.all_answers().len());

	println!("Answers everyone answered yes to: {}", sum_total_all);
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Group(pub Vec<Person>);
impl From<&str> for Group {
	fn from(input: &str) -> Self {
		Self(input.split("\n").map(|p| p.into()).collect::<Vec<Person>>())
	}
}
impl Group {
	pub fn len(&self) -> usize {
		self.0.iter().fold(0, |acc, person| acc + person.len())
	}
	pub fn answers(&self) -> HashSet<Question> {
		let mut answers: HashSet<Question> = HashSet::new();
		for person in &self.0 {
			for question in &person.0 {
				answers.insert(*question);
			}
		}
		answers
	}
	pub fn all_answers(&self) -> HashSet<Question> {
		let mut all_answers = self.answers();
		for person in &self.0 {
			for answer in all_answers.clone() {
				if !person.0.contains(&answer) {
					all_answers.remove(&answer);
				}
			}
		}
		all_answers
	}
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Person(pub Vec<Question>);
impl From<&str> for Person {
	fn from(input: &str) -> Self {
		Self(input.chars().map(|c| c.into()).collect::<Vec<Question>>())
	}
}
impl Person {
	pub fn len(&self) -> usize {
		self.0.len()
	}
	pub fn contains(&self, q: Question) -> bool {
		self.0.contains(&q)
	}
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Question {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
}
impl From<char> for Question {
	fn from(input: char) -> Self {
		use Question::*;
		match input {
			'a' => A,
			'b' => B,
			'c' => C,
			'd' => D,
			'e' => E,
			'f' => F,
			'g' => G,
			'h' => H,
			'i' => I,
			'j' => J,
			'k' => K,
			'l' => L,
			'm' => M,
			'n' => N,
			'o' => O,
			'p' => P,
			'q' => Q,
			'r' => R,
			's' => S,
			't' => T,
			'u' => U,
			'v' => V,
			'w' => W,
			'x' => X,
			'y' => Y,
			'z' => Z,
			_ => unreachable!(),
		}
	}
}
