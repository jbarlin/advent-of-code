use crate::AoCDay;
use crate::graph::Graph;
use crate::torus_map::Torus;

pub struct Code;

pub const DAY_20_DATA: &str = include_str!("../../inputs/2019/Day20.txt");

impl AoCDay for Code{
	fn part1(&self) -> String {
		let map = Torus::str_to_map(DAY_20_DATA);
		let graph = Graph::from_two_d_map(map);
		graph.traverse_deep(false).to_string()
	}
	fn part2(&self) -> String {
		let map = Torus::str_to_map(DAY_20_DATA);
		let graph = Graph::from_two_d_map(map);
		graph.traverse_deep(true).to_string()
	}
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn test_pt1_a(){
		let map = Torus::str_to_map(include_str!("../../inputs/2019/Day20-eg-a.txt"));
		let graph = Graph::from_two_d_map(map);
		assert_eq!(graph.traverse_deep(false), 23);
	}

	#[test]
	fn test_pt1_b(){
		let map = Torus::str_to_map(include_str!("../../inputs/2019/Day20-eg-b.txt"));
		let graph = Graph::from_two_d_map(map);
		assert_eq!(graph.traverse_deep(false), 58);
	}

	#[test]
	fn test_pt2_c(){
		let map = Torus::str_to_map(include_str!("../../inputs/2019/Day20-eg-c.txt"));
		let graph = Graph::from_two_d_map(map);
		assert_eq!(graph.traverse_deep(true), 396);
	}
}