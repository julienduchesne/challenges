use super::group_config::GroupConfig;

pub struct ProjectEuler {}

impl GroupConfig for ProjectEuler {
    fn name(&self) -> &'static str {
        return "Project Euler";
    }
    fn url(&self) -> &'static str {
        return "https://projecteuler.net/";
    }
}
