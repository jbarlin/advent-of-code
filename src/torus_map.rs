use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use crate::cell::CellType;
use crate::coords::Coords;
use crate::direction::Direction;
use crate::map::TwoDMap;

type Map<A, B> = BTreeMap<A, B>;

pub struct Torus {}

impl Torus {

	pub fn str_to_map(input: &str) -> TwoDMap{
		Torus::parse_to_two_d_map(Torus::parse_input(input))
	}

	fn parse_input(input: &str) -> Map<Coords, char> {
		let mut retme: Map<Coords, char> = Map::new();
		let mut y: i64 = 0;
		for line in input.lines() {
			let mut x: i64 = 0;
			for ch in line.chars() {
				retme.insert(Coords { x, y }, ch);
				x += 1;
			}
			y += 1;
		}
		retme
	}
	fn parse_to_two_d_map(input: Map<Coords, char>) -> TwoDMap {
		let mut cell_map: Map<Coords, CellType> = Map::new();
		//Let's start by parsing the main map
		input
			.iter()
			.filter(|(&coord, &ch)| {
				return ch == '#' || ch == '.';
			})
			.for_each(|(&coord, &ch)| {
				if ch == '.' {
					cell_map.insert(coord, CellType::Space);
				} else {
					cell_map.insert(coord, CellType::NormalBarrier);
				}
			});
		//Try and work out the outer part (2 chars) and the inner part
		//Then parse the inner/outer parts
		//I hope
		let all_btm_rgh: Coords = *input.iter().last().unwrap().0;
		let maz_btm_rgh: Coords = all_btm_rgh.add_y(-2).add_x(-2);
		let maz_top_lft: Coords = Coords { x: 2, y: 2 };

		let inr = input
			.iter()
			.filter(|(&coord, &ch)| {
				ch != '.' && ch != '#' && (
					coord.y > maz_top_lft.y && coord.y < maz_btm_rgh.y
				) && (
					coord.x > maz_top_lft.x && coord.x < maz_btm_rgh.x
				)
			});
		let inr_top_lft = inr.clone().min_by(|(&a_coord, _), (&b_coord, _)| {
			let y_cmp = a_coord.y.cmp(&b_coord.y);
			match y_cmp {
				Ordering::Equal => a_coord.x.cmp(&b_coord.x),
				_ => y_cmp
			}
		})
			.unwrap()
			.0
			.clone();
		let inr_btm_right = inr.max_by(|(&a_coord, _), (&b_coord, _)| {
			let y_cmp = a_coord.y.cmp(&b_coord.y);
			match y_cmp {
				Ordering::Equal => a_coord.x.cmp(&b_coord.x),
				_ => y_cmp
			}
		})
			.unwrap()
			.0.clone();
		//To work out the u16 value, take the u8 of both the chars and add together as (.0 * 100 + .1)
		//That should give unique values for each (AA, AB, AC... BA, BB... ZZ) etc.
		//ofc AA will be start and ZZ will be goal
		//OK, let's loop around the outer edge!
		input
			.iter()
			.filter(|(&coord, &ch)| {
				return (coord.x == 0 || coord.x == all_btm_rgh.x
					|| coord.y == 0 || coord.y == all_btm_rgh.y)
					&& ch != ' '
				;
			})
			.for_each(|(&coord, &ch)| {
				if coord.x == 0 {
					let nxc = coord.add_x(1);
					let nx: char = input.get(&nxc).unwrap().clone();
					//Read left to right, so:
					let cd: u16 = (ch as u16) * 100 + (nx as u16);
					Torus::handle_portal_outer(&mut cell_map, nxc.move_direction(Direction::East), cd)
				} else if coord.y == 0 {
					let nxc = coord.add_y(1);
					let nx: char = input.get(&nxc).unwrap().clone();
					//Read top to bottom, so:
					let cd: u16 = (ch as u16) * 100 + (nx as u16);
					Torus::handle_portal_outer(&mut cell_map, nxc.move_direction(Direction::South), cd)
				} else if coord.x == all_btm_rgh.x {
					let nxc = coord.add_x(-1);
					let nx: char = input.get(&nxc).unwrap().clone();
					//Read left to right, so:
					let cd: u16 = (nx as u16) * 100 + (ch as u16);
					Torus::handle_portal_outer(&mut cell_map, nxc.move_direction(Direction::West), cd)
				} else {
					let nxc = coord.add_y(-1);
					let nx: char = input.get(&nxc).unwrap().clone();
					//Read top to bottom, so:
					let cd: u16 = (nx as u16) * 100 + (ch as u16);
					Torus::handle_portal_outer(&mut cell_map, nxc.move_direction(Direction::North), cd)
				}
			});
		//Now the inner edge!
		input
			.iter()
			.filter(|(&coord, &ch)| {
				return (coord.x == inr_top_lft.x || coord.x == inr_btm_right.x
					|| coord.y == inr_top_lft.y || coord.y == inr_btm_right.y)
				&& ch != ' ' && ch != '#' && ch != '.'
				;
			})
			.for_each(|(&coord, &ch)| {
				if coord.x == inr_top_lft.x {
					let nxc = coord.add_x(1);
					let nx: char = input.get(&nxc).unwrap().clone();
					//Read left to right, so:
					let cd: u16 = (ch as u16) * 100 + (nx as u16);
					Torus::handle_portal_inner(&mut cell_map, coord.move_direction(Direction::West), cd);
				} else if coord.y == inr_top_lft.y {
					
					let nxc = coord.add_y(1);
					let nx: char = input.get(&nxc).unwrap().clone();
					//Read top to bottom, so:
					let cd: u16 = (ch as u16) * 100 + (nx as u16);
					Torus::handle_portal_inner(&mut cell_map, coord.move_direction(Direction::North), cd);
				} else if coord.x == inr_btm_right.x {
					
					let nxc = coord.add_x(-1);
					let nx: char = input.get(&nxc).unwrap().clone();
					//Read left to right, so:
					let cd: u16 = (nx as u16) * 100 + (ch as u16);
					Torus::handle_portal_inner(&mut cell_map, coord.move_direction(Direction::East), cd);
				} else {
					
					let nxc = coord.add_y(-1);
					let nx: char = input.get(&nxc).unwrap().clone();
					//Read top to bottom, so:
					let cd: u16 = (nx as u16) * 100 + (ch as u16);
					Torus::handle_portal_inner(&mut cell_map, coord.move_direction(Direction::South), cd);
				}
			});
		return TwoDMap::from_map(cell_map);
	}

	fn handle_portal_outer(mut cell_map: &mut Map<Coords, CellType>, nxc: Coords, cd: u16) {
		if cd == (('A' as u16) * 100 + ('A' as u16)) {
			cell_map.insert(nxc, CellType::Start('@'));
		} else if cd == (('Z' as u16) * 100 + ('Z' as u16)) {
			cell_map.insert(nxc, CellType::Goal('A'));
		} else {
			cell_map.insert(nxc, CellType::WarpOuter(cd));
		}
	}
	fn handle_portal_inner(mut cell_map: &mut Map<Coords, CellType>, nxc: Coords, cd: u16) {
		if cd == (('A' as u16) * 100 + ('A' as u16)) {
			cell_map.insert(nxc, CellType::Start('@'));
		} else if cd == (('Z' as u16) * 100 + ('Z' as u16)) {
			cell_map.insert(nxc, CellType::Goal('A'));
		} else {
			cell_map.insert(nxc, CellType::WarpInner(cd));
		}
	}
}