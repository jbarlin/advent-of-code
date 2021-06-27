use crate::AoCDay;

use super::super::intcode::IntCodeVM;
use super::super::intcode::Memory;

pub struct Code;
const DAY_2_DATA: [i64; 173] = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,6,19,23,2,6,23,27,1,5,27,31,2,31,9,35,1,35,5,39,1,39,5,43,1,43,10,47,2,6,47,51,1,51,5,55,2,55,6,59,1,5,59,63,2,63,6,67,1,5,67,71,1,71,6,75,2,75,10,79,1,79,5,83,2,83,6,87,1,87,5,91,2,9,91,95,1,95,6,99,2,9,99,103,2,9,103,107,1,5,107,111,1,111,5,115,1,115,13,119,1,13,119,123,2,6,123,127,1,5,127,131,1,9,131,135,1,135,9,139,2,139,6,143,1,143,5,147,2,147,6,151,1,5,151,155,2,6,155,159,1,159,2,163,1,9,163,0,99,2,0,14,0];

impl AoCDay for Code{
	fn part1(&self) -> String {
		let mut program: Memory = Vec::from(DAY_2_DATA);
		program[1] = 12;
		program[2] = 2;
		let mut vm = IntCodeVM::new(program);
		vm.run_all();
		return vm.get_zero().to_string();
    }
	fn part2(&self) -> String {
		let mut program: Memory = Vec::from(DAY_2_DATA);
		for noun in 0..99{
			program[1] = noun;
			for verb in 0..99{
				program[2] = verb;
				let mut vm = IntCodeVM::new(program.clone());
				vm.run_all();
				if vm.get_zero() == 19690720{
					return ((100 * noun) + verb).to_string();
				}
			}
		}
		return "Error".to_string();
	}
}

