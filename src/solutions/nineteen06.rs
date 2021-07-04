use std::{collections::HashMap, str};

use crate::SinglePart;

pub struct Code;

pub const FL_CONT: &str = include_str!("../../inputs/2019/Day6.txt");

enum SpaceThing<'a>{
	COM,
	SAN,
	YOU,
	Planet(&'a str)
}

impl<'a> SpaceThing<'a> {
    fn from_str(s: &'a str) -> Self {
        if s == "COM" {
            Self::COM
        } else if s == "SAN" {
            Self::SAN
        }else if s == "YOU" {
            Self::YOU
        } else {
            Self::Planet(s)
        }
    }
}

struct Orbit<'a> {
    orbited: SpaceThing<'a>,
    orbiter: SpaceThing<'a>,
}

type OGraph<'a> = HashMap<&'a str, SpaceThing<'a>>;

impl SinglePart for Code{
	fn run(&self) -> String {
		let graph: OGraph = make_hashmap(parse_input(FL_CONT));
		let res: (u32, u32) = (count_orbits(&graph),count_path(&graph, "YOU", "SAN"));
		let fmt = format!("Part 1:\t{}\nPart 2:\t{}\n",res.0, res.1);
		return fmt;
    }
}

pub fn part_1_impl(input: &str) -> u32{
	let graph: OGraph = make_hashmap(parse_input(input));
	return count_orbits(&graph);
}

pub fn part_2_impl(input: &str) -> u32{
	let graph: OGraph = make_hashmap(parse_input(input));
	return count_path(&graph,"YOU","SAN");
}

fn path_to_com<'a>(graph: &OGraph<'a>, start: &'a str) -> Vec<&'a str> {
    let mut path = Vec::new();
    let mut stack = vec![&start];

    loop {
        match stack.pop() {
            None => return path,
            Some(orbiter) => match graph.get(*orbiter) {
                Some(SpaceThing::COM) => path.push("COM"),
				Some(SpaceThing::SAN) => path.push("SAN"),
				Some(SpaceThing::YOU) => path.push("YOU"),
                Some(SpaceThing::Planet(name)) => {
                    stack.push(name);
                    path.push(*name);
                }
                None => unreachable!(),
            },
        }
    }
}

fn count_path<'a>(graph: &OGraph<'a>, start: &'a str, end: &'a str) -> u32 {
    let start_to_com = path_to_com(graph, start);
    let end_to_com = path_to_com(graph, end);

    for (start_index, start_node) in start_to_com.iter().enumerate() {
        let end_index_optional = end_to_com.iter().position(|node| node == start_node);
        match end_index_optional {
            Some(end_index) => return (start_index + end_index) as u32,
            None => (),
        }
    }
    panic!("Cannot find path?")
}

fn parse_input(inp: &str) -> Vec<Orbit>{
	inp
	.split('\n')
	.filter(|s| s.len() > 0)
	.map(|str| pair_to_struct(str))
	.collect()
}

fn count_orbits(graph: &OGraph) -> u32 {
    let mut stack: Vec<&str> = graph.keys().map(|a| *a).collect();
    let mut orbits: u32 = 0;

    loop {
        match stack.pop() {
            None => break,
            Some(object) => match graph.get(object) {
                Some(SpaceThing::COM) => (),
                Some(SpaceThing::Planet(name)) => stack.push(&name),
				Some(SpaceThing::SAN) => stack.push("SAN"),
				Some(SpaceThing::YOU) => stack.push("YOU"),
                None => unreachable!(),
            },
        }
        orbits += 1
    }

    orbits
}

fn make_hashmap(input: Vec<Orbit>) -> OGraph{
	input.into_iter().map(|orbit|
        match orbit.orbiter {
            SpaceThing::Planet(name) => (name, orbit.orbited),
			SpaceThing::SAN => ("SAN", orbit.orbited),
			SpaceThing::YOU => ("YOU", orbit.orbited),
            SpaceThing::COM => unreachable!("COM Can't orbit stuff"),
        }
    ).collect::<OGraph>()
}

fn pair_to_struct(inp: &str) -> Orbit{
	let mut split = inp.split(')');
	Orbit{
		orbited: SpaceThing::from_str(split.next().unwrap()),
		orbiter: SpaceThing::from_str(split.next().unwrap())
	}
}


#[cfg(test)]
mod tests{
	use super::*;

	#[test]
    fn test_examples_part_a(){
		assert_eq!(42, part_1_impl(include_str!("../../inputs/2019/Day6-testa.txt")));
    }

	#[test]
    fn test_examples_part_b(){
		assert_eq!(4, part_2_impl(include_str!("../../inputs/2019/Day6-testb.txt")));
    }
}