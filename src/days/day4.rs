use lazy_static::lazy_static;
use nom::character::complete::digit1;
use regex::Regex;

pub fn day4(_: Vec<String>) {
	let input_raw = crate::util::read_input("day4.txt");
	let passports = input_raw
		.split("\n\n")
		.map(|input| Passport::from_str(input))
		.collect::<Vec<Passport>>();

	println!("Processed {} passports", passports.len());

	let valid_count =
		passports.iter().fold(
			0u32,
			|acc, passport| {
				if passport.valid() {
					acc + 1
				} else {
					acc
				}
			},
		);

	println!("There are {} valid passports", valid_count);
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Passport {
	birth_year: Option<u32>,
	issue_year: Option<u32>,
	expiration_year: Option<u32>,
	height: Option<String>,
	hair_color: Option<String>,
	eye_color: Option<String>,
	passport_id: Option<String>,
	country_id: Option<u64>,
}

impl Passport {
	pub fn valid(&self) -> bool {
		lazy_static! {
			static ref HAIR_REGEX: Regex = Regex::new(r"^#[a-fA-F0-9]{6}$").unwrap();
			static ref EYE_REGEX: Regex =
				Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
			static ref PID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
		}

		let mut valid = true;

		valid &= self.birth_year.map_or(false, |y| y >= 1920 && y <= 2002);

		valid &= self.issue_year.map_or(false, |y| y >= 2010 && y <= 2020);

		valid &= self
			.expiration_year
			.map_or(false, |y| y >= 2020 && y <= 2030);

		valid &= self.height.as_ref().map_or(false, |h| {
			let (v, h) = digit1::<&str, ()>(h.as_str()).unwrap_or_default();
			let h: u32 = h.parse().unwrap();
			(v == "cm" && h >= 150 && h <= 193) || (v == "in" && h >= 59 && h <= 76)
		});

		valid &= self
			.hair_color
			.as_ref()
			.map_or(false, |c| HAIR_REGEX.is_match(&c));

		valid &= self
			.eye_color
			.as_ref()
			.map_or(false, |c| EYE_REGEX.is_match(&c));

		valid &= self
			.passport_id
			.as_ref()
			.map_or(false, |c| PID_REGEX.is_match(&c));

		valid
	}
	pub fn from_str(input: &str) -> Self {
		let mut passport = Self::default();
		for field in input.split_whitespace() {
			let (k, v) = {
				let mut split = field.split(":");
				(split.next().unwrap(), split.next().unwrap())
			};
			match k {
				"byr" => passport.birth_year = Some(v.parse().unwrap()),
				"iyr" => passport.issue_year = Some(v.parse().unwrap()),
				"eyr" => passport.expiration_year = Some(v.parse().unwrap()),
				"hgt" => passport.height = Some(v.parse().unwrap()),
				"hcl" => passport.hair_color = Some(v.parse().unwrap()),
				"ecl" => passport.eye_color = Some(v.parse().unwrap()),
				"pid" => passport.passport_id = Some(v.parse().unwrap()),
				"cid" => passport.country_id = Some(v.parse().unwrap()),
				_ => unreachable!(),
			}
		}
		passport
	}
}

impl Default for Passport {
	fn default() -> Self {
		Self {
			birth_year: None,
			issue_year: None,
			expiration_year: None,
			height: None,
			hair_color: None,
			eye_color: None,
			passport_id: None,
			country_id: None,
		}
	}
}
