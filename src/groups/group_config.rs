use super::challenge_config::ChallengeConfig;

pub trait GroupConfig {
    fn new() -> Self
    where
        Self: Sized;
    fn name(&self) -> &str;
    fn url(&self) -> &str;
    fn challenges(&self) -> &Vec<Box<dyn ChallengeConfig>>;
    fn challenge(&self, challenge_title: &str) -> Option<&Box<dyn ChallengeConfig>> {
        return self
            .challenges()
            .iter()
            .find(|x| x.title() == challenge_title);
    }
    fn challenge_names(&self) -> Vec<String> {
        return self
            .challenges()
            .iter()
            .map(|x| x.title().to_string())
            .collect();
    }
}
