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
    day13::Day13,
    day14::Day14,
    day15::Day15,
    day16::Day16,
    day17::Day17,
    day18::Day18,
    day19::Day19,
    day20::Day20,
    day21::Day21,
    day22::Day22,
    day23::Day23,
    day24::Day24,
    day25::Day25,
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
                Box::new(Day13 {}),
                Box::new(Day14 {}),
                Box::new(Day15 {}),
                Box::new(Day16 {}),
                Box::new(Day17 {}),
                Box::new(Day18 {}),
                Box::new(Day19 {}),
                Box::new(Day20 {}),
                Box::new(Day21 {}),
                Box::new(Day22 {}),
                Box::new(Day23 {}),
                Box::new(Day24 {}),
                Box::new(Day25 {}),
            ],
        };
    }

    fn name(&self) -> &str {
        return "Advent of Code 2020";
    }
    fn url(&self) -> &str {
        return "https://adventofcode.com/2020";
    }

    fn challenges(&self) -> &Vec<Box<dyn ChallengeConfig>> {
        return &self.challenges;
    }
}
