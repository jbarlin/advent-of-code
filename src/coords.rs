use crate::direction::Direction;
use crate::intcode::NumType;

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Coords {
	pub x: NumType,
	pub y: NumType,
}

impl Coords {
	pub fn move_direction(&self, d: Direction) -> Coords {
		let change = d.to_change();
		Coords {
			x: self.x + change.0,
			y: self.y + change.1,
		}
	}
	pub fn move_away(&self, d: Direction) -> Coords {
		let change = d.to_change();
		Coords {
			x: self.x - change.0,
			y: self.y - change.1,
		}
	}
	pub fn turn_to_face(&self, next: &Coords) -> Direction {
		if (self.x + 1) == next.x {
			Direction::East
		}else if (self.x - 1) == next.x {
			Direction::West
		} else if (self.y + 1) == next.y {
			Direction::North
		} else {
			Direction::South
		}
	}
	pub fn flat_dist(&self, oth: &Coords) -> usize{
		let diff_x = if self.x > oth.x {(self.x - oth.x).abs()} else {(oth.x - self.x).abs()} as usize;
		let diff_y = if self.y > oth.y {(self.y - oth.y).abs()} else {(oth.y - self.y).abs()} as usize;
		if diff_x > 0 && diff_y > 0 {
			panic!("Cannot have both gt than 0")
		} else if diff_y > 0 {
			diff_y
		}else{
			diff_x
		}
	}
	pub fn add_y(&self, y: NumType) -> Coords{
		Coords{
			x: self.x,
			y: self.y + y
		}
	}
	pub fn add_x(&self, x: NumType) -> Coords{
		Coords{
			x: self.x + x,
			y: self.y
		}
	}
}
