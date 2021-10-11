use itertools::Itertools;
use crate::AoCDay;
use crate::coords::Coords;
use crate::intcode::{IntCodeVM, NumType};

pub struct Code;

pub const DAY_19_DATA:[NumType;424] = [109,424,203,1,21101,0,11,0,1106,0,282,21102,18,1,0,1106,0,259,2101,0,1,221,203,1,21102,1,31,0,1105,1,282,21101,0,38,0,1106,0,259,20102,1,23,2,22101,0,1,3,21102,1,1,1,21101,57,0,0,1105,1,303,2102,1,1,222,20101,0,221,3,21002,221,1,2,21101,0,259,1,21102,1,80,0,1105,1,225,21102,125,1,2,21102,1,91,0,1106,0,303,2101,0,1,223,21002,222,1,4,21102,1,259,3,21102,225,1,2,21102,225,1,1,21101,0,118,0,1106,0,225,20102,1,222,3,21101,0,69,2,21102,1,133,0,1106,0,303,21202,1,-1,1,22001,223,1,1,21102,148,1,0,1106,0,259,1201,1,0,223,20101,0,221,4,21001,222,0,3,21102,1,22,2,1001,132,-2,224,1002,224,2,224,1001,224,3,224,1002,132,-1,132,1,224,132,224,21001,224,1,1,21102,195,1,0,106,0,108,20207,1,223,2,20101,0,23,1,21102,-1,1,3,21101,0,214,0,1105,1,303,22101,1,1,1,204,1,99,0,0,0,0,109,5,1202,-4,1,249,21202,-3,1,1,22102,1,-2,2,21201,-1,0,3,21101,250,0,0,1106,0,225,22102,1,1,-4,109,-5,2105,1,0,109,3,22107,0,-2,-1,21202,-1,2,-1,21201,-1,-1,-1,22202,-1,-2,-2,109,-3,2106,0,0,109,3,21207,-2,0,-1,1206,-1,294,104,0,99,22101,0,-2,-2,109,-3,2106,0,0,109,5,22207,-3,-4,-1,1206,-1,346,22201,-4,-3,-4,21202,-3,-1,-1,22201,-4,-1,2,21202,2,-1,-1,22201,-4,-1,1,22102,1,-2,3,21101,0,343,0,1106,0,303,1105,1,415,22207,-2,-3,-1,1206,-1,387,22201,-3,-2,-3,21202,-2,-1,-1,22201,-3,-1,3,21202,3,-1,-1,22201,-3,-1,2,22102,1,-4,1,21101,384,0,0,1106,0,303,1106,0,415,21202,-4,-1,-4,22201,-4,-3,-4,22202,-3,-2,-2,22202,-2,-4,-4,22202,-3,-2,-3,21202,-4,-1,-2,22201,-3,-2,1,21202,1,1,-4,109,-5,2105,1,0];

impl AoCDay for Code{
	fn part1(&self) -> String {
		let vm: IntCodeVM = IntCodeVM::new(Vec::from(DAY_19_DATA));
		(0_i64..50_i64)
			.combinations_with_replacement(2)
			.fold(0_i64,|inc:i64, i: Vec<i64>|{
				let mut vm = vm.clone();
				vm.push_input(i[0]);
				vm.push_input(i[1]);
				vm.run_all();
				return inc + vm.output().borrow_mut().pop_front().unwrap();
			}).to_string()
	}

	fn part2(&self) -> String {
		let vm: IntCodeVM = IntCodeVM::new(Vec::from(DAY_19_DATA));
		let mut curr_tl = Coords{
			x: 0,
			y: 0
		};
		loop{
			//OK, check tr
			let curr_tr = curr_tl.add_x(99);
			let mut vma = vm.clone();
			vma.push_input(curr_tr.x);
			vma.push_input(curr_tr.y);
			vma.run_all();
			let res = vma.output().borrow_mut().pop_front().unwrap();
			if res == 1 {
				//OK, we can try bottom left
				let curr_bl = curr_tl.add_y(99);
				let mut vma = vm.clone();
				vma.push_input(curr_bl.x);
				vma.push_input(curr_bl.y);
				vma.run_all();
				let res = vma.output().borrow_mut().pop_front().unwrap();
				if res == 1 {
					//OK, check the current position
					let mut vma = vm.clone();
					vma.push_input(curr_tl.x);
					vma.push_input(curr_tl.y);
					vma.run_all();
					let res = vma.output().borrow_mut().pop_front().unwrap();
					if res == 1{
						let curr_br = curr_tl.add_x(99).add_y(99);
						let mut vma = vm.clone();
						vma.push_input(curr_br.x);
						vma.push_input(curr_br.y);
						vma.run_all();
						let res = vma.output().borrow_mut().pop_front().unwrap();
						if res == 1{
							break;
						}else{
							//????
							curr_tl = curr_tl.add_x(1).add_y(1);
						}
					}else{
						curr_tl = curr_tl.add_x(1).add_y(1);
					}
				}else{
					curr_tl = curr_tl.add_x(1);
				}
			}else {
				curr_tl = curr_tl.add_y(1);
			}
		}
		//6900945
		(((curr_tl.x as usize) * 10_000_usize) + (curr_tl.y as usize)).to_string()
	}
}