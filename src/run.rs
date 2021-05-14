use std::fmt::Display;
use std::str::FromStr;

use color_eyre::eyre::Result;
use structopt::StructOpt;

use crate::day::Day;

#[derive(StructOpt)]
pub struct Run {
    /// Problem day to run
    day: Day,
    /// Part to run
    #[structopt(long, short, default_value)]
    part: Part,
}

#[derive(StructOpt)]
enum Part {
    Part1,
    Part2,
    Both,
}

impl FromStr for Part {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "part1" | "1" => Ok(Self::Part1),
            "part2" | "2" => Ok(Self::Part2),
            "both" | "b" => Ok(Self::Both),
            _ => Err("Unknown"),
        }
    }
}

impl Default for Part {
    fn default() -> Self {
        Self::Both
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Part1 => write!(f, "part1"),
            Part::Part2 => write!(f, "part2"),
            Part::Both => write!(f, "both"),
        }
    }
}

impl Run {
    pub fn run(&self) -> Result<String> {
        let code = self.day.get_code();
        let part = &self.part;
        let output = code.either(|code| match part{
            Part::Part1 => code.part1(),
            Part::Part2 => code.part2(),
            Part::Both => code.both(),
        }, |code| code.run());
        Ok(output)
    }
}
