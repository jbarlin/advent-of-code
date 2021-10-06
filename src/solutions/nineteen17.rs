use std::sync::Arc;

use crate::SinglePart;
use crate::coords::Coords;
use crate::direction::Direction;
use crate::intcode::{IntCodeVM, NumType};
use crate::map::TwoDMap;

pub struct Code;

const DAY_17_DATA: [NumType; 1467] = [1, 330, 331, 332, 109, 3132, 1102, 1, 1182, 16, 1101, 1467, 0, 24, 101, 0, 0, 570, 1006, 570, 36, 101, 0, 571, 0, 1001, 570, -1, 570, 1001, 24, 1, 24, 1105, 1, 18, 1008, 571, 0, 571, 1001, 16, 1, 16, 1008, 16, 1467, 570, 1006, 570, 14, 21102, 58, 1, 0, 1106, 0, 786, 1006, 332, 62, 99, 21102, 1, 333, 1, 21102, 73, 1, 0, 1106, 0, 579, 1101, 0, 0, 572, 1101, 0, 0, 573, 3, 574, 101, 1, 573, 573, 1007, 574, 65, 570, 1005, 570, 151, 107, 67, 574, 570, 1005, 570, 151, 1001, 574, -64, 574, 1002, 574, -1, 574, 1001, 572, 1, 572, 1007, 572, 11, 570, 1006, 570, 165, 101, 1182, 572, 127, 1001, 574, 0, 0, 3, 574, 101, 1, 573, 573, 1008, 574, 10, 570, 1005, 570, 189, 1008, 574, 44, 570, 1006, 570, 158, 1105, 1, 81, 21101, 0, 340, 1, 1106, 0, 177, 21101, 0, 477, 1, 1105, 1, 177, 21101, 514, 0, 1, 21102, 176, 1, 0, 1106, 0, 579, 99, 21102, 1, 184, 0, 1106, 0, 579, 4, 574, 104, 10, 99, 1007, 573, 22, 570, 1006, 570, 165, 1002, 572, 1, 1182, 21102, 1, 375, 1, 21101, 0, 211, 0, 1106, 0, 579, 21101, 1182, 11, 1, 21102, 1, 222, 0, 1106, 0, 979, 21102, 388, 1, 1, 21102, 233, 1, 0, 1105, 1, 579, 21101, 1182, 22, 1, 21101, 244, 0, 0, 1106, 0, 979, 21102, 1, 401, 1, 21101, 255, 0, 0, 1105, 1, 579, 21101, 1182, 33, 1, 21102, 266, 1, 0, 1105, 1, 979, 21102, 1, 414, 1, 21102, 1, 277, 0, 1105, 1, 579, 3, 575, 1008, 575, 89, 570, 1008, 575, 121, 575, 1, 575, 570, 575, 3, 574, 1008, 574, 10, 570, 1006, 570, 291, 104, 10, 21102, 1, 1182, 1, 21101, 313, 0, 0, 1105, 1, 622, 1005, 575, 327, 1101, 0, 1, 575, 21102, 1, 327, 0, 1106, 0, 786, 4, 438, 99, 0, 1, 1, 6, 77, 97, 105, 110, 58, 10, 33, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 101, 32, 98, 117, 116, 32, 103, 111, 116, 58, 32, 0, 12, 70, 117, 110, 99, 116, 105, 111, 110, 32, 65, 58, 10, 12, 70, 117, 110, 99, 116, 105, 111, 110, 32, 66, 58, 10, 12, 70, 117, 110, 99, 116, 105, 111, 110, 32, 67, 58, 10, 23, 67, 111, 110, 116, 105, 110, 117, 111, 117, 115, 32, 118, 105, 100, 101, 111, 32, 102, 101, 101, 100, 63, 10, 0, 37, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 82, 44, 32, 76, 44, 32, 111, 114, 32, 100, 105, 115, 116, 97, 110, 99, 101, 32, 98, 117, 116, 32, 103, 111, 116, 58, 32, 36, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 99, 111, 109, 109, 97, 32, 111, 114, 32, 110, 101, 119, 108, 105, 110, 101, 32, 98, 117, 116, 32, 103, 111, 116, 58, 32, 43, 10, 68, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 109, 97, 121, 32, 98, 101, 32, 97, 116, 32, 109, 111, 115, 116, 32, 50, 48, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114, 115, 33, 10, 94, 62, 118, 60, 0, 1, 0, -1, -1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 10, 0, 109, 4, 1202, -3, 1, 587, 20102, 1, 0, -1, 22101, 1, -3, -3, 21101, 0, 0, -2, 2208, -2, -1, 570, 1005, 570, 617, 2201, -3, -2, 609, 4, 0, 21201, -2, 1, -2, 1105, 1, 597, 109, -4, 2105, 1, 0, 109, 5, 2102, 1, -4, 630, 20102, 1, 0, -2, 22101, 1, -4, -4, 21101, 0, 0, -3, 2208, -3, -2, 570, 1005, 570, 781, 2201, -4, -3, 653, 20101, 0, 0, -1, 1208, -1, -4, 570, 1005, 570, 709, 1208, -1, -5, 570, 1005, 570, 734, 1207, -1, 0, 570, 1005, 570, 759, 1206, -1, 774, 1001, 578, 562, 684, 1, 0, 576, 576, 1001, 578, 566, 692, 1, 0, 577, 577, 21101, 702, 0, 0, 1105, 1, 786, 21201, -1, -1, -1, 1105, 1, 676, 1001, 578, 1, 578, 1008, 578, 4, 570, 1006, 570, 724, 1001, 578, -4, 578, 21101, 0, 731, 0, 1106, 0, 786, 1106, 0, 774, 1001, 578, -1, 578, 1008, 578, -1, 570, 1006, 570, 749, 1001, 578, 4, 578, 21101, 0, 756, 0, 1105, 1, 786, 1105, 1, 774, 21202, -1, -11, 1, 22101, 1182, 1, 1, 21102, 1, 774, 0, 1106, 0, 622, 21201, -3, 1, -3, 1106, 0, 640, 109, -5, 2106, 0, 0, 109, 7, 1005, 575, 802, 21002, 576, 1, -6, 20101, 0, 577, -5, 1105, 1, 814, 21101, 0, 0, -1, 21101, 0, 0, -5, 21102, 1, 0, -6, 20208, -6, 576, -2, 208, -5, 577, 570, 22002, 570, -2, -2, 21202, -5, 45, -3, 22201, -6, -3, -3, 22101, 1467, -3, -3, 1201, -3, 0, 843, 1005, 0, 863, 21202, -2, 42, -4, 22101, 46, -4, -4, 1206, -2, 924, 21102, 1, 1, -1, 1105, 1, 924, 1205, -2, 873, 21101, 0, 35, -4, 1105, 1, 924, 2102, 1, -3, 878, 1008, 0, 1, 570, 1006, 570, 916, 1001, 374, 1, 374, 2102, 1, -3, 895, 1102, 2, 1, 0, 1201, -3, 0, 902, 1001, 438, 0, 438, 2202, -6, -5, 570, 1, 570, 374, 570, 1, 570, 438, 438, 1001, 578, 558, 921, 21001, 0, 0, -4, 1006, 575, 959, 204, -4, 22101, 1, -6, -6, 1208, -6, 45, 570, 1006, 570, 814, 104, 10, 22101, 1, -5, -5, 1208, -5, 37, 570, 1006, 570, 810, 104, 10, 1206, -1, 974, 99, 1206, -1, 974, 1102, 1, 1, 575, 21101, 0, 973, 0, 1106, 0, 786, 99, 109, -7, 2105, 1, 0, 109, 6, 21101, 0, 0, -4, 21102, 0, 1, -3, 203, -2, 22101, 1, -3, -3, 21208, -2, 82, -1, 1205, -1, 1030, 21208, -2, 76, -1, 1205, -1, 1037, 21207, -2, 48, -1, 1205, -1, 1124, 22107, 57, -2, -1, 1205, -1, 1124, 21201, -2, -48, -2, 1106, 0, 1041, 21102, 1, -4, -2, 1105, 1, 1041, 21101, 0, -5, -2, 21201, -4, 1, -4, 21207, -4, 11, -1, 1206, -1, 1138, 2201, -5, -4, 1059, 1202, -2, 1, 0, 203, -2, 22101, 1, -3, -3, 21207, -2, 48, -1, 1205, -1, 1107, 22107, 57, -2, -1, 1205, -1, 1107, 21201, -2, -48, -2, 2201, -5, -4, 1090, 20102, 10, 0, -1, 22201, -2, -1, -2, 2201, -5, -4, 1103, 2101, 0, -2, 0, 1106, 0, 1060, 21208, -2, 10, -1, 1205, -1, 1162, 21208, -2, 44, -1, 1206, -1, 1131, 1106, 0, 989, 21102, 1, 439, 1, 1105, 1, 1150, 21101, 0, 477, 1, 1106, 0, 1150, 21102, 1, 514, 1, 21102, 1, 1149, 0, 1105, 1, 579, 99, 21101, 1157, 0, 0, 1106, 0, 579, 204, -2, 104, 10, 99, 21207, -3, 22, -1, 1206, -1, 1138, 1201, -5, 0, 1176, 2102, 1, -4, 0, 109, -6, 2106, 0, 0, 8, 9, 36, 1, 7, 1, 36, 1, 1, 13, 30, 1, 7, 1, 5, 1, 30, 1, 7, 1, 5, 1, 7, 11, 12, 1, 7, 1, 5, 1, 7, 1, 9, 1, 12, 1, 7, 1, 5, 1, 7, 1, 1, 13, 8, 1, 7, 1, 5, 1, 7, 1, 1, 1, 7, 1, 3, 1, 8, 1, 7, 1, 5, 1, 7, 1, 1, 1, 7, 1, 3, 1, 8, 1, 7, 1, 5, 1, 7, 1, 1, 1, 7, 1, 3, 10, 5, 9, 1, 13, 3, 1, 3, 1, 14, 1, 1, 1, 7, 1, 5, 1, 1, 1, 3, 1, 3, 1, 3, 1, 8, 9, 5, 9, 1, 1, 3, 1, 3, 1, 3, 1, 8, 1, 5, 1, 7, 1, 1, 1, 7, 1, 3, 1, 3, 1, 3, 1, 8, 1, 5, 1, 7, 1, 1, 1, 7, 1, 3, 1, 3, 1, 3, 1, 8, 1, 5, 1, 7, 1, 1, 1, 7, 1, 3, 1, 3, 1, 3, 1, 8, 1, 5, 1, 7, 1, 1, 1, 7, 9, 3, 1, 8, 1, 5, 1, 7, 1, 1, 1, 11, 1, 7, 1, 8, 1, 5, 1, 7, 1, 1, 11, 1, 9, 8, 1, 5, 1, 7, 1, 11, 1, 18, 13, 1, 1, 11, 1, 24, 1, 5, 1, 1, 1, 11, 1, 24, 9, 11, 1, 30, 1, 13, 1, 30, 1, 13, 1, 30, 1, 13, 1, 30, 1, 13, 9, 22, 1, 21, 1, 22, 11, 11, 1, 32, 1, 11, 1, 32, 1, 11, 1, 32, 1, 11, 1, 32, 1, 11, 1, 32, 1, 11, 1, 32, 1, 11, 1, 32, 1, 11, 1, 32, 13, 2];

