use std::convert::TryInto;

const ROWS: usize = 128;
const COLUMNS: usize = 8;

pub fn day05(_: Vec<String>) {
	let input_raw = crate::util::read_input("day05.txt");
	let seats = input_raw
		.split("\n")
		.map(|input| Partition::new(input))
		.collect::<Vec<Partition>>();

	println!("Parsed {} seats", seats.len());

	let highest_id = seats
		.iter()
		.fold(0, |max, seat| std::cmp::max(max, seat.id()));

	println!("The highest id is: {}", highest_id);

	let mut taken_seats = [[false; COLUMNS]; ROWS];
	for seat in seats {
		let (row, column) = seat.location();
		taken_seats[row][column] = true;
	}
	for i in 0..ROWS {
		for j in 0..COLUMNS {
			if !taken_seats[i][j] {
				println!("Open seat: row {}, column {}, id: {}", i, j, i * 8 + j);
			}
		}
	}
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum PartitionRow {
	Front,
	Back,
}

impl From<char> for PartitionRow {
	fn from(input: char) -> Self {
		match input {
			'F' => PartitionRow::Front,
			'B' => PartitionRow::Back,
			_ => unreachable!(),
		}
	}
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum PartitionColumn {
	Right,
	Left,
}

impl From<char> for PartitionColumn {
	fn from(input: char) -> Self {
		match input {
			'R' => PartitionColumn::Right,
			'L' => PartitionColumn::Left,
			_ => unreachable!(),
		}
	}
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Partition {
	row: [PartitionRow; 7],
	column: [PartitionColumn; 3],
}

impl Partition {
	pub fn new(input: &str) -> Self {
		Self {
			row: input
				.chars()
				.take(7)
				.map(|c| c.into())
				.collect::<Vec<PartitionRow>>()
				.as_slice()
				.try_into()
				.unwrap(),
			column: input
				.chars()
				.skip(7)
				.map(|c| c.into())
				.collect::<Vec<PartitionColumn>>()
				.as_slice()
				.try_into()
				.unwrap(),
		}
	}

	pub fn location(&self) -> (usize, usize) {
		// println!("RPD\tRPS\tRow");
		let mut row_part_size = ROWS;
		let mut row = row_part_size;
		for i in 0..7 {
			row_part_size /= 2;

			match self.row[i] {
				PartitionRow::Front => row -= row_part_size,
				PartitionRow::Back => row = row,
			}
			// println!("{:?}\t{}\t{}", self.row[i], row_part_size, row);
		}

		let mut column_part_size = COLUMNS;
		let mut column = column_part_size;
		for i in 0..3 {
			column_part_size /= 2;
			match self.column[i] {
				PartitionColumn::Right => column = column,
				PartitionColumn::Left => column -= column_part_size,
			}
		}

		(row - 1, column - 1)
	}

	pub fn id(&self) -> u32 {
		let (row, column) = self.location();
		(row * 8 + column) as u32
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_create_partition_location() {
		assert_eq!(
			Partition::new("BFFFBBFRRR"),
			Partition {
				row: [
					PartitionRow::Back,
					PartitionRow::Front,
					PartitionRow::Front,
					PartitionRow::Front,
					PartitionRow::Back,
					PartitionRow::Back,
					PartitionRow::Front,
				],
				column: [
					PartitionColumn::Right,
					PartitionColumn::Right,
					PartitionColumn::Right,
				]
			}
		)
	}

	#[test]
	fn test_partition_location() {
		assert_eq!(Partition::new("FBFBBFFRLR").location(), (44, 5));
		assert_eq!(Partition::new("BFFFBBFRRR").location(), (70, 7));
		assert_eq!(Partition::new("FFFBBBFRRR").location(), (14, 7));
		assert_eq!(Partition::new("BBFFBBFRLL").location(), (102, 4));

		assert_eq!(Partition::new("BFFFBBFRRR").id(), 567);
		assert_eq!(Partition::new("FFFBBBFRRR").id(), 119);
		assert_eq!(Partition::new("BBFFBBFRLL").id(), 820);
	}
}
