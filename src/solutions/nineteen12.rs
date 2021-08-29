use std::cmp::Ordering;
use crate::AoCDay;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Axis {
	position: i64,
	velocity: i64,
}

fn velocity_change(a: i64, b: i64) -> i64 {
	match a.cmp(&b) {
		Ordering::Less => 1,
		Ordering::Equal => 0,
		Ordering::Greater => -1,
	}
}

#[derive(Debug, Clone, Copy)]
struct Moon {
	x: Axis,
	y: Axis,
	z: Axis,
}

impl Moon {

	fn tupled_new(positions: (i64, i64, i64)) -> Moon {
		return Moon {
			x: Axis {
				position: positions.0,
				velocity: 0,
			},
			y: Axis {
				position: positions.1,
				velocity: 0,
			},
			z: Axis {
				position: positions.2,
				velocity: 0,
			},
		};
	}

	fn edit_velocity(&mut self, other: &Moon) {
		self.x.velocity += velocity_change(self.x.position, other.x.position);
		self.y.velocity += velocity_change(self.y.position, other.y.position);
		self.z.velocity += velocity_change(self.z.position, other.z.position);
	}

	fn apply_velocity(&mut self) {
		self.x.position += self.x.velocity;
		self.y.position += self.y.velocity;
		self.z.position += self.z.velocity;
	}

	fn total_energy(&self) -> i64 {
		return (self.x.position.abs() + self.y.position.abs() + self.z.position.abs())
			* (self.x.velocity.abs() + self.y.velocity.abs() + self.z.velocity.abs());
	}
}

struct Jupiter{
	moons: Vec<Moon>
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

impl Jupiter{
	fn new(moon_a: (i64,i64,i64), moon_b: (i64,i64,i64), moon_c: (i64,i64,i64), moon_d: (i64,i64,i64)) -> Jupiter{
		let mut moons: Vec<Moon> = Vec::with_capacity(4);
		moons.push(Moon::tupled_new(moon_a));
		moons.push(Moon::tupled_new(moon_b));
		moons.push(Moon::tupled_new(moon_c));
		moons.push(Moon::tupled_new(moon_d));
		return Jupiter{ moons };
	}

	fn step(&mut self){
		for a in 0..self.moons.len() {
			for b in a + 1..self.moons.len() {
				let moon_a = self.moons[a];
				let moon_b = self.moons[b];
				self.moons[a].edit_velocity(&moon_b);
				self.moons[b].edit_velocity(&moon_a);
			}
		}
	
		for moon in self.moons.iter_mut() {
			moon.apply_velocity();
		}
	}

	fn count_energy(&self) -> i64 {
		self.moons.iter().map(|m| m.total_energy()).sum()
	}

	fn run_all_steps(&mut self, number_steps: usize) -> i64 {
		for _ in 0..number_steps{
			self.step();
		}
		return self.count_energy();
	}

	fn find_repeat_points(&mut self) -> (usize, usize, usize) {
		let (mut repeat_x, mut repeat_y, mut repeat_z) = (None, None, None);
		let start_x = self.moons.iter().map(|m| m.x).collect::<Vec<_>>();
		let start_y = self.moons.iter().map(|m| m.y).collect::<Vec<_>>();
		let start_z = self.moons.iter().map(|m| m.z).collect::<Vec<_>>();
		let mut steps = 0;
		while repeat_x.is_none() || repeat_y.is_none() || repeat_z.is_none() {
			steps += 1;
			self.step();
			if repeat_x.is_none(){
				let curr_x = self.moons.iter().map(|m| m.x).collect::<Vec<_>>();
				if curr_x == start_x {
					repeat_x = Some(steps);
				}
			}
			if repeat_y.is_none(){
				let curr_y = self.moons.iter().map(|m| m.y).collect::<Vec<_>>();
				if curr_y == start_y {
					repeat_y = Some(steps);
				}
			}
			if repeat_z.is_none(){
				let curr_z = self.moons.iter().map(|m| m.z).collect::<Vec<_>>();
				if curr_z == start_z {
					repeat_z = Some(steps);
				}
			}
		}
		(repeat_x.unwrap(), repeat_y.unwrap(), repeat_z.unwrap())
	}

	fn find_steps_till_repeat(&mut self) -> usize{
		let (rx, ry, rz) = self.find_repeat_points();
		return lcm(rx, lcm(ry, rz));
	}

}

pub fn part_1_impl(moon_a: (i64,i64,i64), moon_b: (i64,i64,i64), moon_c: (i64,i64,i64), moon_d: (i64,i64,i64)) -> i64{
	let mut jup: Jupiter = Jupiter::new(moon_a, moon_b, moon_c, moon_d);
	return jup.run_all_steps(1000);
}

pub fn part_2_impl(moon_a: (i64,i64,i64), moon_b: (i64,i64,i64), moon_c: (i64,i64,i64), moon_d: (i64,i64,i64)) -> usize{
	let mut jup: Jupiter = Jupiter::new(moon_a, moon_b, moon_c, moon_d);
	return jup.find_steps_till_repeat();
}

pub struct Code;
impl AoCDay for Code {
	fn part1(&self) -> String {
		return part_1_impl((-17,9,-5), (-1,7,13), (-19,12,5), (-6,-6,-4)).to_string();
	}
	fn part2(&self) -> String {
		return part_2_impl((-17,9,-5), (-1,7,13), (-19,12,5), (-6,-6,-4)).to_string();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_a_part_1(){
		let mut jup: Jupiter = Jupiter::new((-1,0,2), (2,-10,-7), (4,-8,8), (3,5,-1));
		assert_eq!(179, jup.run_all_steps(10));
	}

	#[test]
	fn test_a_part_2(){
		let mut jup: Jupiter = Jupiter::new((-1,0,2), (2,-10,-7), (4,-8,8), (3,5,-1));
		assert_eq!(2772, jup.find_steps_till_repeat());
	}

	#[test]
	fn test_b_part_1(){
		let mut jup: Jupiter = Jupiter::new((-8,-10,0), (5,5,10), (2,-7,3), (9,-8,-3));
		assert_eq!(1940, jup.run_all_steps(100));
	}

	#[test]
	fn test_b_part_2(){
		let mut jup: Jupiter = Jupiter::new((-8,-10,0), (5,5,10), (2,-7,3), (9,-8,-3));
		assert_eq!(4686774924, jup.find_steps_till_repeat());
	}
}