use crate::SinglePart;
use rayon::prelude::*;
pub struct Code;

pub const FL_CONT: &str = include_str!("../../inputs/2019/Day10.txt");
#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Quadrant {
	N,
	NE,
	E,
	SE,
	S,
	SW,
	W,
	NW
}

#[derive(Clone, Debug)]
struct Asteroid {
	xcoord: u16,
	ycoord: u16,
}

#[derive(Debug, Clone)]
struct RelAsteroid {
	relx: i16,
	rely: i16,
	ox: u16,
	oy: u16,
	dist: f64,
	quad: Quadrant,
	angle: f64,
}

impl Asteroid {
	fn to_rel(&self, origin: &Asteroid) -> RelAsteroid {
		//This is, origin is now x=0, y=0
		let relx: i16 = ((self.xcoord as i32) - (origin.xcoord as i32)) as i16;
		let rely: i16 = ((self.ycoord as i32) - (origin.ycoord as i32)) as i16;
		let frelx = relx as f64;
		let frely = rely as f64;
		let dist: f64 = ((frelx * frelx) + (frely * frely)).sqrt();
		let angle: f64 = frely.atan2(frelx) * (180.0 / std::f64::consts::PI);
		let quad: Quadrant =
			if relx == 0 {
				if rely > 0 {
					Quadrant::S
				}else {
					Quadrant::N
				}
			}else if rely == 0 {
				if relx > 0 {
					Quadrant::W
				}else {
					Quadrant::E
				}
			}else {
				if relx > 0 && rely > 0 {
					Quadrant::SE
				} else if relx > 0 {
					Quadrant::NE
				} else if rely > 0 {
					Quadrant::SW
				}else{
					Quadrant::NW
				}
			};
		return RelAsteroid {
			relx,
			ox: self.xcoord,
			rely,
			oy: self.ycoord,
			dist,
			quad,
			angle
		};
	}
}

fn variant_eq<T>(a: &T, b: &T) -> bool {
	std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn create_map(input: &str) -> Vec<Asteroid> {
	let mut map: Vec<Asteroid> = Vec::new();
	for (ycoord, line) in input.lines().map(|x| x.split("")).enumerate() {
		for (xcoord, token) in line.enumerate() {
			if token.eq("#") {
				map.push(Asteroid {
					xcoord: (xcoord - 1) as u16,
					ycoord: ycoord as u16,
				})
			}
		}
	}
	return map;
}

fn to_rel_map(map: Vec<Asteroid>, origin: &Asteroid) -> Vec<RelAsteroid> {
	let mut relmap: Vec<RelAsteroid> = map
		.iter()
		.filter(|other| !(other.ycoord == origin.ycoord && other.xcoord == origin.xcoord))
		.map(|x| x.to_rel(origin))
		.collect();
	relmap.sort_unstable_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap_or(a.relx.cmp(&b.relx)));
	return relmap;
}

fn colinear(mn: &RelAsteroid, xy: &RelAsteroid, ab: Option<&RelAsteroid>) -> bool {
	let a = ab.map(|pt| pt.relx).unwrap_or(0);
	let b = ab.map(|pt| pt.rely).unwrap_or(0);
	let m = mn.relx;
	let n = mn.rely;
	let x = xy.relx;
	let y = xy.rely;
	(n - b) * (x - m) == (y - n) * (m - a)
}

fn observable_asteroids(map: Vec<RelAsteroid>) -> Vec<RelAsteroid> {
	let mut seen: Vec<RelAsteroid> = Vec::new();
	for eval in map.into_iter() {
		match seen
			.iter()
			.find(|obs| variant_eq(&obs.quad, &eval.quad) && colinear(obs, &eval, Option::None))
		{
			Some(_x) => {
				//We have an item that's same Directional and we'd hide behind them, so...
			}
			None => {
				seen.push(eval);
			}
		}
	}
	return seen;
}

fn find_best_asteroid(map: Vec<Asteroid>) -> (Asteroid, Vec<RelAsteroid>) {
	map.iter()
		.enumerate()
		.par_bridge()
		.map(|x| -> (Asteroid, Vec<RelAsteroid>) {
			(
				x.1.clone(),
				observable_asteroids(to_rel_map(map.clone(), x.1)),
			)
		})
		.max_by(|a, b| a.1.len().cmp(&b.1.len()))
		.unwrap()
}

