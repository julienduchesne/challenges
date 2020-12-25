use super::{
    super::{challenge_config::ChallengeConfig, group_config::GroupConfig},
    challenge0001::Challenge1,
};

pub struct ProjectEuler {
    challenges: Vec<Box<dyn ChallengeConfig>>,
}

impl GroupConfig for ProjectEuler {
    fn new() -> Self
    where
        Self: Sized,
    {
        return ProjectEuler {
            challenges: vec![Box::new(Challenge1 {})],
        };
    }

    fn name(&self) -> &str {
        return "Project Euler";
    }

    fn url(&self) -> &str {
        return "https://projecteuler.net/";
    }

    fn challenges(&self) -> &Vec<Box<dyn ChallengeConfig>> {
        return &self.challenges;
    }
}
