use crate::AoCDay;

pub struct Code;

const FL_CONT: &str = include_str!("../../inputs/2019/Day1.txt");

impl AoCDay for Code{
	fn part1(&self) -> String {
        let file_content: String = FL_CONT.to_string();
		let initial_fuel_requirements: usize = file_content
			//Automatically split by line
			.lines()
			//Parse each line into a number and calc fuel
			.map(|lc| calculate_fuel(lc.parse().unwrap()))
			// Fold them all as a sum
			.fold(0,|a, b| a + b);
		return initial_fuel_requirements.to_string();
    }

	fn part2(&self) -> String {
		let file_content: String = FL_CONT.to_string();
		let fuel_requirements: usize = file_content
			//Automatically split by line
			.lines()
			//Parse each line into a number
			.map(|lc| lc.parse().unwrap())
			//Map each to their fuel requirements
			.map(|module| calculate_recursive_fuel(module))
			//Now map the recursive need
			// Fold them all as a sum
			.fold(0,|a, b| a + b);
		return fuel_requirements.to_string();
	}
}

fn calculate_recursive_fuel(fuel: usize) -> usize {
	// 9 / 3 = 3, then 3 - 2 = 1
	// 8 / 3 = 2.blah, then 2 - 2 = 0
	if fuel < 9 {
		return 0;
	}else{
		let fuel_needed = calculate_fuel(fuel);
		return fuel_needed + calculate_recursive_fuel(fuel_needed);
	}
}

fn calculate_fuel(weight: usize) -> usize {
	return (((weight as f64) / 3.0).floor() as usize) - 2;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_numbers() {
        assert_eq!(calculate_fuel(12),2);
		assert_eq!(calculate_fuel(14),2);
		assert_eq!(calculate_fuel(1969),654);
		assert_eq!(calculate_fuel(100756),33583);
		assert_eq!(calculate_recursive_fuel(12), 2);
		assert_eq!(calculate_recursive_fuel(1969), 966);
		assert_eq!(calculate_recursive_fuel(100756), 50346);
    }
}
