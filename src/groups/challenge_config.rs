use std::{collections::HashMap, error::Error, fmt};

#[derive(Debug, Clone)]
pub struct ChallengeError;

impl fmt::Display for ChallengeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}
pub enum VariableType {
    String,
    MultiLineString,
    Integer,
}
pub trait ChallengeConfig {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
    fn description(&self) -> &str;
    fn variables(&self) -> HashMap<&str, VariableType>;
    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String, ChallengeError>;
}
