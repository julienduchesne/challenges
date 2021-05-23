use serde::{Deserialize, Serialize};

use super::{challenge_config::ChallengeConfig, group_config::GroupConfig};

#[derive(Serialize, Deserialize)]
pub struct ApiChallenge {
    id: String,
    title: String,
    description: String,
    port: Option<i32>,
}

impl ChallengeConfig for ApiChallenge {
    fn title(&self) -> &str {
        return self.title.as_str();
    }

    fn description(&self) -> &str {
        return self.description.as_str();
    }

    fn solve(&self, input: &str) -> anyhow::Result<String> {
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!(
                "http://localhost:{}/solve/{}",
                self.port.unwrap(),
                self.id
            ))
            .body(String::from(input))
            .send()?;
        return Ok(res.text()?);
    }
}

pub struct ApiGroupConfig {
    name: String,
    challenges_url: String,
    challenges: Vec<Box<dyn ChallengeConfig>>,
}

impl ApiGroupConfig {
    pub fn new(name: &str, challenges_url: &str, default_port: i32, port_envvar: &str) -> Self
    where
        Self: Sized,
    {
        let port = match std::env::var(port_envvar) {
            Ok(p) => p.parse::<i32>().unwrap_or(default_port),
            Err(_) => default_port,
        };
        let challenges = match ApiGroupConfig::list_challenges(port) {
            Ok(v) => v
                .into_iter()
                .map(|e| Box::new(e) as Box<dyn ChallengeConfig>)
                .collect::<Vec<_>>(),
            Err(err) => {
                eprintln!(
                    "Got an error while listing challenges for Advent of Code 2019: {}",
                    err
                );
                vec![]
            }
        };
        return ApiGroupConfig {
            name: String::from(name),
            challenges_url: String::from(challenges_url),
            challenges: challenges,
        };
    }

    fn list_challenges(port: i32) -> anyhow::Result<Vec<ApiChallenge>> {
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(format!("http://localhost:{}/list/", port))
            .send()?;
        let mut v: Vec<ApiChallenge> = res.json()?;
        v.iter_mut().for_each(|i| i.port = Some(port));
        return Ok(v);
    }
}

impl GroupConfig for ApiGroupConfig {
    fn name(&self) -> &str {
        return self.name.as_str();
    }
    fn url(&self) -> &str {
        return self.challenges_url.as_str();
    }

    fn challenges(&self) -> &Vec<Box<dyn ChallengeConfig>> {
        return &self.challenges;
    }
}
