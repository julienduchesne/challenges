use std::collections::HashMap;

use anyhow::Result;

use crate::groups::challenge_config::{ChallengeConfig, ChallengeError};

pub struct Day25 {}

impl Day25 {
    fn find_loop_size(key: u64) -> Result<u64> {
        let mut value = 1;
        for i in 1..100000000 {
            value = (value * 7) % 20201227;
            if value == key {
                return Ok(i);
            }
        }
        return Err(ChallengeError::new(
            format!(
                "Reached max iter while trying to find loop size for: {}",
                key
            )
            .as_str(),
        )
        .into());
    }
}

impl ChallengeConfig for Day25 {
    fn title(&self) -> &str {
        return "Day 25: Combo Breaker";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["Card public key".to_owned(), "Door public key".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        let card_public_key = variables["Card public key"].parse::<u64>()?;
        let door_public_key = variables["Door public key"].parse::<u64>()?;
        let card_loop_size = Self::find_loop_size(card_public_key)?;
        let mut result = 1;
        for _ in 0..card_loop_size {
            result = (result * door_public_key) % 20201227;
        }

        return Ok(format!("Result: {}", result).to_string());
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use rstest::rstest;

    use super::*;

    #[rstest(
        card_public_key,
        door_public_key,
        expected,
        case("5764801", "17807724", "Result: 14897079"),
        case("14205034", "18047856", "Result: 297257")
    )]
    fn solve(card_public_key: &str, door_public_key: &str, expected: &str) {
        let day = Day25 {};
        assert_eq!(
            day.solve(
                hashmap! {"Card public key" => card_public_key, "Door public key" => door_public_key }
            )
            .unwrap(),
            expected
        );
    }
}
