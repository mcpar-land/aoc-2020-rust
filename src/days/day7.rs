use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{alpha1, digit1, newline, space1},
	combinator::{map, opt},
	multi::separated_list1,
	IResult,
};
use std::collections::{HashMap, HashSet};

pub fn day7(_: Vec<String>) {
	let input_raw = crate::util::read_input("day7.txt");

	let rules = RuleList::from_str(input_raw.as_str());
	println!("Processed {} rules", rules.0.len());

	println!(
		"Bags containing shiny gold bags: {}",
		rules.bags_containing(&my_bag()).len()
	);

	println!(
		"Bags contained in 1 shiny gold bag: {}",
		rules.bags_held_in(&my_bag())
	)
}

fn my_bag() -> Bag {
	Bag::new("shiny", "gold")
}

pub struct RuleList(HashMap<Bag, Vec<(u32, Bag)>>);

impl RuleList {
	pub fn new(input: &Vec<Rule>) -> Self {
		let mut rules_map = HashMap::new();
		for rule in input {
			let r = rule.clone();
			rules_map.insert(r.outer, r.inner);
		}
		Self(rules_map)
	}

	pub fn from_str(input: &str) -> Self {
		Self::new(
			&input
				.split("\n")
				.map(|input| Rule::from_str(input).unwrap().1)
				.collect::<Vec<Rule>>(),
		)
	}

	pub fn bag_contains(&self, target: &Bag, query: &Bag) -> bool {
		if let Some(rule) = self.0.get(query) {
			for (_, rulebag) in rule {
				if rulebag == target || self.bag_contains(target, rulebag) {
					return true;
				}
			}
		}
		return false;
	}

	pub fn bags_containing(&self, target: &Bag) -> Vec<Bag> {
		self
			.0
			.keys()
			.filter(|k| self.bag_contains(target, k.clone()))
			.cloned()
			.collect()
	}

	pub fn bags_held_in(&self, target: &Bag) -> u32 {
		if let Some(rule) = self.0.get(target) {
			rule.iter().fold(0, |total, (amt, bag)| {
				total + (*amt * self.bags_held_in(bag)) + *amt
			})
		} else {
			0
		}
	}
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Bag {
	pub texture: String,
	pub color: String,
}

impl Bag {
	fn new(texture: &str, color: &str) -> Self {
		Self {
			texture: texture.to_string(),
			color: color.to_string(),
		}
	}
	fn from_str(input: &str) -> IResult<&str, Self> {
		let (input, texture) = alpha1(input)?;
		let (input, _) = space1(input)?;
		let (input, color) = alpha1(input)?;
		let (input, _) = tag(" bag")(input)?;
		let (input, _) = opt(tag("s"))(input)?;
		Ok((
			input,
			Bag {
				texture: texture.to_string(),
				color: color.to_string(),
			},
		))
	}
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Rule {
	pub outer: Bag,
	pub inner: Vec<(u32, Bag)>,
}

impl Rule {
	fn from_str(input: &str) -> IResult<&str, Self> {
		let (input, outer) = Bag::from_str(input)?;
		let (input, _) = tag(" contain ")(input)?;

		fn bag_and_num(input: &str) -> IResult<&str, (u32, Bag)> {
			let (input, num) = digit1(input)?;
			let (input, _) = space1(input)?;
			let (input, bag) = Bag::from_str(input)?;
			Ok((input, (num.parse().unwrap(), bag)))
		}

		let (input, inner) = alt((
			separated_list1(tag(", "), bag_and_num),
			map(tag("no other bags"), |_| vec![]),
		))(input)?;
		let (input, _) = tag(".")(input)?;
		Ok((input, Rule { outer, inner }))
	}
	pub fn contains(&self, bag: &Bag) -> bool {
		for (_, inner_bag) in &self.inner {
			if bag == inner_bag {
				return true;
			}
		}
		return false;
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse() {
		assert_eq!(
			Rule::from_str(
				"dotted maroon bags contain 3 clear salmon bags, 1 light salmon bag."
			)
			.unwrap()
			.1,
			Rule {
				outer: Bag::new("dotted", "maroon"),
				inner: vec![
					(3, Bag::new("clear", "salmon")),
					(1, Bag::new("light", "salmon"))
				]
			}
		);
	}

	#[test]
	fn test_contains() {
		let input_raw = crate::util::read_input("day7.txt");
		let rules = RuleList::from_str(input_raw.as_str());
		assert_eq!(
			rules
				.bag_contains(&Bag::new("shiny", "gold"), &Bag::new("light", "black")),
			true
		);
		assert_eq!(
			rules
				.bag_contains(&Bag::new("shiny", "gold"), &Bag::new("pale", "olive")),
			true
		);
	}
}
