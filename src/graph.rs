use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, BTreeSet, VecDeque};
use std::ops::{AddAssign, SubAssign};

use itertools::Itertools;

use crate::cell::CellType;
use crate::coords::Coords;
use crate::direction::Direction;
use crate::map::TwoDMap;

type Map<A, B> = BTreeMap<A, B>;
type Set<A> = BTreeSet<A>;

#[derive(Clone, Copy, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub enum GraphType {
	Barrier(char),
	Goal(char),
	Start(char),
	Warp(char, char, bool, usize),
}

#[derive(Debug)]
pub struct Graph {
	graph: Map<GraphType, Map<GraphType, usize>>,
	goal_count: usize,
	barrier_count: usize,
	start_count: usize,
}

#[derive(PartialEq, Eq, Debug)]
struct TraversalState {
	steps: usize,
	robots: Vec<(GraphType, usize)>,
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
				if edges.len() == 0 {
					panic!("Disconnected node {:?}", coords);
				}
				graph.insert(GraphType::Goal(*ch), edges);
			} else if let CellType::Start(ch) = cell {
				let edges = Graph::find_edges_for(&map.cell_map, *coords);
				if edges.len() == 0 {
					panic!("Disconnected node {:?}", coords);
				}
				graph.insert(GraphType::Start(*ch), edges);
			} else if let CellType::SpecialBarrier(ch) = cell {
				let edges = Graph::find_edges_for(&map.cell_map, *coords);
				if edges.len() == 0 {
					panic!("Disconnected node {:?}", coords);
				}
				graph.insert(GraphType::Barrier(*ch), edges);
			} else if let CellType::WarpInner(ix) = cell {
				let mut edges = Graph::find_edges_for(&map.cell_map, *coords);
				if edges.len() == 0 {
					panic!("Disconnected node {:?}", coords);
				}
				let calc = *ix;
				edges.insert(GraphType::Warp((calc / 100) as u8 as char, (calc % 100) as u8 as char, false, 0), 1);
				graph.insert(GraphType::Warp((calc / 100) as u8 as char, (calc % 100) as u8 as char, true, 0), edges);
			} else if let CellType::WarpOuter(ix) = cell {
				let mut edges = Graph::find_edges_for(&map.cell_map, *coords);
				if edges.len() == 0 {
					panic!("Disconnected node {:?}", coords);
				}
				let calc = *ix;
				edges.insert(GraphType::Warp((calc / 100) as u8 as char, (calc % 100) as u8 as char, true, 0), 1);
				graph.insert(GraphType::Warp((calc / 100) as u8 as char, (calc % 100) as u8 as char, false, 0), edges);
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
		let mut queue: VecDeque<(Coords, usize, Option<Direction>)> = VecDeque::new();
		queue.push_back((coords, 0, Option::None));
		while let Some((curr, steps, _)) = queue.pop_front() {
			let adj = [
				(curr.move_direction(Direction::North), Direction::North),
				(curr.move_direction(Direction::South), Direction::South),
				(curr.move_direction(Direction::East), Direction::East),
				(curr.move_direction(Direction::West), Direction::West),
			];
			for (next, nj) in adj {
				if let Some(kind) = map.get(&next) {
					if !seen.contains(&next) {
						seen.insert(next);
						match kind {
							CellType::SpecialBarrier(ch) => {
								retme.insert(GraphType::Barrier(*ch), steps + 1);
							}
							CellType::Goal(ch) => {
								retme.insert(GraphType::Goal(*ch), steps + 1);
							}
							CellType::Start(ch) => {
								retme.insert(GraphType::Start(*ch), steps + 1);
							}
							CellType::WarpInner(ix) => {
								let calc = *ix;
								retme.insert(GraphType::Warp((calc / 100) as u8 as char, (calc % 100) as u8 as char, true, 0), steps + 1);
							}
							CellType::WarpOuter(ix) => {
								let calc = *ix;
								retme.insert(GraphType::Warp((calc / 100) as u8 as char, (calc % 100) as u8 as char, false, 0), steps + 1);
							}
							CellType::Space => {
								queue.push_back((next, steps + 1, Option::Some(nj)));
							}
							_ => {}
						}
					}
				}
			}
		}

		return retme;
	}

	pub fn make_deep(self, depth: usize) -> Graph{
		//OK, we need to turn all the inner ones to start mapping from 0 -> themselves at 1 on outer
		let mut ig: Map<GraphType, Map<GraphType, usize>> = Map::new();
		self.graph.keys()
			.for_each(|k|{
				match k {
					GraphType::Warp(chA, chB, is_down, _) => {
						if *is_down {
							//OK, so these can exist at 0 but not to the depth entry
							let edges = self.graph.get(k).unwrap();
							for my_depth in 0..depth{
								let my_type = GraphType::Warp(chA.clone(), chB.clone(), is_down.clone(), my_depth);
								let mut my_edges: Map<GraphType, usize> = Map::new();
								edges
									.iter()
									.for_each(|(&wasType, &wasDist)|{
										match wasType {
											GraphType::Goal(_) if my_depth == 0 => {
												my_edges.insert(wasType, wasDist);
											}
											GraphType::Start(_) if my_depth == 0 => {
												my_edges.insert(wasType, wasDist);
											}
											GraphType::Warp(chC, chD, oth_is_down, _) => {
												if (oth_is_down && my_depth != (depth -1)) || chC != *chA || chD != *chB {
													//OK, same but with my depth
													let then_type = GraphType::Warp(chC, chD, oth_is_down, my_depth);
													my_edges.insert(then_type, wasDist);
												}else if wasDist == 1{
													//OK, link to the other me but like, down 1
													let then_type = GraphType::Warp(chA.clone(), chB.clone(), !(is_down.clone()), if *is_down {my_depth + 1}else{my_depth - 1});
													my_edges.insert(then_type, 1);
												}else if wasDist == 2 {
													//
												}else{
													panic!("Uh, no idea what to do with {:?}", wasType);
												}
											}
											_ => {}
										}
									});
								ig.insert(my_type, my_edges);
							}
						}else{
							//OK, so up cannot exist at 0 but can exist at the depth level
							let edges = self.graph.get(k).unwrap();
							for my_depth in 1..=depth{
								let my_type = GraphType::Warp(chA.clone(), chB.clone(), is_down.clone(), my_depth);
								let mut my_edges: Map<GraphType, usize> = Map::new();
								edges
									.iter()
									.for_each(|(&wasType, &wasDist)|{
										match wasType {
											GraphType::Warp(chC, chD, oth_is_down, _) => {
												if oth_is_down || chC != *chA || chD != *chB {
													//OK, same but with my depth
													let then_type = GraphType::Warp(chC, chD, oth_is_down, my_depth);
													my_edges.insert(then_type, wasDist);
												}else if wasDist == 1{
													//OK, link to the other me but like, down 1
													let then_type = GraphType::Warp(chA.clone(), chB.clone(), !(is_down.clone()), if *is_down {my_depth + 1}else{my_depth - 1});
													my_edges.insert(then_type, 1);
												}else if wasDist == 2 {
														//
													}
												else{
													panic!("Uh, no idea what to do with {:?}", wasType);
												}
											}
											_ => {}
										}
									});
								ig.insert(my_type, my_edges);
							}
						}
					},
					_ => {
						ig.insert(k.clone(), Map::new());
						self.graph.get(k)
							.unwrap()
							.iter()
							.for_each(|(innerType, innerSize)| {
								ig.get_mut(&k.clone())
									.unwrap()
									.insert(innerType.clone(), innerSize.clone());
							});
					}
				}
			});
		return Graph{
			graph: ig,
			goal_count: 1,
			start_count: 1,
			barrier_count: 0,
		}
	}

	pub fn print(&self) {
		self.graph
			.iter()
			.for_each(|(&key, values)| {
				print!("{:?} ->\t", key);
				values
					.iter()
					.for_each(|(&kind, &dist)| {
						print!("{:?} - {:?},\t", kind, dist);
					});
				println!("");
			});
	}

	pub fn traverse(self) -> usize {
		return self.traverse_deep(false);
	}

	//Something similar to Dijkstra?
	pub fn traverse_deep(&self, depth_track: bool) -> usize {
		let mut queue = BinaryHeap::new();
		let robots = self.graph
			.keys()
			.filter(|f| match f {
				GraphType::Start(_) => true,
				_ => false
			})
			.map(|f| (*f, 0))
			.collect_vec();

		let start = TraversalState {
			steps: 0,
			robots,
			visited: Set::new(),
		};

		let mut weights: Map<(Vec<(GraphType, usize)>, Set<GraphType>), usize> = Map::new();

		queue.push(start);

		'queue: while let Some(current) = queue.pop() {
			let curr_goals_seen_count = current.goals_met().len();
			if curr_goals_seen_count == self.goal_count {
				return current.steps;
			} else if curr_goals_seen_count == self.goal_count {
				panic!("Cannot finish maze yet not on level 0?");
			}
			if let Some(&best_seen) = weights.get(&(current.robots.clone(), current.visited.clone())) {
				if current.steps > best_seen {
					continue;
				}
			}
			'robots: for (number, &(robo_tile, curr_depth)) in current.robots.iter().enumerate() {
				let mut seen_weights: Map<GraphType, usize> = Map::new();
				for &coords in self.graph.keys() {
					seen_weights.insert(coords, usize::MAX);
				}
				let mut heap: BinaryHeap<(usize, GraphType, usize)> = BinaryHeap::new();
				*seen_weights.get_mut(&current.robots[number].0).unwrap() = 0;
				heap.push((0_usize, current.robots[number].0, curr_depth));
				let mut possible_keys: Set<GraphType> = Set::new();
				'heappop: while let Some((current_steps, current_tile, curr_depth)) = heap.pop() {
					match current_tile {
						GraphType::Goal(_) => {
							if !current.visited.contains(&current_tile) {
								//OK, let's work out the depth here!
								if depth_track && curr_depth != 0 {
									continue 'heappop;
								}
								let ct = current_tile.clone();
								possible_keys.insert(ct);
							}
						}
						_ => {}
					}
					if current_steps > seen_weights[&current_tile] {
						continue 'heappop;
					}
					let mut curr_tile_map_op = self.graph.get(&current_tile);
					let empty_map: Map<GraphType, usize> = Map::new();
					if let None = curr_tile_map_op{
						curr_tile_map_op = Option::Some(&empty_map);
					}
					'graph_ctile: for (&next, &steps) in curr_tile_map_op.unwrap().iter() {
						let mut ndepth: usize = curr_depth.clone();
						match next {
							GraphType::Barrier(v) => {
								if !current
									.visited
									.contains(&GraphType::Goal(v.to_ascii_lowercase()))
								{
									//Can't cross this barrier I think
									continue 'graph_ctile;
								}
							}
							GraphType::Goal(_) => {
								if depth_track && ndepth != 0 {
									//No
									continue 'graph_ctile;
								}
							}
							GraphType::Start(_) => {
								continue 'graph_ctile;
							}
							GraphType::Warp(_, _, down, num) => {
								match current_tile {
									GraphType::Warp(_, _, lmao, num) if lmao != down => {}
									_ => {
										if depth_track && !down && ndepth == 0 {
											continue 'graph_ctile;
										} else if depth_track {
											if down {
												ndepth.add_assign(1)
											} else {
												ndepth.sub_assign(1);
											}
										}
									}
								}
							}
						}
						let pair = (current_steps + steps, next, ndepth);
						if seen_weights.get(&next).is_none() {
							seen_weights.insert(next, usize::MAX);
						}
						if pair.0 < seen_weights[&next] {
							seen_weights.insert(next, pair.0);
							heap.push(pair);
						}
					}
				}
				if possible_keys.len() == 0 {
					println!("No possible solutions for {:?}", current);
				}
				let real_keys: Vec<(GraphType, usize)> = possible_keys.into_iter()
					.map(|node| (node, seen_weights[&node]))
					.collect();
				'rkeyloop: for &(next_node, steps) in real_keys.iter() {
					let mut new_visited: Set<GraphType> = current.visited.clone();
					let mut new_robots = current.robots.clone();
					match next_node {
						GraphType::Warp(_, _, down, num) => {
							match robo_tile {
								GraphType::Warp(_, _, lmao, num) if lmao != down => {}
								_ => {
									if depth_track {
										if curr_depth == 0 && !down {
											continue 'rkeyloop;
										} else {
											if down {
												new_robots[number] = (next_node, curr_depth + 1);
											} else {
												new_robots[number] = (next_node, curr_depth - 1);
											}
										}
									} else {
										new_robots[number] = (next_node, 0);
									}
								}
							}
							
						}
						GraphType::Start(_) => {
							continue 'rkeyloop;
						}
						GraphType::Goal(_) => {
							if depth_track && curr_depth != 0 {
								continue 'rkeyloop;
							} else {
								new_robots[number] = (next_node, 0);
							}
						}
						_ => {
							new_robots[number] = (next_node, curr_depth);
						}
					}
					new_visited.insert(next_node);
					if new_robots[number].0 != next_node {
						new_visited.insert(new_robots[number].0);
					}
					let new_steps = current.steps + steps;
					let known = weights
						.entry((new_robots.clone(), new_visited.clone()))
						.or_insert(usize::MAX);
					if new_steps < *known {
						*known = new_steps;
						let new_state = TraversalState {
							visited: new_visited,
							robots: new_robots,
							steps: new_steps,
						};
						queue.push(new_state);
					}
				}
			}
		}
		usize::MAX
	}
}