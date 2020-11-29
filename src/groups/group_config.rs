use super::challenge_config::ChallengeConfig;

pub trait GroupConfig {
    fn new() -> Self
    where
        Self: Sized;
    fn name(&self) -> &str;
    fn url(&self) -> &str;
    fn challenges(&self) -> &Vec<Box<dyn ChallengeConfig>>;
    fn challenge(&self, challenge_id: &str) -> Option<&Box<dyn ChallengeConfig>> {
        let mut id_to_find = challenge_id;
        if challenge_id.contains(":") {
            id_to_find = challenge_id.split(":").collect::<Vec<&str>>()[0];
        }
        return self.challenges().iter().find(|x| x.id() == id_to_find);
    }
    fn challenge_names(&self) -> Vec<String> {
        return self
            .challenges()
            .iter()
            .map(|x| format!("{id}: {title}", id = x.id(), title = x.title()))
            .collect();
    }
}
