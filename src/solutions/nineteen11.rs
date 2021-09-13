use crate::AoCDay;

use super::super::image_layer::Pixel;
use super::super::image_layer::ImageLayer;
use super::super::intcode::IntCodeVM;
use super::super::intcode::Memory;
use super::super::intcode::NumType;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::coords::Coords;

pub const DAY_11_DATA: [NumType; 633] = [3,8,1005,8,311,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1002,8,1,29,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,50,1,2,19,10,1006,0,23,1,103,14,10,1,1106,15,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,102,1,8,88,1006,0,59,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,1002,8,1,113,2,101,12,10,2,1001,0,10,2,1006,14,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,102,1,8,146,1,1106,11,10,1006,0,2,1,9,8,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,180,1,6,13,10,1,1102,15,10,2,7,1,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,1002,8,1,213,1006,0,74,2,1005,9,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,1002,8,1,243,3,8,1002,8,-1,10,101,1,10,10,4,10,108,1,8,10,4,10,101,0,8,264,2,104,8,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,1001,8,0,290,101,1,9,9,1007,9,952,10,1005,10,15,99,109,633,104,0,104,1,21101,387512640296,0,1,21101,0,328,0,1106,0,432,21102,1,665749660564,1,21101,339,0,0,1106,0,432,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,179318226984,1,1,21101,386,0,0,1105,1,432,21101,46266346499,0,1,21101,0,397,0,1105,1,432,3,10,104,0,104,0,3,10,104,0,104,0,21102,709580555028,1,1,21102,420,1,0,1106,0,432,21102,1,988220642068,1,21101,0,431,0,1106,0,432,99,109,2,21202,-1,1,1,21101,40,0,2,21102,1,463,3,21102,1,453,0,1106,0,496,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,458,459,474,4,0,1001,458,1,458,108,4,458,10,1006,10,490,1102,0,1,458,109,-2,2105,1,0,0,109,4,2102,1,-1,495,1207,-3,0,10,1006,10,513,21101,0,0,-3,21201,-3,0,1,22101,0,-2,2,21102,1,1,3,21101,532,0,0,1106,0,537,109,-4,2106,0,0,109,5,1207,-3,1,10,1006,10,560,2207,-4,-2,10,1006,10,560,22102,1,-4,-4,1105,1,628,21201,-4,0,1,21201,-3,-1,2,21202,-2,2,3,21102,1,579,0,1105,1,537,22101,0,1,-4,21101,1,0,-1,2207,-4,-2,10,1006,10,598,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,620,22101,0,-1,1,21102,620,1,0,106,0,495,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0];

#[derive(Copy, Clone)]
enum Facing {
	YPos,
	YNeg,
	XPos,
	XNeg,
}
enum Turn {
	Left = 0,
	Right = 1,
}
impl Turn {
	pub fn convert_int(value: i64) -> Turn {
		match value {
			0 => Turn::Left,
			_ => Turn::Right,
		}
	}
}
type Hull = HashMap<Coords, Pixel>;
struct PainterRobot {
	coords: Coords,
	facing: Facing,
	intcode: IntCodeVM,
	hull: Hull,
	vmout: Rc<RefCell<VecDeque<NumType>>>,
}
impl PainterRobot {
	fn new(start_tile: Pixel, intcode_mem: Memory) -> PainterRobot {
		let mut hull = HashMap::new();
		let mut intcode = IntCodeVM::new(intcode_mem);
		let vmout = intcode.output();
		match start_tile {
			Pixel::Black => intcode.push_input(Pixel::Black as i64),
			Pixel::White => {
				intcode.push_input(Pixel::White as i64);
				hull.insert(Coords { x: 0, y: 0 }, Pixel::White);
			}
			_ => unimplemented!(),
		}
		return PainterRobot {
			coords: Coords { x: 0, y: 0 },
			facing: Facing::YPos,
			hull,
			intcode,
			vmout,
		};
	}
	fn read(&self) -> Pixel {
		let r: Option<&Pixel> = self.hull.get(&self.coords);
		match r {
			Some(x) => *x,
			None => Pixel::Black,
		}
	}
	fn paint(&mut self, colour: Pixel) {
		self.hull.insert(self.coords, colour);
	}
	fn turn(&mut self, turn: Turn) {
		match (self.facing, turn) {
			(Facing::YPos, Turn::Left) => {
				self.facing = Facing::XNeg;
				self.coords.x -= 1;
			}
			(Facing::YPos, Turn::Right) => {
				self.facing = Facing::XPos;
				self.coords.x += 1;
			}
			(Facing::YNeg, Turn::Left) => {
				self.facing = Facing::XPos;
				self.coords.x += 1;
			}
			(Facing::YNeg, Turn::Right) => {
				self.facing = Facing::XNeg;
				self.coords.x -= 1;
			}
			(Facing::XPos, Turn::Left) => {
				self.facing = Facing::YPos;
				self.coords.y += 1;
			}
			(Facing::XPos, Turn::Right) => {
				self.facing = Facing::YNeg;
				self.coords.y -= 1;
			}
			(Facing::XNeg, Turn::Left) => {
				self.facing = Facing::YNeg;
				self.coords.y -= 1;
			}
			(Facing::XNeg, Turn::Right) => {
				self.facing = Facing::YPos;
				self.coords.y += 1;
			}
		};
	}
	fn run(mut self) -> Hull {
		while !self.intcode.is_stopped() {
			self.intcode.run_all();
			//Let's see how much we have to do
			let paint: Option<i64> = self.vmout.borrow_mut().pop_front();
			let turn: Option<i64> = self.vmout.borrow_mut().pop_front();
			match (paint, turn) {
				(Some(colour), Some(direction)) => {
					self.paint(Pixel::convert_int(colour));
					self.turn(Turn::convert_int(direction));
				}
				(None, None) => {
					//No action, I guess?
					//Not strictly a fail, the vm might have stopped!
				}
				_ => unreachable!(),
			};
			self.intcode.push_input(self.read() as i64);
		}
		return self.hull;
	}
}

pub fn part_1_impl(inp: [NumType; 633]) -> String {
	let hull = PainterRobot::new(Pixel::Black, Vec::from(inp)).run();
	return hull.len().to_string();
}

pub fn part_2_impl(inp: [NumType; 633]) -> String {
	let hull = PainterRobot::new(Pixel::White, Vec::from(inp)).run();
	let il: ImageLayer = ImageLayer::from_hashmap(hull);
	return il.to_string();
}

pub struct Code;
impl AoCDay for Code {
	fn part1(&self) -> String {
		part_1_impl(DAY_11_DATA)
	}
	fn part2(&self) -> String {
		part_2_impl(DAY_11_DATA)
	}
}
