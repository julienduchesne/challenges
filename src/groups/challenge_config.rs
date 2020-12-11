use std::{collections::HashMap, fmt};

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub struct ChallengeError {
    message: String,
}

impl ChallengeError {
    pub fn new(message: &str) -> ChallengeError {
        return ChallengeError {
            message: message.to_string(),
        };
    }
}

impl fmt::Display for ChallengeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub trait ChallengeConfig {
    fn title(&self) -> &str;
    fn description(&self) -> &str;
    fn variables(&self) -> Vec<String>;
    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String, ChallengeError>;
    fn solve_string(&self, variables: HashMap<String, String>) -> Result<String, ChallengeError> {
        return self.solve(
            variables
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect(),
        );
    }
}
