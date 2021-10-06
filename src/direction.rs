use crate::intcode::NumType;

#[derive(Clone, Copy, Debug, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub enum Direction {
	North = 0,
	East = 1,
	South = 2,
	West = 3,
}

impl Direction {
	pub fn to_change(&self) -> (NumType, NumType) {
		match self {
			Direction::North => (0, -1),
			Direction::East => (1, 0),
			Direction::South => (0, 1),
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
	pub fn to_turn_commands(&self, other: Direction) -> &str {
		match self {
			Direction::North => {
				match other {
					Direction::North => "",
					Direction::East => "R,",
					Direction::South => "R,R,",
					Direction::West => "L,"
				}
			}
			Direction::East => {
				match other {
					Direction::North => "L,",
					Direction::East => "",
					Direction::South => "R,",
					Direction::West => "R,R,"
				}
			}
			Direction::South => {match other {
				Direction::North => "R,R,",
				Direction::East => "L,",
				Direction::South => "",
				Direction::West => "R,"
			}}
			Direction::West => {
				match other {
					Direction::North => "R,",
					Direction::East => "R,R,",
					Direction::South => "L,",
					Direction::West => ""
				}
			}
		}
	}
	pub fn opposite(&self) -> Direction {
		match self {
			Direction::North => Direction::South,
			Direction::East => Direction::East,
			Direction::South => Direction::North,
			Direction::West => Direction::West,
		}
	}
}
