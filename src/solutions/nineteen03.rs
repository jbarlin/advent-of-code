use std::collections::{HashMap};

use crate::{SinglePart};

const FL_CONT: &str = include_str!("../../inputs/2019/Day3.txt");
pub struct Code;

impl SinglePart for Code{
	fn run(&self) -> String {
        let file_content: String = FL_CONT.to_string();
		let res = action_str(file_content);
        //return "Part 1\t" + res[0].to_string() + "\nPart 2\t" + res[1].to_string();
		let fmt = format!("Part 1:\t{}\nPart 2:\t{}\n",res.0, res.1);
		return fmt;
    }
}

type NUMBER = i64;
type Point = (NUMBER, NUMBER);

fn manhatten_dist(a: &Point, b: &Point) -> u64{
	return ((a.0 - b.0).abs() as u64) + ((a.1 - b.1).abs() as u64);
}

fn action_str(input: String) -> (u64, u64){
	let mut results: HashMap<Point, [Option<u64>; 2]> = HashMap::new();
	let mut curr_line = 0;
	for line in input.lines(){
		let mut line_point_at: Point = (0,0);
		let mut steps: u64 = 1;

		for seg in line.trim().split(",").map(|s| s.trim()){
			let direction: (NUMBER,NUMBER) =  match seg.chars().next().unwrap() {
				'U' => (0,1),
				'D' => (0, -1),
				'L' => (-1,0),
				'R' => (1,0),
				_ => unreachable!()
			};

			let distance: NUMBER = seg[1..].parse().unwrap();

			let destination: Point = line_point_at;
			let mut c_point:Point = line_point_at;

			for i in 1..=distance {
				c_point = (destination.0 + (i * direction.0), destination.1 + (i * direction.1));

				let value = results.entry(c_point).or_insert_with(|| [None, None]);

				if value[curr_line].is_none() {
					value[curr_line] = Some(steps);
				}
				steps += 1
			}

			line_point_at = c_point;
		}
		curr_line += 1;
	}

	return results
	.iter()
	.filter(|(_,v)| v[0].is_some() && v[1].is_some())
	.fold((std::u64::MAX, std::u64::MAX), |(min_dist, min_steps),(point, steppy)| {
		let cdist = manhatten_dist(&(0,0), point);
		let ret_dist = if min_dist < cdist {min_dist} else {cdist};
		let csteps = steppy[0].unwrap() + steppy[1].unwrap();
		let ret_steps = if min_steps < csteps {min_steps} else {csteps};
		return (ret_dist, ret_steps);
	});
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_examples_part_a_first(){
		assert_eq!((6,30), action_str("R8,U5,L5,D3\nU7,R6,D4,L4".to_string()));
    }
	#[test]
    fn test_examples_part_a_second(){
		assert_eq!((159,610), action_str("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83".to_string()));
    }
	#[test]
    fn test_examples_part_a_third(){
		assert_eq!((135,410), action_str("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string()));
    }
}