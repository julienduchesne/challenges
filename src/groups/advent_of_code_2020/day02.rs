use std::{collections::HashMap, error::Error};

use crate::groups::challenge_config::ChallengeError;

use super::super::challenge_config::ChallengeConfig;
use maplit::hashmap;

pub struct Day2 {}

impl ChallengeConfig for Day2 {
    fn id(&self) -> &str {
        return "Day 2";
    }

    fn title(&self) -> &str {
        return "Test2";
    }

    fn description(&self) -> &str {
        return "Something2";
    }

    fn variables(&self) -> HashMap<&str, crate::groups::challenge_config::VariableType> {
        return hashmap! {};
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String, ChallengeError> {
        return Ok("Answer".to_string());
    }
}
