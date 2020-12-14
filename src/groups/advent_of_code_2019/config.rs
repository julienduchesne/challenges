
use super::{
    super::{challenge_config::ChallengeConfig, group_config::GroupConfig},
    day01::Day1,
};

pub struct AdventOfCode2019 {
    challenges: Vec<Box<dyn ChallengeConfig>>,
}

impl GroupConfig for AdventOfCode2019 {
    fn new() -> Self
    where
        Self: Sized,
    {
        return AdventOfCode2019 {
            challenges: vec![Box::new(Day1 {})],
        };
    }

    fn name(&self) -> &str {
        return "Advent of Code 2019";
    }
    fn url(&self) -> &str {
        return "https://adventofcode.com/2019";
    }

    fn challenges(&self) -> &Vec<Box<dyn ChallengeConfig>> {
        return &self.challenges;
    }
}
