use crate::AoCDay;

use super::super::intcode::IntCodeVM;
use super::super::intcode::Memory;

pub struct Code;

const FL_CONT: &str = include_str!("../../inputs/2019/Day5.txt");


impl AoCDay for Code{
	fn part1(&self) -> String {
        let file_content: String = FL_CONT.to_string();
		let program: Memory = IntCodeVM::parse_str(file_content);
		let mut vm = IntCodeVM::new(program);
		vm.push_input(1);
		vm.run_all();
		let mut out = vm.output();
		return out.pop_back().unwrap().to_string();
    }
	fn part2(&self) -> String {
		let file_content: String = FL_CONT.to_string();
		let program: Memory = IntCodeVM::parse_str(file_content);
		let mut vm = IntCodeVM::new(program);
		vm.push_input(5);
		vm.run_all();
		let mut out = vm.output();
		return out.pop_back().unwrap().to_string();
	}
}