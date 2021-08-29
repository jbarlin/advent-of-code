use std::collections::HashMap;

use regex::Regex;

use crate::AoCDay;

pub struct Code;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Ingredient {
	nm: String,
	qty: u64,
}

impl Ingredient {
	fn from_str(qty: &str, nm: &str) -> Ingredient {
		Ingredient {
			nm: nm.to_owned(),
			qty: qty.parse().unwrap(),
		}
	}

	fn new(qty: u64, nm: String) -> Ingredient {
		Ingredient { nm, qty }
	}
	fn multiply(&self, mult_by: u64) -> Ingredient {
		Ingredient::new(self.qty * mult_by, self.nm.clone())
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Process {
	makes: Ingredient,
	from: Vec<Ingredient>,
}

impl Process {
	fn new(makes: Ingredient, from: Vec<Ingredient>) -> Process {
		Process { makes, from }
	}
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct NanoFactory {
	//Map from Name (String) -> Process
	grimoire: HashMap<String, Process>,
	stored: HashMap<String, u64>,
}

enum ToMake {
	None,
	Some(u64),
	All(u64),
}

type OreCost = u64;

impl NanoFactory {
	fn from_reactions_list(input: &str) -> NanoFactory {
		let mut nano = NanoFactory { grimoire: HashMap::new(), stored: HashMap::new() };
		// split from => makes
		let process_regx: Regex = Regex::new(r"(?m)^(.+?)=> (.+?)$").unwrap();
		// parse qty nm
		let ingredient_regx: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
		for line in process_regx.captures_iter(input) {
			let from: &str = &line[1];
			let to: &str = &line[2];

			let mut inputs: Vec<Ingredient> = Vec::new();
			for ingreds in ingredient_regx.captures_iter(from) {
				inputs.push(Ingredient::from_str(
					&ingreds[1],
					&ingreds[2],
				));
			}
			let makes_cap = ingredient_regx.captures(to).unwrap();
			let makes = Ingredient::from_str(&makes_cap[1], &makes_cap[2]);
			nano.grimoire.insert(makes.nm.clone(), Process::new(makes, inputs));
		}
		return nano;
	}

	fn use_leftovers(&mut self, to_make: &Ingredient) -> ToMake {
		//OK, we have 3 possible states.
		// 1 - we have it, don't make it
		// 2 - we have *some*, but not enough
		// 3 - we don't have any, make it all
		return if let Some(amount) = self.stored.get_mut(&to_make.nm) {
			if to_make.qty <= *amount {
				//State 1 = don't make any
				*amount -= to_make.qty;
				ToMake::None
			} else {
				//State 2 = make *some*
				let ret = ToMake::Some(to_make.qty - *amount);
				self.stored.remove(&to_make.nm);
				ret
			}
		} else {
			//State 3 = make it all!
			ToMake::All(to_make.qty)
		};
	}

	fn make_desired_fuel(&mut self, desired_fuel: u64) -> OreCost {
		let mut ore_cost: OreCost = 0;
		let fuel = Ingredient::new(desired_fuel, "FUEL".to_owned());
		let mut requirement_list = vec![fuel];
		while let Some(need_to_make) = requirement_list.pop() {
			if need_to_make.nm == "ORE" {
				ore_cost += need_to_make.qty
			} else {
				//OK, I actually have to make something here!
				//let's see if we have any leftovers first up!
				let remaining = self.use_leftovers(&need_to_make);
				match remaining {
					ToMake::None => {}
					ToMake::Some(remainder) | ToMake::All(remainder) => {
						//Find a process that makes this!
						let process = self.grimoire
							.get(&need_to_make.nm)
							.expect(&*format!("Must have some way of making {}", need_to_make.nm));

						let multiplier = (remainder as f64 / process.makes.qty as f64).ceil() as u64;

						if let Some(checkd_mult) = (process.makes.qty * multiplier).checked_sub(remainder) {
							if checkd_mult > 0 {
								*self.stored.entry(need_to_make.nm).or_insert(0) += checkd_mult;
							}
						}

						for ntm in process.from.iter() {
							requirement_list.push(ntm.multiply(multiplier));
						}
					}
				}
			}
		}

		return ore_cost;
	}
}

pub fn part_1_impl(input: &str) -> String {
	NanoFactory::from_reactions_list(input).make_desired_fuel(1).to_string()
}

// Work out what straight dividing it would be (worst case) fuel creation,
// And then massively increase that (double? triple?) that as a goal so we can kinda binary search the possible values?
fn target_ore_use(nfac: NanoFactory, target_ore_cost: OreCost) -> String {
	let cost_one: OreCost = nfac.clone().make_desired_fuel(1);
	//Worst case
	let mut fuel_min_case: OreCost = target_ore_cost / cost_one;
	//Excessive beyond scope case
	let mut fuel_max_case: OreCost = fuel_min_case * 3;
	//While there is a greater difference than 1 we should be binary searching
	while fuel_max_case - fuel_min_case > 1 {
		//look at the middle case, establish if higher or lower
		let desired_fuel = (fuel_min_case + fuel_max_case) / 2;
		let ore_cost = nfac.clone().make_desired_fuel(desired_fuel);
		if ore_cost == target_ore_cost {
			//Lol, just return this
			return desired_fuel.to_string();
		} else if ore_cost < target_ore_cost {
			fuel_min_case = desired_fuel;
		} else {
			fuel_max_case = desired_fuel;
		}
	}
	//Fuel max and fuel min should be the same OR max should be one above the min
	// min should be correct tho since it's always going to come under (looking at the above code)?
	fuel_min_case.to_string()
}


pub fn part_2_impl(input: &str) -> String {
	target_ore_use(NanoFactory::from_reactions_list(input), 1_000_000_000_000)
}

pub const DAY_14_DATA: &str = include_str!("../../inputs/2019/Day14.txt");

impl AoCDay for Code {
	fn part1(&self) -> String {
		return part_1_impl(DAY_14_DATA);
	}

	fn part2(&self) -> String {
		return part_2_impl(DAY_14_DATA);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const STR_A: &str = include_str!("../../inputs/2019/Day14-eg-a.txt");
	const STR_B: &str = include_str!("../../inputs/2019/Day14-eg-b.txt");
	const STR_C: &str = include_str!("../../inputs/2019/Day14-eg-c.txt");
	const STR_D: &str = include_str!("../../inputs/2019/Day14-eg-d.txt");
	const STR_E: &str = include_str!("../../inputs/2019/Day14-eg-e.txt");

	#[test]
	fn test_example_a() {
		assert_eq!(part_1_impl(STR_A), "31");
	}

	#[test]
	fn test_example_b() {
		assert_eq!(part_1_impl(STR_B), "165");
	}

	#[test]
	fn test_example_c() {
		assert_eq!(part_1_impl(STR_C), "13312");
		assert_eq!(part_2_impl(STR_C), "82892753");
	}

	#[test]
	fn test_example_d() {
		assert_eq!(part_1_impl(STR_D), "180697");
		assert_eq!(part_2_impl(STR_D), "5586022");
	}

	#[test]
	fn test_example_e() {
		assert_eq!(part_1_impl(STR_E), "2210736");
		assert_eq!(part_2_impl(STR_E), "460664");
	}
}