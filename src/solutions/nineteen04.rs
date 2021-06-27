use crate::SinglePart;

pub struct Code;
pub const MIN: u32 = 168630;
pub const MAX: u32 = 718098;

impl SinglePart for Code{
	fn run(&self) -> String {
		let res = count_passwords_between(MIN, MAX);
		let fmt = format!("Part 1:\t{}\nPart 2:\t{}\n",res.0, res.1);
		return fmt;
    }
}

const OPS: [u32;6] = [100000,10000,1000,100,10,1];

pub fn count_passwords_between(min: u32, max: u32) -> (u32, u32){
	let mut count_a: u32 = 0;
	let mut count_b: u32 = 0;
	let mut num_checking = min;
	while num_checking < max {
		let mut last_num: u8 = 0;
		let mut curr_run_same: u8 = 1;
		let mut dbl: bool = false;
		let mut just_dbl: bool = false;
		OPS.iter()
		.for_each(|div: &u32| {
			let mut num: u8 = ((num_checking / div) % 10) as u8;
			while num < last_num {
				num_checking -= num_checking % div;
				num_checking += div;
				num += 1;
			}
			if num == last_num{
				curr_run_same += 1;
			} else {
				if curr_run_same == 2{
					dbl = true;
					just_dbl = true;
				} else if curr_run_same > 2 {
					dbl = true;
				}
				curr_run_same = 1;
			}
			last_num = num;
		});
		if curr_run_same == 2{
			dbl = true;
			just_dbl = true;
		} else if curr_run_same > 2 {
			dbl = true;
		}
		if dbl && num_checking < max {
			count_a += 1;
			if just_dbl {
				count_b += 1;
			}
		}
		num_checking += 1;
	}


	return (count_a, count_b);
}

