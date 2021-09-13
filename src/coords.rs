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
	pub fn turn_to_face(&self, next: &Coords) -> Direction {
		Direction::from_coord_delta(next.x - self.x, next.y - self.y)
	}
}
