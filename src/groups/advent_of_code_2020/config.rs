use super::super::{challenge_config::ChallengeConfig, group_config::GroupConfig};

pub struct AdventOfCode2020 {
    challenges: Vec<Box<dyn ChallengeConfig>>,
}

impl GroupConfig for AdventOfCode2020 {
    fn new() -> Self
    where
        Self: Sized,
    {
        return AdventOfCode2020 { challenges: vec![] };
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
