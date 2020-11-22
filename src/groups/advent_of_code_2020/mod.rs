use super::group_config::GroupConfig;

pub struct AdventOfCode2020 {}

impl GroupConfig for AdventOfCode2020 {
    fn name(&self) -> &'static str {
        return "Advent of Code 2020";
    }
    fn url(&self) -> &'static str {
        return "https://adventofcode.com/";
    }
}
