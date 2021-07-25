pub type NumType = i64;

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub type Memory = Vec<NumType>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
	Ready,
	Stopped,
	Reading,
}

impl Default for State {
	fn default() -> Self {
		State::Ready
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
	Position = 0,
	Immediate = 1,
	Relative = 2,
}

impl Mode {
	fn convert_int(value: NumType) -> Mode {
		match value {
			0 => Mode::Position,
			1 => Mode::Immediate,
			2 => Mode::Relative,
			_ => unreachable!(),
		}
	}
}
impl Default for Mode {
	fn default() -> Self {
		Mode::Position
	}
}

pub enum Opcode {
	Add = 1,
	Multiply = 2,
	Input = 3,
	Output = 4,
	JumpIfTrue = 5,
	JumpIfFalse = 6,
	LessThan = 7,
	Equals = 8,
	AdjustRel = 9,
	Stop = 99,
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
			9 => Opcode::AdjustRel,
			99 => Opcode::Stop,
			_ => panic!("Not a valid op code - {}", value),
		}
	}
}

#[derive(Debug, Clone)]
pub struct IntCodeVM {
	pub memory: Memory,
	pub register: usize,
	pub state: State,
	pub mode: Mode,
	pub relative: NumType,
	input: Rc<RefCell<VecDeque<NumType>>>,
	output: Rc<RefCell<VecDeque<NumType>>>,
}

fn safer_unsigned_add(u: usize, i: NumType) -> usize{
	if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u64 as usize).unwrap()
    } else {
        u.checked_add(i as usize).unwrap()
    }
}

fn signed_add_to_unsigned(a: NumType, b: NumType) -> usize {
	if a.is_negative() && b.is_negative(){
		panic!("Cannot add two negative numbers to make an unsigned number");
	}else if a.is_negative(){
		safer_unsigned_add(b as usize, a)
	}else{
		safer_unsigned_add(a as usize, b)
	}
}

impl IntCodeVM {
	pub fn new(memory: Memory) -> Self {
		let inp: VecDeque<NumType> = VecDeque::new();
		let out: VecDeque<NumType> = VecDeque::new();
		Self {
			memory,
			register: 0,
			relative: 0,
			state: State::default(),
			mode: Mode::default(),
			input: Rc::new(RefCell::new(inp)),
			output: Rc::new(RefCell::new(out)),
		}
	}

	pub fn new_networked(memory: Memory, input_from: &IntCodeVM) -> Self {
		let out: VecDeque<NumType> = VecDeque::new();
		Self {
			memory,
			register: 0,
			relative: 0,
			state: State::default(),
			mode: Mode::default(),
			input: input_from.output(),
			output: Rc::new(RefCell::new(out)),
		}
	}

	pub fn push_input(&mut self, entry: NumType) {
		self.input.borrow_mut().push_back(entry);
		if self.state == State::Reading {
			//We should be right to keep running!
			self.state = State::Ready;
		}
	}

	pub fn output(&self) -> Rc<RefCell<VecDeque<NumType>>> {
		Rc::clone(&self.output)
	}

	pub fn get_zero(&self) -> NumType {
		return self.memory[0];
	}

	pub fn run_all(&mut self) {
		loop {
			match self.state {
				State::Ready => {
					self.run_one_command();
				}
				State::Stopped => {
					break;
				}
				State::Reading => {
					if self.input.borrow().len() > 0 {
						self.state = State::Ready;
					} else {
						break;
					}
				}
			}
		}
	}

	fn read_mem(&self, index: usize, mode: Mode) -> NumType {
		match mode {
			Mode::Immediate => *self.memory.get(index).unwrap_or(&0),
			Mode::Position => *self
				.memory
				.get(*self.memory.get(index).unwrap_or(&0) as usize)
				.unwrap_or(&0),
			Mode::Relative => {
				let oindx = *self.memory.get(index).unwrap_or(&0);
				let nindx = signed_add_to_unsigned(oindx, self.relative);
				return *self
					.memory
					.get(nindx)
					.unwrap_or(&0);
			}
		}
	}

