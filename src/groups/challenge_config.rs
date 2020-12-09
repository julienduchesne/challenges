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

#[derive(Clone, PartialEq)]
pub enum VariableType {
    Integer,
    MultiLineString,
    String,
}
pub trait ChallengeConfig {
    fn title(&self) -> &str;
    fn description(&self) -> &str;
    fn variables(&self) -> HashMap<String, VariableType>;
    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String, ChallengeError>;
}
