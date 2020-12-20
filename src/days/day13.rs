use nom::{
	bytes::complete::tag,
	character::complete::{alpha1, alphanumeric1, digit1},
	combinator::map,
	multi::separated_list1,
	IResult,
};

pub fn day13(_: Vec<String>) {
	let raw_input = crate::util::read_input("day13.txt");

	let (estimate, bus_list) = list_and_estimate(raw_input.as_str()).unwrap().1;

	println!("{:?}, {:?}", estimate, bus_list);

	let (id, wait) = nearest_id(estimate, bus_list);
	println!(
		"Wait {} mins for bus id {}. ({} - {}) * {} = {}",
		wait,
		id,
		wait,
		estimate,
		id,
		(wait - estimate) * id
	);
}

/// Requires `bus_list` to be sorted
pub fn nearest_id(estimate: u32, bus_list: Vec<u32>) -> (u32, u32) {
	let mut nearest: Vec<(u32, u32)> = bus_list
		.iter()
		.map(|bus| (*bus, (estimate / *bus) * bus + bus))
		.collect();
	nearest.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
	println!("nearests: {:?}", nearest);
	*nearest.first().unwrap()
}

pub fn list_and_estimate(input: &str) -> IResult<&str, (u32, Vec<u32>)> {
	let (input, estimate) = map(digit1, |s: &str| s.parse().unwrap())(input)?;
	let (input, _) = tag("\n")(input)?;
	let (input, bus_list) = bus_list(input)?;
	Ok((input, (estimate, bus_list)))
}

pub fn bus_list(input: &str) -> IResult<&str, Vec<u32>> {
	separated_list1(tag(","), bus)(input).map(|(input, val)| {
		(input, {
			let mut s: Vec<u32> = val.iter().filter_map(|val| *val).collect();
			s.sort();
			s
		})
	})
}

pub fn bus(input: &str) -> IResult<&str, Option<u32>> {
	map(alphanumeric1, |s: &str| match s {
		"x" => None,
		s => Some(s.parse().unwrap()),
	})(input)
}
