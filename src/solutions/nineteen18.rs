use crate::AoCDay;
use crate::map::{TwoDMap, Graph, CellType};
use crate::coords::Coords;
use crate::direction::Direction::{North, East, South, West};

pub struct Code;

pub const DAY_18_DATA: &str = include_str!("../../inputs/2019/Day18.txt");
pub const DAY_18_P2_DATA: &str = include_str!("../../inputs/2019/Day18-2.txt");

fn parse_maze(input: &str) -> (TwoDMap, Vec<Coords>) {
	let uppercase = (b'A'..=b'Z')
		.map(|c| c as char)
		.filter(|c| c.is_alphabetic())
		.collect::<Vec<_>>();
	let lowercase = (b'a'..=b'z')
		.map(|c| c as char)
		.filter(|c| c.is_alphabetic())
		.collect::<Vec<_>>();
	TwoDMap::from_str(
		input,
		'.',
		'#',
		uppercase,
		lowercase,
		Vec::from(['@','1','2','3','4'])
	)
}

impl AoCDay for Code{
	fn part1(&self) -> String {
		let maze = parse_maze(DAY_18_DATA);
		let graph = Graph::from_two_d_map(maze.0);
		graph.traverse().to_string()
	}

	fn part2(&self) -> String {
		let maze = parse_maze(DAY_18_P2_DATA);
		let graph = Graph::from_two_d_map(maze.0);
		graph.traverse().to_string()
	}
}