fn part_2_solver(map: Vec<Asteroid>, part_a_asteroid: &Asteroid) -> usize {
	let mut rebased_asteroids: Vec<RelAsteroid> = to_rel_map(map, &part_a_asteroid);
	let mut seeables: Vec<RelAsteroid> = Vec::new();
	loop {
		let mut currently_seeable = observable_asteroids(rebased_asteroids.clone());
		currently_seeable.sort_unstable_by(|a, b| {
			if variant_eq(&a.quad, &b.quad) {
				a.angle.partial_cmp(&b.angle).unwrap_or(a.relx.cmp(&b.relx))
			} else {
				a.quad.partial_cmp(&b.quad).unwrap_or(a.relx.cmp(&b.relx))
			}
		});
		rebased_asteroids.drain_filter(|eval| {
			match currently_seeable
				.iter()
				.find(|obs| obs.ox == eval.ox && obs.oy == eval.oy)
			{
				Some(_x) => true,
				None => false,
			}
		});
		seeables.append(&mut currently_seeable);
		if rebased_asteroids.len() == 0 {
			break;
		}
	}
	let num200 = seeables.get(199).unwrap();
	return (num200.ox as usize * 100) + (num200.oy as usize);
}

pub fn solve(input: &str) -> String {
	let map: Vec<Asteroid> = create_map(input);
	let aster_mp = find_best_asteroid(map.clone());
	let part_a_answer: usize = aster_mp.1.len();
	//OK, so now we can routinely eliminate asteroids until we find the 200th!

	let fmt = format!(
		"Part 1:\t{}\nPart 2:\t{}\n",
		part_a_answer,
		part_2_solver(map, &aster_mp.0)
	);
	return fmt;
}

impl SinglePart for Code {
	fn run(&self) -> String {
		solve(FL_CONT)
	}
}
#[cfg(test)]
mod tests {
	use super::*;

	const STR_A: &str = include_str!("../../inputs/2019/Day10-eg-a.txt");
	const STR_B: &str = include_str!("../../inputs/2019/Day10-eg-b.txt");
	const STR_C: &str = include_str!("../../inputs/2019/Day10-eg-c.txt");
	const STR_D: &str = include_str!("../../inputs/2019/Day10-eg-d.txt");
	const STR_E: &str = include_str!("../../inputs/2019/Day10-eg-e.txt");

	#[test]
	fn test_example_a() {
		let map: Vec<Asteroid> = create_map(STR_A);
		let aster_mp = find_best_asteroid(map);
		let aster: Asteroid = aster_mp.0;
		assert_eq!(aster.xcoord, 3);
		assert_eq!(aster.ycoord, 4);
		assert_eq!(aster_mp.1.len(), 8);
	}

	#[test]
	fn test_example_b() {
		let map: Vec<Asteroid> = create_map(STR_B);
		let aster_mp = find_best_asteroid(map);
		let aster: Asteroid = aster_mp.0;
		assert_eq!(aster.xcoord, 5);
		assert_eq!(aster.ycoord, 8);
		assert_eq!(aster_mp.1.len(), 33);
	}

	#[test]
	fn test_example_c() {
		let map: Vec<Asteroid> = create_map(STR_C);
		let aster_mp = find_best_asteroid(map);
		let aster: Asteroid = aster_mp.0;
		assert_eq!(aster.xcoord, 1);
		assert_eq!(aster.ycoord, 2);
		assert_eq!(aster_mp.1.len(), 35);
	}

	#[test]
	fn test_example_d() {
		let map: Vec<Asteroid> = create_map(STR_D);
		let aster_mp = find_best_asteroid(map);
		let aster: Asteroid = aster_mp.0;
		assert_eq!(aster.xcoord, 6);
		assert_eq!(aster.ycoord, 3);
		assert_eq!(aster_mp.1.len(), 41);
	}

	#[test]
	fn test_example_e() {
		let map: Vec<Asteroid> = create_map(STR_E);
		let aster_mp = find_best_asteroid(map.clone());
		let aster: &Asteroid = &aster_mp.0;
		assert_eq!(aster.xcoord, 11);
		assert_eq!(aster.ycoord, 13);
		assert_eq!(aster_mp.1.len(), 210);
		assert_eq!(part_2_solver(map, &aster_mp.0), 802);
	}
}
