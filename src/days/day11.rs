use std::collections::HashSet;

pub fn day11(_: Vec<String>) {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ship(Vec<Vec<Pos>>);

impl Ship {
	pub fn new() -> Self {
		Self(vec![])
	}
	pub fn from_str(input: &str) -> Self {
		let mut ship = Self::new();
		let lines = input.split("\n").collect::<Vec<&str>>();
		let w = lines[0].len();
		let h = lines.len();
		for i in 0..h {
			ship.0.push(vec![]);
			for j in 0..w {
				ship.0[i].push(Pos::from_char(lines[i].chars().nth(j).unwrap()))
			}
		}
		ship
	}
	pub fn get(&self, (x, y): (usize, usize)) -> Option<&Pos> {
		self.0.get(y).map(|row| row.get(x)).flatten()
	}
	pub fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Pos> {
		self.0.get_mut(y).map(|row| row.get_mut(x)).flatten()
	}
	pub fn size(&self) -> (usize, usize) {
		(self.0[0].len(), self.0.len())
	}

	pub fn equal(&self, other: &Self) -> bool {
		let (w, h) = self.size();
		for y in 0..h {
			for x in 0..w {
				if self.get((x, y)) != other.get((x, y)) {
					return false;
				}
			}
		}
		return true;
	}

	pub fn taken_adjacent_seats(&self, (x, y): (usize, usize)) -> usize {
		let mut count: usize = 0;
		let offsets: [(i32, i32); 8] = [
			(-1, -1),
			(0, -1),
			(1, -1),
			(-1, 0),
			(1, 0),
			(-1, 1),
			(0, 1),
			(1, 1),
		];
		for (o_x, o_y) in offsets.iter() {
			let fx = x as i32 + o_x;
			let fy = y as i32 + o_y;
			if fx >= 0 && fy >= 0 {
				if let Some(Pos::Seat(seat)) = self.get((fx as usize, fy as usize)) {
					if seat.occupied {
						count += 1;
					}
				}
			}
		}
		count
	}

	pub fn apply_flags(&mut self) {
		let (w, h) = self.size();
		for y in 0..h {
			for x in 0..w {
				if let Some(Pos::Seat(seat)) = self.get_mut((x, y)) {
					if !seat.locked {
						match seat.flag {
							SeatFlag::Exit => {
								seat.occupied = false;
							}
							SeatFlag::Enter => {
								seat.occupied = true;
							}
							_ => {}
						}
						seat.flag = SeatFlag::NoChange;
					}
				}
			}
		}
	}

	pub fn run_round(&mut self) -> usize {
		let (w, h) = self.size();
		let mut changes = HashSet::<(usize, usize)>::new();
		for y in 0..h {
			for x in 0..w {
				if let Some(Pos::Seat(seat)) = self.get_mut((x, y)) {
					if !seat.occupied && !seat.locked {
						seat.flag = SeatFlag::Enter;
						changes.insert((x, y));
					}
				}
			}
		}
		self.apply_flags();

		for y in 0..h {
			for x in 0..w {
				if self.taken_adjacent_seats((x, y)) >= 4 {
					if let Some(Pos::Seat(seat)) = self.get_mut((x, y)) {
						if seat.occupied && !seat.locked {
							seat.flag = SeatFlag::Exit;
							changes.remove(&(x, y));
						}
					}
				}
			}
		}
		self.apply_flags();

		for y in 0..h {
			for x in 0..w {
				if let Some(Pos::Seat(seat)) = self.get_mut((x, y)) {
					if seat.occupied {
						seat.locked = true;
					}
				}
			}
		}
		changes.len()
	}

	pub fn run_until_done(&mut self) {
		let mut round_count = 0;
		loop {
			round_count += 1;
			let changes_count = self.run_round();
			println!("Ran {}: {} changes", round_count, changes_count);
			if changes_count == 0 {
				return;
			}
		}
	}
}

impl std::fmt::Display for Ship {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut s: String = String::new();
		let (w, h) = (self.0[0].len(), self.0.len());
		for i in 0..h {
			for j in 0..w {
				s.push_str(format!("{}", self.get((j, i)).unwrap()).as_str())
			}
			s.push('\n');
		}
		write!(f, "{}", s)
	}
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum SeatFlag {
	NoChange,
	Exit,
	Enter,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pos {
	Floor,
	Seat(Seat),
}

impl Pos {
	pub fn from_char(c: char) -> Self {
		match c {
			'.' => Self::Floor,
			'L' => Self::Seat(Seat {
				occupied: false,
				..Default::default()
			}),
			_ => unreachable!(),
		}
	}
}

impl std::fmt::Display for Pos {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Pos::Floor => ".",
				Pos::Seat(seat) =>
					if seat.occupied {
						"#"
					} else {
						"L"
					},
			}
		)
	}
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Seat {
	pub occupied: bool,
	pub flag: SeatFlag,
	pub locked: bool,
}

impl Default for Seat {
	fn default() -> Self {
		Self {
			occupied: false,
			flag: SeatFlag::NoChange,
			locked: false,
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	pub fn testing_ship() -> Ship {
		Ship::from_str(
			"L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL",
		)
	}

	#[test]
	fn test_run() {
		let mut ship = testing_ship();

		println!("{}", ship);
		println!("Running 1");
		ship.run_until_done();
		println!("Run done");
		println!("{}", ship);
	}
}
