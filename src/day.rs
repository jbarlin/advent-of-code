use std::convert::TryInto;
use std::num::NonZeroU8;
use std::str::FromStr;

use aoc_2020::*;
use either::*;
use structopt::StructOpt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, StructOpt)]
pub struct Day {
    day: NonZeroU8,
}

impl FromStr for Day {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day: u8 = s.parse().map_err(|_| "Day needs to be an integer")?;
        if day < 1 || day > 25 {
            Err("Day value needs to be between 1 and 25 (inclusive)")
        } else {
            let day = day.try_into().unwrap();
            Ok(Self { day })
        }
    }
}

impl Day {
    pub fn get_code(&self) -> Either<&dyn AoCDay, &dyn SinglePart> {
        match self.day.get() {
        22 => Left(&day22::Code),
        23 => Left(&day23::Code),
        24 => Left(&day24::Code),
        25 => Right(&day25::Code),
        _ => unreachable!(),
        }
    }
}
