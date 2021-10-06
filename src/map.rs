use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, BTreeSet, VecDeque};

use itertools::Itertools;

use crate::coords::Coords;
use crate::direction::Direction;
use crate::intcode::NumType;

type Map<A, B> = BTreeMap<A, B>;
type Set<A> = BTreeSet<A>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellType {
	Space,
	NormalBarrier,
	SpecialBarrier(char),
	Goal(char),
	Start(char),
}

#[derive(Clone, Copy, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub enum GraphType {
	Barrier(char),
	Goal(char),
	Start(char),
}

impl CellType {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntersectionTarget {
	pub direction: Direction,
	pub target: Coords,
	pub distance: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoDMap {
	cell_map: Map<Coords, CellType>,
	intersections: Map<Coords, Vec<IntersectionTarget>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Path {
	pub path: Vec<Coords>,
	nodes_visited: Map<Coords, usize>,
	collected_goals: Vec<char>,
}

pub struct Graph {
	graph: Map<GraphType, Map<GraphType, usize>>,
	goal_count: usize,
	barrier_count: usize,
	start_count: usize,
}

#[derive(PartialEq, Eq)]
struct TraversalState {
	steps: usize,
	robots: Vec<GraphType>,
	visited: Set<GraphType>,
}

impl TraversalState {
	fn goals_met(&self) -> Set<&GraphType> {
		self.visited.iter().filter(|f| match f {
			GraphType::Goal(_) => true,
			_ => false
		}).collect()
	}
}

impl Ord for TraversalState {
	fn cmp(&self, other: &Self) -> Ordering {
		other
			.steps
			.cmp(&self.steps)
			.then(self.visited.len().cmp(&other.visited.len()))
	}
}

impl PartialOrd for TraversalState {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Graph {
	pub fn from_two_d_map(map: TwoDMap) -> Graph {
		let mut graph: Map<GraphType, Map<GraphType, usize>> = Map::new();
		for (coords, cell) in map.cell_map.iter() {
			if let CellType::Goal(ch) = cell {
				let edges = Graph::find_edges_for(&map.cell_map, *coords);
				graph.insert(GraphType::Goal(*ch), edges);
			} else if let CellType::Start(ch) = cell {
				let edges = Graph::find_edges_for(&map.cell_map, *coords);
				graph.insert(GraphType::Start(*ch), edges);
			} else if let CellType::SpecialBarrier(ch) = cell {
				let edges = Graph::find_edges_for(&map.cell_map, *coords);
				graph.insert(GraphType::Barrier(*ch), edges);
			}
		}
		let goal_count = map.cell_map.iter().filter(|f| match (*f).1 {
			CellType::Goal(_) => true,
			_ => false
		}).count();
		let start_count = map.cell_map.iter().filter(|f| match (*f).1 {
			CellType::Start(_) => true,
			_ => false
		}).count();
		let barrier_count = map.cell_map.iter().filter(|f| match (*f).1 {
			CellType::SpecialBarrier(_) => true,
			_ => false
		}).count();
		return Graph {
			graph,
			goal_count,
			barrier_count,
			start_count,
		};
	}
	fn find_edges_for(map: &Map<Coords, CellType>, coords: Coords) -> Map<GraphType, usize> {
		//So basically, we want a map from here to all the other nodes we can hit that are of any interest
		let mut seen: Set<Coords> = Set::new();
		let mut retme: Map<GraphType, usize> = Map::new();
		let mut queue: VecDeque<(Coords, usize)> = VecDeque::new();
		queue.push_back((coords, 0));
		while let Some((curr, steps)) = queue.pop_front() {
			let adj = [
				curr.move_direction(Direction::North),
				curr.move_direction(Direction::South),
				curr.move_direction(Direction::East),
				curr.move_direction(Direction::West),
			];
			for next in adj {
				if let Some(kind) = map.get(&next) {
					if !seen.contains(&next) {
						seen.insert(next);
						match kind {
							CellType::Space => {
								queue.push_back((next, steps + 1));
							}
							CellType::SpecialBarrier(ch) => {
								retme.insert(GraphType::Barrier(*ch), steps + 1);
							}
							CellType::Goal(ch) => {
								retme.insert(GraphType::Goal(*ch), steps + 1);
							}
							CellType::Start(ch) => {
								retme.insert(GraphType::Start(*ch), steps + 1);
							}
							_ => {}
						}
					}
				}
			}
		}

		return retme;
	}

	//Something similar to Dijkstra?
	pub fn traverse(self) -> usize {
		let mut queue = BinaryHeap::new();

		let robots = self.graph
			.keys()
			.filter(|f| match f {
				GraphType::Start(_) => true,
				_ => false
			})
			.map(|f| *f)
			.collect_vec();

		let start = TraversalState {
			steps: 0,
			robots,
			visited: Set::new()
		};

		let mut weights: Map<(Vec<GraphType>, Set<GraphType>), usize> = Map::new();

		queue.push(start);

		while let Some(current) = queue.pop(){
			if current.goals_met().len() == self.goal_count {
				return current.steps;
			}
			if let Some(&best_seen) = weights.get(&(current.robots.clone(), current.visited.clone())){
				if current.steps > best_seen {
					continue;
				}
			}
			for (number, _) in current.robots.iter().enumerate(){
				let mut seen_weights: Map<GraphType, usize> = Map::new();
				for &coords in self.graph.keys(){
					seen_weights.insert(coords, usize::MAX);
				}
				let mut heap: BinaryHeap<(usize, GraphType)> = BinaryHeap::new();
				*seen_weights.get_mut(&current.robots[number]).unwrap() = 0;
				heap.push((0_usize, current.robots[number]));
				let mut possible_keys: Set<GraphType> = Set::new();
				while let Some((current_steps, current_tile)) = heap.pop(){
					match current_tile {
						GraphType::Goal(_) => {
							if !current.visited.contains(&current_tile) {
								let ct = current_tile.clone();
								possible_keys.insert(ct);
							}
						}
						_ => {}
					}
					if current_steps > seen_weights[&current_tile]{
						continue;
					}
					for (&next, &steps) in self.graph.get(&current_tile).unwrap().iter(){
						match next {
							GraphType::Barrier(v) => {
								if !current
									.visited
									.contains(&GraphType::Goal(v.to_ascii_lowercase()))
								{
									//Can't cross this barrier I think
									continue;
								}
							},
							_ => {}
						}
						let pair = (current_steps + steps, next);
						if pair.0 < seen_weights[&next]{
							seen_weights.insert(next, pair.0);
							heap.push(pair);
						}
					}
				}
				let real_keys: Vec<(GraphType, usize)> = possible_keys.into_iter()
					.map(|node| (node, seen_weights[&node]))
					.collect();
				for &(next_node, steps) in real_keys.iter(){
					let mut new_visited: Set<GraphType> = current.visited.clone();
					new_visited.insert(next_node);
					let mut new_robots = current.robots.clone();
					new_robots[number] = next_node;
					let new_steps = current.steps + steps;
					let known = weights
						.entry((new_robots.clone(), new_visited.clone()))
						.or_insert(usize::MAX);
					if new_steps < *known{
						*known = new_steps;
						let new_state = TraversalState{
							visited: new_visited,
							robots: new_robots,
							steps: new_steps
						};
						queue.push(new_state);
					}
				}
			}
		}
		usize::MAX
	}
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