use std::{
    io::Write,
    process::{Command, Stdio},
};

use serde::{Deserialize, Serialize};

use super::super::{challenge_config::ChallengeConfig, group_config::GroupConfig};

#[derive(Serialize, Deserialize)]
pub struct GoChallenge {
    id: i32,
    title: String,
    description: String,
}

impl ChallengeConfig for GoChallenge {
    fn title(&self) -> &str {
        return self.title.as_str();
    }

    fn description(&self) -> &str {
        return self.description.as_str();
    }

    fn solve(&self, input: &str) -> anyhow::Result<String> {
        let mut command = Command::new("./advent_of_code_2019")
            .current_dir("/Users/julienduchesne/Repos/challenges-rust-tui/target/debug") // TODO: Packaging
            .arg("solve")
            .arg(self.id.to_string())
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()?;
        let stdin = command.stdin.as_mut().unwrap();
        stdin.write_all(input.as_bytes())?;
        // Close stdin to finish and avoid indefinite blocking
        drop(stdin);
        let output = String::from_utf8(command.wait_with_output()?.stdout)?;
        return Ok(output);
    }
}

pub struct AdventOfCode2019 {
    port: i32,
    challenges: Vec<Box<dyn ChallengeConfig>>,
}

impl AdventOfCode2019 {
    fn list_challenges(port: i32) -> anyhow::Result<Vec<GoChallenge>> {
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(format!("http://localhost:{}/list", port))
            .send()?;
        let v: Vec<GoChallenge> = res.json()?;
        return Ok(v);
    }
}

impl GroupConfig for AdventOfCode2019 {
    fn new() -> Self
    where
        Self: Sized,
    {
        let port = match std::env::var("CHALLENGES_AOC_2019_PORT") {
            Ok(p) => p.parse::<i32>().unwrap_or(8082),
            Err(_) => 8082,
        };
        let challenges = match AdventOfCode2019::list_challenges(port) {
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
        return AdventOfCode2019 {
            port: port,
            challenges: challenges,
        };
    }

    fn name(&self) -> &str {
        return "Advent of Code 2019";
    }
    fn url(&self) -> &str {
        return "https://adventofcode.com/2019";
    }

    fn challenges(&self) -> &Vec<Box<dyn ChallengeConfig>> {
        return &self.challenges;
    }
}
