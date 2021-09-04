use std::convert::TryInto;
use std::str::FromStr;

use aoc_2020::*;
use either::*;
use structopt::StructOpt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, StructOpt)]
pub struct Day {
	day: u32,
}

impl FromStr for Day {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let day: usize = s.parse().map_err(|_| "Day needs to be an integer")?;
		let day = day.try_into().unwrap();
		Ok(Self { day })
	}
}

impl Day {
	pub fn get_code(&self) -> Either<&dyn AoCDay, &dyn SinglePart> {
		match self.day {
			201901 => Left(&nineteen01::Code),
			201902 => Left(&nineteen02::Code),
			201903 => Right(&nineteen03::Code),
			201904 => Right(&nineteen04::Code),
			201905 => Left(&nineteen05::Code),
			201906 => Right(&nineteen06::Code),
			201907 => Left(&nineteen07::Code),
			201908 => Left(&nineteen08::Code),
			201909 => Left(&nineteen09::Code),
			201910 => Right(&nineteen10::Code),
			201911 => Left(&nineteen11::Code),
			201912 => Left(&nineteen12::Code),
			201913 => Left(&nineteen13::Code),
			201914 => Left(&nineteen14::Code),
			202022 => Left(&day22::Code),
			202023 => Left(&day23::Code),
			202024 => Left(&day24::Code),
			202025 => Right(&day25::Code),
			_ => unreachable!(),
		}
	}
}
