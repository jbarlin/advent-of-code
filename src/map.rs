use std::collections::{BTreeMap, BTreeSet, VecDeque};

//use itertools::Itertools;

use crate::cell::CellType;
use crate::coords::Coords;
use crate::direction::Direction;
use crate::image_layer::{ImageLayer, Pixel};
use crate::intcode::NumType;

type Map<A, B> = BTreeMap<A, B>;
type Set<A> = BTreeSet<A>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntersectionTarget {
	pub direction: Direction,
	pub target: Coords,
	pub distance: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoDMap {
	pub(crate) cell_map: Map<Coords, CellType>,
	intersections: Map<Coords, Vec<IntersectionTarget>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Path {
	pub path: Vec<Coords>,
	nodes_visited: Map<Coords, usize>,
	collected_goals: Vec<char>,
}

impl Path {
	pub fn path(&self) -> &Vec<Coords> {
		&self.path
	}
	pub fn cost(&self) -> usize {
		self.path.len()
	}
}

impl TwoDMap {
	pub fn cell_map(&self) -> &Map<Coords, CellType> {
		&self.cell_map
	}
	pub fn intersections(&self) -> &Map<Coords, Vec<IntersectionTarget>> {
		&self.intersections
	}

	pub fn step_all_tiles(&self, start_point: Coords, facing: Direction) -> Vec<Path> {
		//Setup for the while loop
		let mut pathways: Map<Coords, usize> = Map::new();
		self.cell_map
			.iter()
			.for_each(|tuple| {
				match tuple.1 {
					CellType::NormalBarrier => {}
					CellType::SpecialBarrier(_) => {}
					_ => {
						pathways.insert(tuple.0.clone(), 0);
					}
				}
			});
		*pathways.entry(start_point).or_insert(0) += 1;
		let mut path = Vec::new();
		path.push(start_point);
		let start_path = Path {
			path,
			nodes_visited: pathways.clone(),
			collected_goals: Vec::new(),
		};
		//Run the while loop!
		let mut completed_paths: Vec<Path> = Vec::new();
		let mut active_paths: Vec<(Path, Coords, Direction)> = Vec::new();
		active_paths.push((start_path, start_point, facing));
		while let Some((curr_path, curr_point, curr_facing)) = active_paths.pop() {
			let has_hit_all = curr_path.nodes_visited
				.iter()
				.all(|f| (*(f.1)) > 0);
			if has_hit_all {
				completed_paths.push(curr_path.clone());
			} else {
				let north = curr_point.move_direction(Direction::North);
				let times_moved_north = match curr_path.nodes_visited
					.get(&north) {
					None => usize::MAX,
					Some(x) => x.clone() + if curr_facing == Direction::South { 300 } else { 0 }
				};
				let south = curr_point.move_direction(Direction::South);
				let times_moved_south = match curr_path.nodes_visited
					.get(&south) {
					None => usize::MAX,
					Some(x) => x.clone() + if curr_facing == Direction::North { 300 } else { 0 }
				};

				let east = curr_point.move_direction(Direction::East);
				let times_moved_east = match curr_path.nodes_visited
					.get(&east) {
					None => usize::MAX,
					Some(x) => x.clone() + if curr_facing == Direction::West { 300 } else { 0 }
				};

				let west = curr_point.move_direction(Direction::West);
				let times_moved_west = match curr_path.nodes_visited
					.get(&west) {
					None => usize::MAX,
					Some(x) => x.clone() + if curr_facing == Direction::East { 300 } else { 0 }
				};

				let min: usize = times_moved_north.min(times_moved_south)
					.min(times_moved_west)
					.min(times_moved_east);
				if min > 3 {
					//println!("Early finish:\tn: {:?}\ts: {:?}\te: {:?}\tw: {:?}\tc: {:?}", times_moved_north, times_moved_south, times_moved_east, times_moved_west, curr_point);
					//Probs not hun
					continue;
				} else {
					//println!("LOOP LOOP:\tn: {:?}\ts: {:?}\te: {:?}\tw: {:?}\tc: {:?}", times_moved_north, times_moved_south, times_moved_east, times_moved_west, curr_point);
					if times_moved_west == min {
						let mut n_path = curr_path.clone();
						n_path.path.push(west);
						n_path.nodes_visited.insert(west, min + 1);
						active_paths.push((n_path, west, Direction::West))
					}
					if times_moved_south == min {
						let mut n_path = curr_path.clone();
						n_path.path.push(south);
						n_path.nodes_visited.insert(south, min + 1);
						active_paths.push((n_path, south, Direction::South))
					}
					if times_moved_east == min {
						let mut n_path = curr_path.clone();
						n_path.path.push(east);
						n_path.nodes_visited.insert(east, min + 1);
						active_paths.push((n_path, east, Direction::East))
					}
					if times_moved_north == min {
						let mut n_path = curr_path.clone();
						n_path.path.push(north);
						n_path.nodes_visited.insert(north, min + 1);
						active_paths.push((n_path, north, Direction::North))
					}
				}
			}
		}
		completed_paths
	}

	pub fn from_str(input: &str, space: char, barrier: char, special_barrier: Vec<char>, special_goal: Vec<char>, curr_loc: Vec<char>)
					-> (TwoDMap, Vec<Coords>) {
		let mut cell_map: Map<Coords, CellType> = Map::new();
		let mut curr_lcs: Vec<Coords> = Vec::new();
		let mut y: NumType = 0;
		for line in input.trim().lines() {
			let mut x = 0;
			for ch in line.chars() {
				if ch == space {
					cell_map.insert(Coords { x, y }, CellType::Space);
				} else if ch == barrier {
					cell_map.insert(Coords { x, y }, CellType::NormalBarrier);
				} else if special_barrier.contains(&ch) {
					cell_map.insert(Coords { x, y }, CellType::SpecialBarrier(ch));
				} else if special_goal.contains(&ch) {
					cell_map.insert(Coords { x, y }, CellType::Goal(ch));
				} else if curr_loc.contains(&ch) {
					cell_map.insert(Coords { x, y }, CellType::Start(ch));
					curr_lcs.push(Coords { x, y });
				} else {
					panic!("Cannot deal with unknown");
				}
				x += 1;
			}
			y += 1;
		}
		(TwoDMap::from_map(cell_map), curr_lcs)
	}

	pub fn print(&self){
		let mx = self.cell_map
			.iter()
			.map(|(&coord, &ct)|{
				match ct {
					CellType::Space => (coord, Pixel::Black),
					CellType::NormalBarrier => (coord, Pixel::White),
					CellType::SpecialBarrier(_) => (coord, Pixel::CapP),
					CellType::Goal(_) => (coord, Pixel::LowP),
					CellType::Start(_) => (coord, Pixel::Transparent),
					CellType::WarpInner(_) => (coord, Pixel::Star),
					CellType::WarpOuter(_) => (coord, Pixel::Star),
					CellType::Items => (coord, Pixel::Star),
				}
			})
			.collect();
		println!("{}",ImageLayer::from_hashmap(mx).to_string());
	}
	
	pub fn from_output_of_chars
	(output: &mut VecDeque<NumType>, space: char, barrier: char, special_barrier: Vec<char>, special_goal: Vec<char>, curr_loc: Vec<char>)
	 -> (TwoDMap, Vec<(Coords, char)>) {
		let mut cell_map: Map<Coords, CellType> = Map::new();
		let mut curr_lcs: Vec<(Coords, char)> = Vec::new();
		let mut x: NumType = 0;
		let mut y: NumType = 0;
		while let Some(retable) = output.pop_front() {
			if retable == 10 {
				x = 0;
				y += 1;
			} else {
				let ch = (retable as u8) as char;
				if ch == space {
					cell_map.insert(Coords { x, y }, CellType::Space);
				} else if ch == barrier {
					cell_map.insert(Coords { x, y }, CellType::NormalBarrier);
				} else if special_barrier.contains(&ch) {
					cell_map.insert(Coords { x, y }, CellType::SpecialBarrier(ch.clone()));
				} else if special_goal.contains(&ch) {
					cell_map.insert(Coords { x, y }, CellType::Goal(ch.clone()));
				} else if curr_loc.contains(&ch) {
					cell_map.insert(Coords { x, y }, CellType::Space);
					curr_lcs.push((Coords { x, y }, ch.clone()));
				} else {
					panic!("Cannot deal with unknown");
				}
				x += 1;
			}
		}
		return (TwoDMap::from_map(cell_map), curr_lcs);
	}

	fn can_move_dir(cell_map: &Map<Coords, CellType>, coord: &Coords, direction: Direction) -> bool {
		match cell_map.get(&coord.move_direction(direction)) {
			None => false,
			Some(ct) => {
				match ct {
					CellType::NormalBarrier => false,
					_ => true
				}
			}
		}
	}

	pub fn from_map(cell_map: Map<Coords, CellType>) -> TwoDMap {
		//OK, need to work out the intersections!
		let mut intersections: Map<Coords, Vec<IntersectionTarget>> = Map::new();
		let mut branched: Set<Coords> = Set::new();
		for (coord, celltype) in &cell_map {
			match celltype {
				CellType::NormalBarrier => {}
				_ => {
					let can_move_north = TwoDMap::can_move_dir(&cell_map, coord, Direction::North);
					let can_move_south = TwoDMap::can_move_dir(&cell_map, coord, Direction::South);
					let can_move_east = TwoDMap::can_move_dir(&cell_map, coord, Direction::East);
					let can_move_west = TwoDMap::can_move_dir(&cell_map, coord, Direction::West);
					if can_move_north {
						if can_move_south {
							if can_move_east || can_move_west {
								branched.insert(coord.clone());
							}
						} else {
							if can_move_east && can_move_west {
								branched.insert(coord.clone());
							}
						}
					} else {}
				}
			}
		}
		//Ok, so in theory every one of these branched would end up at at least one other point?
		//So we need to find ones in which the x's are equal OR the y's are equal AND there is no wall between?
		for coord in branched {
			let mut places: Vec<IntersectionTarget> = Vec::new();
			[Direction::North, Direction::South, Direction::East, Direction::West]
				.iter()
				.for_each(|dir_b| {
					let dir = *dir_b;
					//Try and travel this way at least once
					let mut trav = coord.move_direction(dir);
					match cell_map.get(&trav) {
						None => {}
						Some(e) => {
							match e {
								CellType::NormalBarrier => {}
								_ => {
									//OK, we can actually do things with this now!
									//Keep travelling until we hit a wall (or not exist) and then record that in the vec!
									'lp: loop {
										trav = trav.move_direction(dir);
										match cell_map.get(&trav) {
											None => { break 'lp; }
											Some(e) => {
												match e {
													CellType::NormalBarrier => { break 'lp; }
													_ => {
														continue 'lp;
													}
												}
											}
										}
									}
									trav = trav.move_away(dir);
									if trav.x != coord.x || trav.y != coord.y {
										places.push(IntersectionTarget {
											direction: dir,
											target: trav,
											distance: trav.flat_dist(&coord),
										})
									}
								}
							}
						}
					}
				});
			if places.len() > 1 {
				intersections
					.insert(coord, places);
			}
		}
		TwoDMap::new(cell_map, intersections)
	}

	pub fn new(cell_map: Map<Coords, CellType>, intersections: Map<Coords, Vec<IntersectionTarget>>) -> TwoDMap {
		TwoDMap {
			cell_map,
			intersections,
		}
	}
}