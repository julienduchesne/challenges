use std::{collections::HashMap, error::Error};

use super::super::{
    challenge_config::ChallengeConfig, challenge_config::ChallengeError,
    challenge_config::VariableType,
};
use maplit::hashmap;

pub struct Challenge1 {}

impl ChallengeConfig for Challenge1 {
    fn id(&self) -> &str {
        return "1";
    }

    fn title(&self) -> &str {
        return "Multiples of 3 and 5";
    }

    fn description(&self) -> &str {
        return "If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
        Find the sum of all the multiples of 3 or 5 below {x}.";
    }

    fn variables(&self) -> HashMap<&str, crate::groups::challenge_config::VariableType> {
        return hashmap! {"x" => VariableType::Integer};
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String, ChallengeError> {
        let max: i64 = serde_json::from_str(variables["x"]).unwrap();
        return Ok("Answer".to_string());
    }
}
