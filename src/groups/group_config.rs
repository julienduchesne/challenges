use super::challenge_config::ChallengeConfig;

pub trait GroupConfig {
    fn new() -> Self
    where
        Self: Sized;
    fn name(&self) -> &str;
    fn url(&self) -> &str;
    fn challenges(&self) -> &Vec<Box<dyn ChallengeConfig>>;
    fn challenge_names(&self) -> Vec<String> {
        return self
            .challenges()
            .iter()
            .map(|x| format!("{id}: {title}", id = x.id(), title = x.title()))
            .collect();
    }
}
