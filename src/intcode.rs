type NumType = i64;

use std::collections::VecDeque;

pub type Memory = Vec<NumType>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
	Ready,
	Stopped,
	Reading
}
impl Default for State {
	fn default() -> Self {
		State::Ready
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode{
	Position = 0,
	Immidiate = 1
}

impl Mode{
	fn convert_int(value: NumType) -> Mode{
		match value{
			0 => Mode::Position,
			1 => Mode::Immidiate,
			_ => unreachable!()
		}
	}
}
impl Default for Mode{
	fn default() -> Self {
		Mode::Position
	}
}

#[derive(Debug, Clone)]
pub struct IntCodeVM {
	pub memory: Memory,
	pub register: usize,
	pub state: State,
	pub mode: Mode,
	input: VecDeque<NumType>,
	output: VecDeque<NumType>
}
pub enum Opcode{
	Add = 1,
	Multiply = 2,
	Input = 3,
	Output = 4,
	JumpIfTrue = 5,
	JumpIfFalse = 6,
	LessThan = 7,
	Equals = 8,
	Stop = 99
}

impl Opcode {
	fn convert_int(value: NumType) -> Opcode {
		match value {
			1 => Opcode::Add,
			2 => Opcode::Multiply,
			3 => Opcode::Input,
			4 => Opcode::Output,
			5 => Opcode::JumpIfTrue,
			6 => Opcode::JumpIfFalse,
			7 => Opcode::LessThan,
			8 => Opcode::Equals,
			99 => Opcode::Stop,
			_ => panic!("Not a valid op code - {}", value),
		}
	}
}

impl IntCodeVM{

	pub fn parse_str(fl_content: String) -> Memory {
		return fl_content
			//Automatically split by line
			.lines()
			//First line
			.next().unwrap()
			//Split by comma
			.split(",")
			//Parse each line into a number
			.map(|lc| lc.parse().unwrap())
			// collect them
			.collect();
	}

	pub fn new_from_str(fl_content: String) -> Self{
		return IntCodeVM::new(IntCodeVM::parse_str(fl_content));
	}

	pub fn new(memory: Memory) -> Self {
		Self {
			memory,
			register: 0,
			state: State::default(),
			mode: Mode::default(),
			input: VecDeque::new(),
			output: VecDeque::new()
		}
	}

	pub fn push_input(&mut self, entry: NumType){
		self.input.push_back(entry);
		if self.state == State::Reading{
			//We should be right to keep running!
			self.state = State::Ready;
		}
	}

	pub fn output(&mut self) -> VecDeque<NumType>{
		let retable = std::mem::replace(&mut self.output, VecDeque::new());
		return retable;
	}

	pub fn get_zero(&self) -> NumType {
		return self.memory[0];
	}

	pub fn run_all(&mut self){
		loop{
			match self.state {
				State::Ready => {
					self.run_one_command();
				}
				State::Stopped => {
					break;
				}
				State::Reading => {
					break;
				}
			}
		}
	}

	fn read(&self, index: usize, mode: Mode) -> NumType {
		match mode {
			Mode::Immidiate => self.memory[index],
			Mode::Position => self.memory[self.memory[index] as usize],
		}
	}
	pub fn write(&mut self, index: usize, mode: Mode, value: NumType) {
		let rel_index = self.memory[index];
		match mode {
			Mode::Position => self.memory[rel_index as usize] = value,
			_ => unreachable!()
		}
	}

	pub fn run_one_command(&mut self){

		let command = self.memory[self.register];

		let opcode = Opcode::convert_int(command % 100);
		let mut mode_raw = command / 100;
		let mut modes_find = || {
			let mode = mode_raw % 10;
			mode_raw /= 10;
			Mode::convert_int(mode)
		};
		let next_instruction = match opcode {
			Opcode::Add => {
				let a = self.read(self.register + 1, modes_find());
				let b = self.read(self.register + 2, modes_find());
				self.write(self.register+3, modes_find(), a + b);
				self.register + 4
			}
			Opcode::Multiply => {
				let a = self.read(self.register + 1, modes_find());
				let b = self.read(self.register + 2, modes_find());
				self.write(self.register+3, modes_find(), a * b);
				self.register + 4
			}
			Opcode::Stop => {
				self.state = State::Stopped;
				self.register + 1
			}
			Opcode::Input => {
				match self.input.pop_front() {
					Some(nvar) => {
						self.write(self.register + 1, modes_find(), nvar);
						self.register + 2
					},
					None => {
						self.state = State::Reading;
						self.register
					}
				}
			},
			Opcode::Output => {
				self.output.push_back(self.read(self.register + 1, modes_find()));
				self.register + 2
			},
			Opcode::JumpIfTrue => {
				let act = self.read(self.register + 1, modes_find());
				if act != 0 {
					self.read(self.register + 2, modes_find()) as usize
				}else{
					self.register + 3
				}
			},
			Opcode::JumpIfFalse => {
				let act = self.read(self.register + 1, modes_find());
				if act == 0 {
					self.read(self.register + 2, modes_find()) as usize
				}else{
					self.register + 3
				}
			},
			Opcode::LessThan => {
				let a = self.read(self.register + 1, modes_find());
				let b = self.read(self.register + 2, modes_find());
				self.write(self.register + 3, modes_find(), if a < b {1} else {0});
				self.register + 4
			},
			Opcode::Equals => {
				let a = self.read(self.register + 1, modes_find());
				let b = self.read(self.register + 2, modes_find());
				self.write(self.register + 3, modes_find(), if a == b {1} else {0});
				self.register + 4
			}
		} as usize;
		self.register = next_instruction;
		return;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_egs_d5_b(){
		let mut vmaa = IntCodeVM::new(vec![3,9,8,9,10,9,4,9,99,-1,8]);
		vmaa.push_input(8);
		vmaa.run_all();
		assert_eq!(1,vmaa.output().pop_front().unwrap());

		let mut vmab = IntCodeVM::new(vec![3,9,8,9,10,9,4,9,99,-1,8]);
		vmab.push_input(81);
		vmab.run_all();
		assert_eq!(0,vmab.output().pop_front().unwrap());

		let mut vmba = IntCodeVM::new(vec![3,9,7,9,10,9,4,9,99,-1,8]);
		vmba.push_input(88);
		vmba.run_all();
		assert_eq!(0,vmba.output().pop_front().unwrap());

		let mut vmbb = IntCodeVM::new(vec![3,9,7,9,10,9,4,9,99,-1,8]);
		vmbb.push_input(3);
		vmbb.run_all();
		assert_eq!(1,vmbb.output().pop_front().unwrap());


		let mut vmca = IntCodeVM::new(vec![3,3,1108,-1,8,3,4,3,99]);
		vmca.push_input(8);
		vmca.run_all();
		assert_eq!(1,vmca.output().pop_front().unwrap());

		let mut vmcb = IntCodeVM::new(vec![3,3,1108,-1,8,3,4,3,99]);
		vmcb.push_input(81);
		vmcb.run_all();
		assert_eq!(0,vmcb.output().pop_front().unwrap());

		let mut vmda = IntCodeVM::new(vec![3,3,1107,-1,8,3,4,3,99]);
		vmda.push_input(88);
		vmda.run_all();
		assert_eq!(0,vmda.output().pop_front().unwrap());

		let mut vmdb = IntCodeVM::new(vec![3,3,1107,-1,8,3,4,3,99]);
		vmdb.push_input(3);
		vmdb.run_all();
		assert_eq!(1,vmdb.output().pop_front().unwrap());


		let mut vmea = IntCodeVM::new(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]);
		vmea.push_input(0);
		vmea.run_all();
		assert_eq!(0,vmea.output().pop_front().unwrap());

		let mut vmeb = IntCodeVM::new(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]);
		vmeb.push_input(81);
		vmeb.run_all();
		assert_eq!(1,vmeb.output().pop_front().unwrap());

		let mut vmfa = IntCodeVM::new(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1]);
		vmfa.push_input(0);
		vmfa.run_all();
		assert_eq!(0,vmfa.output().pop_front().unwrap());

		let mut vmfb = IntCodeVM::new(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1]);
		vmfb.push_input(81);
		vmfb.run_all();
		assert_eq!(1,vmfb.output().pop_front().unwrap());


