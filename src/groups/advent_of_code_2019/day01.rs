use std::collections::HashMap;

use anyhow::Result;

use crate::groups::challenge_config::{ChallengeConfig, ChallengeError};

pub struct Day1 {}

impl Day1 {}

impl ChallengeConfig for Day1 {
    fn title(&self) -> &str {
        return "Day 1: The Tyranny of the Rocket Equation";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["masses".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        return Ok(format!("Part 1: {}\nPart 2: {}", 0, 0).to_string());
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use rstest::rstest;

    use super::*;
}
