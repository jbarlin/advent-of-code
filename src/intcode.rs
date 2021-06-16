type NumType = usize;

pub type Memory = Vec<NumType>;

#[derive(Debug, Clone)]
pub struct IntCodeVM {
	pub memory: Memory,
	pub register: usize,
	pub stopped: bool
}
pub enum Opcode{
	Add = 1,
	Multiply = 2,
	Stop = 99
}

impl Opcode {
    fn convert_int(value: NumType) -> Opcode {
        match value {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            99 => Opcode::Stop,
            _ => panic!("Not a valid op code?"),
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
			stopped: false
        }
    }

	pub fn get_zero(&self) -> NumType {
		return self.memory[0];
	}

	pub fn run_all(&mut self){
		loop{
			match self.stopped {
				false => {
					self.run_one_command();
				}
				true => {
					break;
				}
			}
		}
	}

	pub fn run_one_command(&mut self){
		let command = self.memory[self.register];
		let opcode = Opcode::convert_int(command);
		let next_instruction = match opcode {
			Opcode::Add => {
				let psn_a = self.memory[self.register + 1];
				let psn_b = self.memory[self.register + 2];
				let dest = self.memory[self.register + 3];
				self.memory[dest] = self.memory[psn_a] + self.memory[psn_b];
				self.register + 4
			}
			Opcode::Multiply => {
				let psn_a = self.memory[self.register + 1];
				let psn_b = self.memory[self.register + 2];
				let dest = self.memory[self.register + 3];
				self.memory[dest] = self.memory[psn_a] * self.memory[psn_b];
				self.register + 4
			}
			Opcode::Stop => {
				self.stopped = true;
				self.register + 1
			}
		};
		self.register = next_instruction;
		return;
	}
}


#[cfg(test)]
mod tests {
    use super::*;
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
