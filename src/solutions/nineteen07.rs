use std::ops::RangeInclusive;

use crate::AoCDay;
use crate::intcode::Memory;
use itertools::Itertools;
use rayon::prelude::*;

use super::super::intcode::IntCodeVM;
use super::super::intcode::NumType;

pub struct Code;

pub const DAY_7_DATA:[NumType;519] = [3,8,1001,8,10,8,105,1,0,0,21,34,59,76,101,114,195,276,357,438,99999,3,9,1001,9,4,9,1002,9,4,9,4,9,99,3,9,102,4,9,9,101,2,9,9,102,4,9,9,1001,9,3,9,102,2,9,9,4,9,99,3,9,101,4,9,9,102,5,9,9,101,5,9,9,4,9,99,3,9,102,2,9,9,1001,9,4,9,102,4,9,9,1001,9,4,9,1002,9,3,9,4,9,99,3,9,101,2,9,9,1002,9,3,9,4,9,99,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,99,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,99];

impl AoCDay for Code{
	fn part1(&self) -> String {
		let memory = Vec::from(DAY_7_DATA);
		return perform_work(memory, 0..=4, false).to_string();
    }
	fn part2(&self) -> String {
		let memory = Vec::from(DAY_7_DATA);
		return perform_work(memory, 5..=9, true).to_string();
	}
}

pub fn perform_work(memory: Memory, range: RangeInclusive<i64>, link_a_and_e: bool) -> i64{
	range
		.permutations(5)
		.par_bridge()
		.map(|res|{
			let mut vma = IntCodeVM::new(memory.clone());
			let mut vmb = IntCodeVM::new_networked(memory.clone(), &vma);
			let mut vmc = IntCodeVM::new_networked(memory.clone(), &vmb);
			let mut vmd = IntCodeVM::new_networked(memory.clone(), &vmc);
			let mut vme = IntCodeVM::new_networked(memory.clone(), &vmd);
			if link_a_and_e {
				vma.network_to(&vme);
			}
			vma.push_input(res[0]);
			vma.push_input(0);
			vmb.push_input(res[1]);
			vmc.push_input(res[2]);
			vmd.push_input(res[3]);
			vme.push_input(res[4]);
			loop{
				vma.run_all();
				vmb.run_all();
				vmc.run_all();
				vmd.run_all();
				vme.run_all();
				if vma.is_stopped() && vmb.is_stopped() && vmc.is_stopped() && vmd.is_stopped() && vme.is_stopped() {
					break;
				}
			}
			return vme.output().take().pop_front().unwrap();
		})
		.max_by(|x, y| x.cmp(y))
		.unwrap_or(0)
}