use std::collections::HashMap;

use super::super::{challenge_config::ChallengeConfig, challenge_config::VariableType};
use maplit::hashmap;

pub struct Day1 {}

impl ChallengeConfig for Day1 {
    fn id(&self) -> &str {
        return "Day 1";
    }

    fn title(&self) -> &str {
        return "Test1";
    }

    fn description(&self) -> &str {
        return "Something1";
    }

    fn variables(&self) -> HashMap<&str, crate::groups::challenge_config::VariableType> {
        return hashmap! {};
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> &str {
        return "Answer";
    }
}