	fn write_mem(&mut self, index: usize, mode: Mode, value: NumType) {
		match mode {
			Mode::Position => {
				let rel_index = self.read_mem(index, Mode::Immediate) as usize;
				if self.memory.len() <= rel_index {
					self.memory.resize(rel_index + 1, 0);
				}
				self.memory[rel_index] = value
			}
			Mode::Relative => {
				let rel_index = self.read_mem(index, Mode::Immediate);
				let nindx = signed_add_to_unsigned(rel_index, self.relative);
				if self.memory.len() <= nindx {
					self.memory.resize(nindx + 1, 0);
				}
				self.memory[nindx] = value
			}
			_ => unreachable!(),
		}
	}

	fn run_one_command(&mut self) {
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
				let a = self.read_mem(self.register + 1, modes_find());
				let b = self.read_mem(self.register + 2, modes_find());
				self.write_mem(self.register + 3, modes_find(), a + b);
				self.register + 4
			}
			Opcode::Multiply => {
				let a = self.read_mem(self.register + 1, modes_find());
				let b = self.read_mem(self.register + 2, modes_find());
				self.write_mem(self.register + 3, modes_find(), a * b);
				self.register + 4
			}
			Opcode::Stop => {
				self.state = State::Stopped;
				self.register + 1
			}
			Opcode::AdjustRel => {
				let adjust_by = self.read_mem(self.register + 1, modes_find());
				self.relative += adjust_by;
				self.register + 2
			}
			Opcode::Input => {
				let opt: Option<NumType> = self.input.borrow_mut().pop_front();
				match opt {
					Some(nvar) => {
						self.write_mem(self.register + 1, modes_find(), nvar);
						self.register + 2
					}
					None => {
						self.state = State::Reading;
						self.register
					}
				}
			}
			Opcode::Output => {
				self.output
					.borrow_mut()
					.push_back(self.read_mem(self.register + 1, modes_find()));
				self.register + 2
			}
			Opcode::JumpIfTrue => {
				let act = self.read_mem(self.register + 1, modes_find());
				if act != 0 {
					self.read_mem(self.register + 2, modes_find()) as usize
				} else {
					self.register + 3
				}
			}
			Opcode::JumpIfFalse => {
				let act = self.read_mem(self.register + 1, modes_find());
				if act == 0 {
					self.read_mem(self.register + 2, modes_find()) as usize
				} else {
					self.register + 3
				}
			}
			Opcode::LessThan => {
				let a = self.read_mem(self.register + 1, modes_find());
				let b = self.read_mem(self.register + 2, modes_find());
				self.write_mem(self.register + 3, modes_find(), if a < b { 1 } else { 0 });
				self.register + 4
			}
			Opcode::Equals => {
				let a = self.read_mem(self.register + 1, modes_find());
				let b = self.read_mem(self.register + 2, modes_find());
				self.write_mem(self.register + 3, modes_find(), if a == b { 1 } else { 0 });
				self.register + 4
			}
		} as usize;
		self.register = next_instruction;
		return;
	}

	/// Get a reference to the int code v m's input.
	pub fn input(&self) -> Rc<RefCell<VecDeque<NumType>>> {
		Rc::clone(&self.input)
	}

	pub fn network_to(&mut self, other: &IntCodeVM) {
		self.input = other.output();
	}

	pub fn is_stopped(&self) -> bool {
		matches!(self.state, State::Stopped)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_egs_d9_a(){
		let memory_a = vec![1102,34915192,34915192,7,4,7,99,0];
		let mut vma = IntCodeVM::new(memory_a);
		vma.run_all();
		let output_a = vma.output().take();
		assert_eq!(1,output_a.len());
		assert_eq!(16, output_a[0].to_string().len());

		let mut memory_b = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
		let mut vmb = IntCodeVM::new(memory_b.clone());
		vmb.run_all();
		let mut output_b = vmb.output().take();
		assert_eq!(memory_b.len(), output_b.len());
		for _i in 0..16{
			assert_eq!(memory_b.pop(),output_b.pop_back());
		}
	}

	#[test]
	fn test_egs_d7_b() {
		let memory_a = vec![
			3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
			28, 1005, 28, 6, 99, 0, 0, 5,
		];
		let mut vmaa = IntCodeVM::new(memory_a.clone());
		let mut vmab = IntCodeVM::new_networked(memory_a.clone(), &vmaa);
		let mut vmac = IntCodeVM::new_networked(memory_a.clone(), &vmab);
		let mut vmad = IntCodeVM::new_networked(memory_a.clone(), &vmac);
		let mut vmae = IntCodeVM::new_networked(memory_a, &vmad);
		vmaa.network_to(&vmae);
		vmaa.push_input(9);
		vmaa.push_input(0);
		vmab.push_input(8);
		vmac.push_input(7);
		vmad.push_input(6);
		vmae.push_input(5);
		loop {
			vmaa.run_all();
			vmab.run_all();
			vmac.run_all();
			vmad.run_all();
			vmae.run_all();

			if vmaa.is_stopped()
				&& vmab.is_stopped()
				&& vmac.is_stopped()
				&& vmad.is_stopped()
				&& vmae.is_stopped()
			{
				break;
			}
		}
		assert_eq!(139629729, vmae.output().borrow_mut().pop_front().unwrap());

		let memory_b = vec![
			3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
			-5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
			53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
		];
		let mut vmba = IntCodeVM::new(memory_b.clone());
		let mut vmbb = IntCodeVM::new_networked(memory_b.clone(), &vmba);
		let mut vmbc = IntCodeVM::new_networked(memory_b.clone(), &vmbb);
		let mut vmbd = IntCodeVM::new_networked(memory_b.clone(), &vmbc);
		let mut vmbe = IntCodeVM::new_networked(memory_b, &vmbd);
		vmba.network_to(&vmbe);
		vmba.push_input(9);
		vmba.push_input(0);
		vmbb.push_input(7);
		vmbc.push_input(8);
		vmbd.push_input(5);
		vmbe.push_input(6);
		vmba.run_all();
		loop {
			vmba.run_all();
			vmbb.run_all();
			vmbc.run_all();
			vmbd.run_all();
			vmbe.run_all();
			if vmba.is_stopped()
				&& vmbb.is_stopped()
				&& vmbc.is_stopped()
				&& vmbd.is_stopped()
				&& vmbe.is_stopped()
			{
				break;
			}
		}
		assert_eq!(18216, vmbe.output().borrow_mut().pop_front().unwrap());
	}

	#[test]
	fn test_egs_d7_a() {
		let memory_a = vec![
			3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
		];
		let mut vmaa = IntCodeVM::new(memory_a.clone());
		let mut vmab = IntCodeVM::new_networked(memory_a.clone(), &vmaa);
		let mut vmac = IntCodeVM::new_networked(memory_a.clone(), &vmab);
		let mut vmad = IntCodeVM::new_networked(memory_a.clone(), &vmac);
		let mut vmae = IntCodeVM::new_networked(memory_a, &vmad);
		vmaa.push_input(4);
		vmaa.push_input(0);
		vmab.push_input(3);
		vmac.push_input(2);
		vmad.push_input(1);
		vmae.push_input(0);
		vmaa.run_all();
		assert!(vmaa.is_stopped());
		vmab.run_all();
		assert!(vmab.is_stopped());
		vmac.run_all();
		assert!(vmac.is_stopped());
		vmad.run_all();
		assert!(vmad.is_stopped());
		vmae.run_all();
		assert!(vmae.is_stopped());
		assert_eq!(43210, vmae.output().borrow_mut().pop_front().unwrap());

		let memory_b = vec![
			3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
			99, 0, 0,
		];
		let mut vmba = IntCodeVM::new(memory_b.clone());
		let mut vmbb = IntCodeVM::new_networked(memory_b.clone(), &vmba);
		let mut vmbc = IntCodeVM::new_networked(memory_b.clone(), &vmbb);
		let mut vmbd = IntCodeVM::new_networked(memory_b.clone(), &vmbc);
		let mut vmbe = IntCodeVM::new_networked(memory_b, &vmbd);
		vmba.push_input(0);
		vmba.push_input(0);
		vmbb.push_input(1);
		vmbc.push_input(2);
		vmbd.push_input(3);
		vmbe.push_input(4);
		vmba.run_all();
		assert!(vmba.is_stopped());
		vmbb.run_all();
		assert!(vmbb.is_stopped());
		vmbc.run_all();
		assert!(vmbc.is_stopped());
		vmbd.run_all();
		assert!(vmbd.is_stopped());
		vmbe.run_all();
		assert!(vmbe.is_stopped());
		assert_eq!(54321, vmbe.output().borrow_mut().pop_front().unwrap());

		let memory_c = vec![
			3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
			33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
		];
		let mut vmca = IntCodeVM::new(memory_c.clone());
		let mut vmcb = IntCodeVM::new_networked(memory_c.clone(), &vmca);
		let mut vmcc = IntCodeVM::new_networked(memory_c.clone(), &vmcb);
		let mut vmcd = IntCodeVM::new_networked(memory_c.clone(), &vmcc);
		let mut vmce = IntCodeVM::new_networked(memory_c, &vmcd);
		vmca.push_input(1);
		vmca.push_input(0);
		vmcb.push_input(0);
		vmcc.push_input(4);
		vmcd.push_input(3);
		vmce.push_input(2);
		vmca.run_all();
		assert!(vmca.is_stopped());
		vmcb.run_all();
		assert!(vmcb.is_stopped());
		vmcc.run_all();
		assert!(vmcc.is_stopped());
		vmcd.run_all();
		assert!(vmcd.is_stopped());
		vmce.run_all();
		assert!(vmce.is_stopped());
		assert_eq!(65210, vmce.output().borrow_mut().pop_front().unwrap());
	}

	#[test]
	fn test_egs_d5_b() {
		let mut vmaa = IntCodeVM::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
		vmaa.push_input(8);
		vmaa.run_all();
		assert_eq!(1, vmaa.output().borrow_mut().pop_front().unwrap());

		let mut vmab = IntCodeVM::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
		vmab.push_input(81);
		vmab.run_all();
		assert_eq!(0, vmab.output().borrow_mut().pop_front().unwrap());

		let mut vmba = IntCodeVM::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
		vmba.push_input(88);
		vmba.run_all();
		assert_eq!(0, vmba.output().borrow_mut().pop_front().unwrap());

		let mut vmbb = IntCodeVM::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
		vmbb.push_input(3);
		vmbb.run_all();
		assert_eq!(1, vmbb.output().borrow_mut().pop_front().unwrap());

		let mut vmca = IntCodeVM::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
		vmca.push_input(8);
		vmca.run_all();
		assert_eq!(1, vmca.output().borrow_mut().pop_front().unwrap());

		let mut vmcb = IntCodeVM::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
		vmcb.push_input(81);
		vmcb.run_all();
		assert_eq!(0, vmcb.output().borrow_mut().pop_front().unwrap());

		let mut vmda = IntCodeVM::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
		vmda.push_input(88);
		vmda.run_all();
		assert_eq!(0, vmda.output().borrow_mut().pop_front().unwrap());

		let mut vmdb = IntCodeVM::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
		vmdb.push_input(3);
		vmdb.run_all();
		assert_eq!(1, vmdb.output().borrow_mut().pop_front().unwrap());

		let mut vmea = IntCodeVM::new(vec![
			3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
		]);
		vmea.push_input(0);
		vmea.run_all();
		assert_eq!(0, vmea.output().borrow_mut().pop_front().unwrap());

		let mut vmeb = IntCodeVM::new(vec![
			3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
		]);
		vmeb.push_input(81);
		vmeb.run_all();
		assert_eq!(1, vmeb.output().borrow_mut().pop_front().unwrap());

		let mut vmfa = IntCodeVM::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
		vmfa.push_input(0);
		vmfa.run_all();
		assert_eq!(0, vmfa.output().borrow_mut().pop_front().unwrap());

		let mut vmfb = IntCodeVM::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
		vmfb.push_input(81);
		vmfb.run_all();
		assert_eq!(1, vmfb.output().borrow_mut().pop_front().unwrap());

		let mut vmga = IntCodeVM::new(vec![
			3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
			0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
			20, 1105, 1, 46, 98, 99,
		]);
		vmga.push_input(7);
		vmga.run_all();
		assert_eq!(999, vmga.output().borrow_mut().pop_front().unwrap());

		let mut vmgb = IntCodeVM::new(vec![
			3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
			0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
			20, 1105, 1, 46, 98, 99,
		]);
		vmgb.push_input(8);
		vmgb.run_all();
		assert_eq!(1000, vmgb.output().borrow_mut().pop_front().unwrap());

		let mut vmgc = IntCodeVM::new(vec![
			3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
			0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
			20, 1105, 1, 46, 98, 99,
		]);
		vmgc.push_input(9);
		vmgc.run_all();
		assert_eq!(1001, vmgc.output().borrow_mut().pop_front().unwrap());
	}

	#[test]
	fn test_egs_d5_a() {
		let mut vma = IntCodeVM::new(vec![3, 0, 4, 0, 99]);
		vma.push_input(8);
		vma.run_all();
		let outa = vma.output();
		assert_eq!(State::Stopped, vma.state);
		assert_eq!(1, outa.borrow().len());
		assert_eq!(8, outa.borrow_mut().pop_front().unwrap());

		let mut vmb = IntCodeVM::new(vec![3, 0, 4, 0, 99]);
		vmb.run_all();
		let outba = vmb.output();
		assert_eq!(State::Reading, vmb.state);
		assert_eq!(0, outba.borrow().len());
		vmb.push_input(6);
		let outbb = vmb.output();
		assert_eq!(State::Ready, vmb.state);
		assert_eq!(0, outbb.borrow().len());
		vmb.run_all();
		let outbc = vmb.output();
		assert_eq!(State::Stopped, vmb.state);
		assert_eq!(1, outbc.borrow().len());
		assert_eq!(6, outbc.borrow_mut().pop_front().unwrap());

		let mut vmc = IntCodeVM::new(vec![1002, 4, 3, 4, 33]);
		vmc.run_all();
		let post_c: Memory = vec![1002, 4, 3, 4, 99];
		assert_eq!(vmc.memory, post_c);
	}
	#[test]
	fn test_egs_d2() {
		let mut vma = IntCodeVM::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
		vma.run_all();
		let post_a: Memory = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
		assert_eq!(vma.memory, post_a);

		let mut vmb = IntCodeVM::new(vec![1, 0, 0, 0, 99]);
		vmb.run_all();
		let post_b: Memory = vec![2, 0, 0, 0, 99];
		assert_eq!(vmb.memory, post_b);

		let mut vmc = IntCodeVM::new(vec![2, 3, 0, 3, 99]);
		vmc.run_all();
		let post_c: Memory = vec![2, 3, 0, 6, 99];
		assert_eq!(vmc.memory, post_c);

		let mut vmd = IntCodeVM::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
		vmd.run_all();
		let post_d: Memory = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
		assert_eq!(vmd.memory, post_d);

		let mut vme = IntCodeVM::new(vec![2, 4, 4, 5, 99, 0]);
		vme.run_all();
		let post_e: Memory = vec![2, 4, 4, 5, 99, 9801];
		assert_eq!(vme.memory, post_e);
	}
}
