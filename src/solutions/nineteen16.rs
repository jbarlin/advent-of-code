use crate::AoCDay;

pub const DAY_16_DATA: &str = include_str!("../../inputs/2019/Day16.txt");

pub struct Code;

fn parse_input(input: &str) -> Vec<u8> {
	return input
		.chars()
		.map(|c| c.to_string().parse::<u8>().unwrap())
		.collect();
}

pub fn part_1_impl(input: &str) -> String {
	let inp = parse_input(input);
	get_digits_after_phases(8, 100, inp, 1, 0)
}

pub fn part_2_impl(input: &str) -> String {
	let inp = parse_input(input);
	let mut num_dropped: usize = 0;
	for i in 0..7 {
		num_dropped = (num_dropped * 10) + (inp[i] as usize);
	}
	get_digits_after_phases(8, 100, inp, 10_000, num_dropped)
}

fn get_digits_after_phases(num_digits: usize, num_phases: u8, input: Vec<u8>, num_repeats: usize, num_dropped: usize) -> String {
	//If we are in second half it looks like we can use a shortcut!
	let use_shortcut: bool = num_dropped > ((input.len() * num_repeats) / 2);
	assert!(num_dropped < (input.len() * num_repeats));
	let repeating: Vec<u8> = input.repeat(num_repeats);
	let (_, split_part): (&[u8], &[u8]) = repeating.split_at(num_dropped);
	let mut phasable: Vec<u8> = Vec::from(split_part);
	assert!(phasable.len() > 1);
	if !use_shortcut {
		for _i in 0..num_phases {
			perform_phase_normal(&mut phasable);
		}
	} else {
		for _i in 0..num_phases {
			perform_phase_shortcut(&mut phasable);
		}
	}

	return phasable.drain(0..num_digits)
		.fold("".to_string(), |acc, nv| {
			return format!("{}{}", acc, nv.to_string());
		});
}

fn perform_phase_shortcut(signal: &mut Vec<u8>) {
	for indx in (0..=(signal.len() - 2)).rev() {
		signal[indx] = (signal[indx] + signal[indx + 1]) % 10;
	}
}

fn perform_phase_normal(signal: &mut Vec<u8>) {
	let original = signal.clone();
	let mult_table: Vec<i64> = vec![1, 0, -1, 0];
	for change_index in 0..original.len() {
		let mut accumulator: i64 = 0;
		for look_index in change_index..original.len() {
			let diff: usize = look_index - change_index;
			let divided: usize = diff / (change_index + 1);
			let mult_by: i64 = mult_table[divided % 4];
			accumulator += (original[look_index] as i64) * mult_by;
		}
		signal[change_index] = (accumulator.abs() % 10) as u8;
	}
}

impl AoCDay for Code {
	fn part1(&self) -> String {
		return part_1_impl(DAY_16_DATA);
	}

	fn part2(&self) -> String {
		return part_2_impl(DAY_16_DATA);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example_a() {
		let inp = parse_input("12345678");
		assert_eq!(
			get_digits_after_phases(8, 4, inp, 1, 0),
			"01029498");
	}

	#[test]
	fn test_example_b() {
		let inp = parse_input("80871224585914546619083218645595");
		assert_eq!(
			get_digits_after_phases(8, 100, inp, 1, 0),
			"24176176");
	}

	#[test]
	fn test_example_c() {
		let inp = parse_input("19617804207202209144916044189917");
		assert_eq!(
			get_digits_after_phases(8, 100, inp, 1, 0),
			"73745418");
	}

	#[test]
	fn test_example_d() {
		let inp = parse_input("69317163492948606335995924319873");
		assert_eq!(
			get_digits_after_phases(8, 100, inp, 1, 0),
			"52432133");
	}

	#[test]
	fn test_example_e() {
		let inp = parse_input("03036732577212944063491565474664");
		assert_eq!(
			get_digits_after_phases(8, 100, inp, 10000, 0303673),
			"84462026");
	}
}