#![feature(drain_filter)]
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
    // pub mod nineteen01;
    // pub mod nineteen02;
    // pub mod nineteen03;
    // pub mod nineteen04;
    // pub mod nineteen05;
    // pub mod nineteen06;
    // pub mod nineteen07;
    // pub mod nineteen08;
    // pub mod nineteen09;
    // pub mod nineteen10;
    // pub mod nineteen11;
    // pub mod nineteen12;
    // pub mod nineteen13;
    // pub mod nineteen14;
    //pub mod nineteen15;
    //pub mod nineteen16;
    //pub mod nineteen17;
    //pub mod nineteen18;
    //pub mod nineteen19;
    //pub mod nineteen20;
    //pub mod nineteen21;
    //pub mod nineteen22;
    //pub mod nineteen23;
    pub mod nineteen25;
    // pub mod day22;
    // pub mod day23;
    // pub mod day24;
    // pub mod day25;
}

pub use solutions::*;
pub mod intcode;
pub mod image_layer;
mod coords;
mod direction;
mod map;
//mod graph;
mod cell;
//mod torus_map;