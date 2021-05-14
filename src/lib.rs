pub trait SinglePart{
    fn run(&self) -> String;
}

pub trait AoCDay{
    fn part1(&self) -> String;
    fn part2(&self) -> String;
    /// This method should be implemented if solving both parts together is more efficient than doing them one at a time
    fn both(&self) -> String {
        let p1 = self.part1();
        let p2 = self.part2();
        format!(
            "Part1: {}\n\
            Part2: {}",
            p1, p2
        )
    }
}

pub mod solutions {
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
}

pub use solutions::*;

