use std::collections::HashMap;

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
    fn solve(&self, variables: HashMap<&str, &str>) -> &str;
}
