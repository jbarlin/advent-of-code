use crate::AoCDay;

use super::super::intcode::IntCodeVM;
use super::super::intcode::Memory;

pub struct Code;

const FL_CONT: &str = include_str!("../../inputs/2019/Day2.txt");


impl AoCDay for Code{
	fn part1(&self) -> String {
        let file_content: String = FL_CONT.to_string();
		let mut program: Memory = IntCodeVM::parse_str(file_content);
		program[1] = 12;
		program[2] = 2;
		let mut vm = IntCodeVM::new(program);
		vm.run_all();
		return vm.get_zero().to_string();
    }
	fn part2(&self) -> String {
		let file_content: String = FL_CONT.to_string();
		let mut program: Memory = IntCodeVM::parse_str(file_content);
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

