use std::fmt;

use anyhow::Result;
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
    fn description(&self) -> &str {
        return "";
    }
    fn solve(&self, input: &str) -> Result<String>;
}
