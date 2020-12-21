use super::{
    advent_of_code_2020::config::AdventOfCode2020, challenge_config::ChallengeConfig,
    group_config::GroupConfig,
};

pub struct GroupManager {
    groups: Vec<Box<dyn GroupConfig>>,
}

impl GroupManager {
    pub fn new() -> GroupManager {
        return GroupManager {
            groups: vec![Box::new(AdventOfCode2020::new())],
        };
    }

    pub fn get_group_names(&self) -> Vec<String> {
        return self.groups.iter().map(|x| x.name().to_string()).collect();
    }

    pub fn get_group(&self, group_name: &str) -> Option<&Box<dyn GroupConfig>> {
        return self.groups.iter().find(|x| x.name() == group_name);
    }

    pub fn get_group_challenge_names(&self, group_name: &str) -> Option<Vec<String>> {
        let group = self.get_group(group_name);
        if group.is_some() {
            return Some(group.unwrap().challenge_names());
        }
        return None;
    }

    pub fn get_challenge(
        &self,
        group_name: &str,
        challenge_name: &str,
    ) -> Option<&Box<dyn ChallengeConfig>> {
        let group = self.get_group(group_name);
        if group.is_some() {
            return group.unwrap().challenge(challenge_name);
        }
        return None;
    }
}
