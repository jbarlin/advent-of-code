use crate::AoCDay;

pub struct Code;

pub const DAY_22_DATA: &str = include_str!("../../inputs/2019/Day22.txt");

#[derive(Clone, Copy, Debug)]
enum ShuffleAction {
	Stack,
	Cut(i64),
	Increment(i64),
}

impl ShuffleAction {
	fn apply(&self, card_num: i64, number_cards: &i64) -> i64 {
		//Rather than track all the cards, let's just apply transforms that work on primes to get the answer
		match self {
			ShuffleAction::Stack => (-card_num - 1).rem_euclid(*number_cards),
			ShuffleAction::Cut(x) => (card_num - x).rem_euclid(*number_cards),
			ShuffleAction::Increment(x) => (card_num * x).rem_euclid(*number_cards)
		}
	}

	fn can_combine(&self, other: &ShuffleAction) -> bool {
		match self {
			ShuffleAction::Stack => {
				match other {
					_ => true
				}
			}
			ShuffleAction::Cut(_) => {
				match other {
					ShuffleAction::Stack => false,
					_ => true
				}
			}
			ShuffleAction::Increment(_) => {
				match other {
					ShuffleAction::Increment(_) => true,
					_ => false
				}
			}
		}
	}

	fn combine(&self, other: &ShuffleAction, number_cards: &i64) -> Vec<ShuffleAction> {
		match self {
			ShuffleAction::Stack => {
				match other {
					ShuffleAction::Stack => Vec::with_capacity(0),
					ShuffleAction::Cut(x) => vec![ShuffleAction::Cut((*number_cards) - (*x)), ShuffleAction::Stack],
					ShuffleAction::Increment(n) => {
						vec![
							ShuffleAction::Increment(*n),
							ShuffleAction::Cut((*number_cards) + 1 - (*n)),
							ShuffleAction::Stack
						]
					}
				}
			}
			ShuffleAction::Cut(x) => {
				match other {
					ShuffleAction::Cut(y) => { vec![ShuffleAction::Cut((x + y).rem_euclid(*number_cards))] }
					ShuffleAction::Increment(n) => {
						vec![ShuffleAction::Increment(*n), ShuffleAction::Cut(
							((*x as i128) * (*n as i128)).rem_euclid((*number_cards) as i128) as i64
						)]
					}
					_ => unreachable!()
				}
			}
			ShuffleAction::Increment(x) => {
				match other {
					ShuffleAction::Increment(n) => {
						vec![ShuffleAction::Increment(((*x as i128) * (*n as i128)).rem_euclid((*number_cards) as i128) as i64)]
					}
					_ => unreachable!()
				}
			}
		}
	}
}

fn collapse(o_actions: &Vec<ShuffleAction>, number_cards: &i64) -> Vec<ShuffleAction> {
	let mut actions: Vec<ShuffleAction> = o_actions.clone();
	while actions.len() > 2 {
		let mut offset: usize = 0;
		while offset < actions.len() - 1 {
			if actions[offset].can_combine(&actions[offset + 1]) {
				let n_actions: Vec<ShuffleAction> = actions[offset].combine(&actions[offset + 1], number_cards);
				let act_size: usize = actions.len();
				let act_to_offset: Vec<ShuffleAction> = Vec::from(actions.split_at(offset).0);
				let act_after_offset: Vec<ShuffleAction> = if act_size >= offset + 2 {
					Vec::from(actions.split_at(offset + 2).1)
				} else {
					Vec::with_capacity(0)
				};
				actions = match n_actions.len() {
					0 => Vec::with_capacity(actions.len() - 2),
					1 => Vec::with_capacity(actions.len() - 1),
					3 => Vec::with_capacity(actions.len() + 1),
					_ => Vec::with_capacity(actions.len())
				};
				act_to_offset
					.iter()
					.for_each(|a| {
						actions.push(a.clone());
					});
				n_actions
					.iter()
					.for_each(|a| {
						actions.push(a.clone());
					});
				act_after_offset
					.iter()
					.for_each(|a| {
						actions.push(a.clone());
					});
				if offset != 0 {
					offset = offset - 1;
				}
			} else {
				offset = offset + 1;
			}
		}
	}
	return actions;
}

fn parse(input: &str) -> Vec<ShuffleAction> {
	let mut actions: Vec<ShuffleAction> = Vec::with_capacity(input.lines().count());
	input
		.lines()
		.for_each(|ln| {
			if ln.starts_with("deal into new stack") {
				actions.push(ShuffleAction::Stack)
			} else if ln.starts_with("cut ") {
				actions.push(ShuffleAction::Cut(ln.replace("cut ", "").parse::<i64>().unwrap()));
			} else if ln.starts_with("deal with increment ") {
				actions.push(ShuffleAction::Increment(ln.replace("deal with increment ", "").parse::<i64>().unwrap()));
			} else {
				panic!("No actions readable from {}", ln)
			}
		});
	return actions;
}

fn repeat_n_times(actions: Vec<ShuffleAction>, num_times: &i64, num_cards: &i64) -> Vec<ShuffleAction>{
	let mut curr: Vec<ShuffleAction> = actions.clone();
	let mut res: Vec<ShuffleAction> = actions.clone();
	//24805322184776 high
	//22781838649882 ? no
	//56620343048299
	//39511854373510
	//8956447112894 ? no
	let mut iters_left: i64 = *num_cards - *num_times - 1;
	while iters_left != 0 {
		if iters_left % 2 == 1 {
			res
				.clone()
				.iter()
				.for_each(|x| {
					curr.push(x.clone());
				});
			curr = collapse(&curr, num_cards);
		}
		res
			.clone()
			.iter()
			.for_each(|x| {
				res.push(x.clone());
			});
		res = collapse(&res, num_cards);
		iters_left = iters_left / 2;
	}
	return curr;
}

impl AoCDay for Code {
	fn part1(&self) -> String {
		let mut card_num: i64 = 2019;
		let num_cards: i64 = 10007;
		let actions = collapse(&parse(DAY_22_DATA), &num_cards);
		actions
			.iter()
			.for_each(|action| {
				card_num = action.apply(card_num, &num_cards);
			});
		return card_num.to_string();
	}

	fn part2(&self) -> String {
		let num_cards: i64 = 119315717514047;
		let num_times: i64 = 101741582076661;
		let mut card_num: i64 = 2020;
		
		let mut actions = collapse(&parse(DAY_22_DATA), &num_cards);
		actions = repeat_n_times(actions, &num_times, &num_cards);
		actions
			.iter()
			.for_each(|action| {
				match action {
					ShuffleAction::Stack => {
						card_num = num_cards - 1 - card_num;
					}
					ShuffleAction::Cut(x) => {
						if card_num < (*x){
							card_num = card_num + (num_cards - (*x))
						}else {
							card_num = card_num - (*x)
						}
					}
					ShuffleAction::Increment(x) => {
						card_num = (((card_num as i128) * ((*x) as i128)) % ((num_cards) as i128)) as i64
					}
				}
			});
		return card_num.to_string();
	}
}