		let mut vmga = IntCodeVM::new(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);
		vmga.push_input(7);
		vmga.run_all();
		assert_eq!(999,vmga.output().pop_front().unwrap());

		let mut vmgb = IntCodeVM::new(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);
		vmgb.push_input(8);
		vmgb.run_all();
		assert_eq!(1000,vmgb.output().pop_front().unwrap());

		let mut vmgc = IntCodeVM::new(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);
		vmgc.push_input(9);
		vmgc.run_all();
		assert_eq!(1001,vmgc.output().pop_front().unwrap());
	}

	#[test]
	fn test_egs_d5_a(){
		let mut vma = IntCodeVM::new(vec![3,0,4,0,99]);
		vma.push_input(8);
		vma.run_all();
		let mut outa = vma.output();
		assert_eq!(State::Stopped, vma.state);
		assert_eq!(1, outa.len());
		assert_eq!(8, outa.pop_front().unwrap());

		let mut vmb = IntCodeVM::new(vec![3,0,4,0,99]);
		vmb.run_all();
		let outba = vmb.output();
		assert_eq!(State::Reading, vmb.state);
		assert_eq!(0, outba.len());
		vmb.push_input(6);
		let outbb = vmb.output();
		assert_eq!(State::Ready, vmb.state);
		assert_eq!(0, outbb.len());
		vmb.run_all();
		let mut outbc = vmb.output();
		assert_eq!(State::Stopped, vmb.state);
		assert_eq!(1, outbc.len());
		assert_eq!(6, outbc.pop_front().unwrap());

		let mut vmc = IntCodeVM::new(vec![1002,4,3,4,33]);
		vmc.run_all();
		let post_c: Memory = vec![1002,4,3,4,99];
		assert_eq!(vmc.memory, post_c);
		
	}
	#[test]
	fn test_examples() {
		let mut vma = IntCodeVM::new( vec![1,9,10,3,2,3,11,0,99,30,40,50]);
		vma.run_all();
	   	let post_a: Memory = vec![3500,9,10,70,2,3,11,0,99,30,40,50];
		assert_eq!(vma.memory, post_a);

		let mut vmb = IntCodeVM::new( vec![1,0,0,0,99]);
		vmb.run_all();
	   	let post_b: Memory = vec![2,0,0,0,99];
		assert_eq!(vmb.memory, post_b);

		let mut vmc = IntCodeVM::new( vec![2,3,0,3,99]);
		vmc.run_all();
	   	let post_c: Memory = vec![2,3,0,6,99];
		assert_eq!(vmc.memory, post_c);

		let mut vmd = IntCodeVM::new( vec![1,1,1,4,99,5,6,0,99]);
		vmd.run_all();
	   	let post_d: Memory = vec![30,1,1,4,2,5,6,0,99];
		assert_eq!(vmd.memory, post_d);

		let mut vme = IntCodeVM::new( vec![2,4,4,5,99,0]);
		vme.run_all();
	   	let post_e: Memory = vec![2,4,4,5,99,9801];
		assert_eq!(vme.memory, post_e);
	}
}
