use super::advent_of_code_2020::AdventOfCode2020;
use super::group_config::GroupConfig;
use super::project_euler::ProjectEuler;

pub struct GroupManager {
    groups: Vec<Box<dyn GroupConfig>>,
}

impl GroupManager {
    pub fn new() -> GroupManager {
        return GroupManager {
            groups: vec![Box::new(AdventOfCode2020 {}), Box::new(ProjectEuler {})],
        };
    }

    pub fn get_group_names(&self) -> Vec<String> {
        return self.groups.iter().map(|x| x.name().to_string()).collect();
    }
}
