use crate::intcode::NumType;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
	North,
	East,
	South,
	West,
}

impl Direction {
	pub fn to_change(&self) -> (NumType, NumType) {
		match self {
			Direction::North => (0, 1),
			Direction::East => (1, 0),
			Direction::South => (0, -1),
			Direction::West => (-1, 0)
		}
	}
	pub fn from_coord_delta(mut x: NumType, y: NumType) -> Direction {
		if x != 0 && y != 0 && y.abs() > x.abs() {
			//x is less significant than the y, in this case?
			// And we only have 4 dirs
			x = 0
		}
		if x < 0 {
			Direction::West
		} else if x > 0 {
			Direction::East
		} else if y < 0 {
			Direction::South
		} else {
			Direction::North
		}
	}
	pub fn to_command(&self) -> NumType {
		match self {
			Direction::North => 1,
			Direction::East => 4,
			Direction::South => 2,
			Direction::West => 3
		}
	}
}
