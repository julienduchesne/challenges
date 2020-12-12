use super::{
    super::{challenge_config::ChallengeConfig, group_config::GroupConfig},
    day01::Day1,
    day02::Day2,
    day03::Day3,
    day04::Day4,
    day05::Day5,
    day06::Day6,
    day07::Day7,
    day08::Day8,
    day09::Day9,
    day10::Day10,
    day11::Day11,
    day12::Day12,
};

pub struct AdventOfCode2020 {
    challenges: Vec<Box<dyn ChallengeConfig>>,
}

impl GroupConfig for AdventOfCode2020 {
    fn new() -> Self
    where
        Self: Sized,
    {
        return AdventOfCode2020 {
            challenges: vec![
                Box::new(Day1 {}),
                Box::new(Day2 {}),
                Box::new(Day3 {}),
                Box::new(Day4 {}),
                Box::new(Day5 {}),
                Box::new(Day6 {}),
                Box::new(Day7 {}),
                Box::new(Day8 {}),
                Box::new(Day9 {}),
                Box::new(Day10 {}),
                Box::new(Day11 {}),
                Box::new(Day12 {}),
            ],
        };
    }

    fn name(&self) -> &str {
        return "Advent of Code 2020";
    }
    fn url(&self) -> &str {
        return "https://adventofcode.com/";
    }

    fn challenges(&self) -> &Vec<Box<dyn ChallengeConfig>> {
        return &self.challenges;
    }
}