struct InstructionSet {
	pub command: Arc<String>,
	pub alen: usize,
	pub blen: usize,
	pub clen: usize,
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Hash)]
struct CompressedInstructions {
	pub main: String,
	pub A: String,
	pub B: String,
	pub C: String,
}

impl CompressedInstructions {
	pub fn len(&self) -> usize {
		self.main.len() + self.A.len() + self.B.len() + self.C.len()
	}
}

pub fn both_parts_impl(input: [NumType; 1467]) -> String {
	let mut output = IntCodeVM::new_run_all_output(Vec::from(input.clone()));
	let map = TwoDMap::from_output_of_chars(
		&mut output,
		'#',
		'.', Vec::new(),
		Vec::new(),
		Vec::from(['^', '>', '<', 'v']),
	);
	let sum: NumType = map.0.intersections()
		.iter()
		.filter(|(_, v)| {
			v.len() == 4
		})
		.map(|(c, _)| {
			c.x * c.y
		})
		.sum();
	//OK, now to work out the shortest path?
	let complete_map = map.0;
	let start_point = *map.1.get(0).unwrap();
	let start_coord = start_point.0;
	let mut curr_facing: Direction = match start_point.1 {
		'^' => Direction::North,
		'>' => Direction::East,
		'<' => Direction::West,
		'v' => Direction::South,
		_ => unreachable!("Huh?")
	};


	let possible_paths = complete_map
		.step_all_tiles(start_coord, curr_facing);

	let paths_as_command_lists: Vec<Arc<String>> = possible_paths
		.iter()
		//.iter()
		.map(|path| {
			let mut best_path: Vec<Coords> = path.path.clone();
			let mut commands: String = "".to_string();
			let mut steps: usize = 0;
			//Axe the start node ig
			best_path.reverse();
			let mut curr_node = best_path.pop().unwrap();
			while let Some(node) = best_path.pop() {
				let need_facing = curr_node.turn_to_face(&node);
				if need_facing.eq(&curr_facing) {
					steps += 1;
				} else {
					if steps > 0 {
						steps.to_string().chars().for_each(|c| commands.push(c));
						commands.push(',');
					}
					steps = 1;
					let turn_command = curr_facing.to_turn_commands(need_facing);
					commands.push_str(turn_command);
					curr_facing = curr_node.turn_to_face(&node);
				}
				curr_node = node;
			}
			if steps > 0 {
				steps.to_string().chars().for_each(|c| commands.push(c));
			}
			return Arc::new(commands);
		})
		.collect();
	let mut instruction_sets: Vec<InstructionSet> = Vec::with_capacity(18_usize.pow(3) * possible_paths.len());
	for a in 2..=20 {
		for b in 2..=20 {
			for c in 2..=20 {
				for pt in &paths_as_command_lists {
					instruction_sets.push(InstructionSet {
						command: pt.clone(),
						alen: a,
						blen: b,
						clen: c,
					});
				}
			}
		}
	}

	let commands = instruction_sets
		.iter()
		.map(|inst| {
			let min_len = inst.alen.min(inst.blen).min(inst.clen) as usize;
			if inst.command.chars().nth(inst.alen).is_none(){
				return Option::None;
			}
			let mut compressed_command = "A".to_string();
			let mut bstr: Option<&str> = Option::None;
			let mut cstr: Option<&str> = Option::None;
			let (astr, mut remaining) = inst.command.split_at(inst.alen as usize);
			while remaining.len() > min_len {
				if remaining.len() >= inst.alen && &remaining[0..=inst.alen] == astr{
					compressed_command.push(',');
					compressed_command.push('A');
					remaining = &remaining[inst.alen..];
				}else{
					if let Some(bsub) = bstr {
						if remaining.len() >= inst.blen && &remaining[0..=inst.blen] == bsub{
							compressed_command.push(',');
							compressed_command.push('B');
							remaining = &remaining[inst.blen..];
						}else{
							if let Some(csub) = cstr {
								if remaining.len() >= inst.clen && &remaining[0..=inst.clen] == csub{
									compressed_command.push(',');
									compressed_command.push('C');
									remaining = &remaining[inst.clen..];
								}else{
									return Option::None;
								}
							} else {
								match remaining.chars().nth(inst.clen) {
									Some(_) => {
										compressed_command.push(',');
										compressed_command.push('C');
										cstr = Option::Some(&remaining[0..=inst.clen]);
										remaining = &remaining[inst.clen..];
									},
									_ => {
										return Option::None;
									}
								}
							}
						}
					} else {
						match remaining.chars().nth(inst.blen) {
							Some(_) => {
								compressed_command.push(',');
								compressed_command.push('B');
								bstr = Option::Some(&remaining[0..=inst.blen]);
								remaining = &remaining[inst.blen..];
							},
							_ => {
								return Option::None;
							}
						}
					}
				}
			}
			if bstr.is_some() && cstr.is_some() && remaining.len() < 2 {
				let mut bfinal = bstr.unwrap();
				if bfinal.chars().nth(0).unwrap() == ',' {
					bfinal = &bfinal[1..];
				}
				if bfinal.chars().nth_back(0).unwrap() == ',' {
					bfinal = &bfinal[0..bfinal.len()]
				}
				let mut cfinal = cstr.unwrap();
				if cfinal.chars().nth(0).unwrap() == ',' {
					cfinal = &cfinal[1..];
				}
				if cfinal.chars().nth_back(0).unwrap() == ',' {
					cfinal = &cfinal[0..cfinal.len()]
				}
				if compressed_command.len() < 20 && cfinal.len() < 20 && bfinal.len() < 20 && astr.len() < 20 {
					return Option::Some(CompressedInstructions {
						main: compressed_command,
						A: astr.to_string(),
						B: bfinal.to_string(),
						C: cfinal.to_string(),
					});
				}else{
					println!("a\t{:?}\tb{:?}\tc{:?}\trem{:?}\tcmd{:?}", astr, bstr, cstr, remaining, compressed_command );
				}
			}
			println!("a\t{:?}\tb{:?}\tc{:?}\trem{:?}\tcmd{:?}", astr, bstr, cstr, remaining, compressed_command );
			return Option::None;
		})
		.filter(|m| m.is_some())
		.map(|v| v.unwrap())
		.min_by(|a, b| {
			a.len().cmp(&b.len())
		});
	let fmt = format!(
		"Part 1:\t{:?}\nPart 2:\t{:#?}\n",
		sum.to_string(),
		commands
	);
	return fmt;
}

impl SinglePart for Code {
	fn run(&self) -> String {
		return both_parts_impl(DAY_17_DATA);
	}
}