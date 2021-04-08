use std::{
    io::Write,
    process::{Command, Stdio},
};

use serde::{Deserialize, Serialize};

use super::super::{challenge_config::ChallengeConfig, group_config::GroupConfig};

#[derive(Serialize, Deserialize)]
pub struct GoChallenge {
    id: String,
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
            .arg("solve")
            .arg(self.id.as_str())
            .stdin(Stdio::piped())
            .spawn()?;
        let stdin = command.stdin.as_mut().unwrap();
        stdin.write_all(input.as_bytes())?;
        // Close stdin to finish and avoid indefinite blocking
        drop(stdin);
        return Ok(String::from_utf8(command.wait_with_output()?.stdout)?);
    }
}

pub struct AdventOfCode2019 {}

impl AdventOfCode2019 {
    fn list_challenges() -> anyhow::Result<Vec<GoChallenge>> {
        let output = Command::new("./advent_of_code_2019").arg("list").output()?;
        let output_str = String::from_utf8(output.stdout)?;
        let v: Vec<GoChallenge> = serde_json::from_str(&output_str)?;
        return Ok(v);
    }
}

impl GroupConfig for AdventOfCode2019 {
    fn new() -> Self
    where
        Self: Sized,
    {
        return AdventOfCode2019 {};
    }

    fn name(&self) -> &str {
        return "Advent of Code 2019";
    }
    fn url(&self) -> &str {
        return "https://adventofcode.com/2019";
    }

    fn challenges(&self) -> &Vec<Box<dyn ChallengeConfig>> {
        let challenges = match AdventOfCode2019::list_challenges() {
            Ok(v) => v
                .into_iter()
                .map(|e| Box::new(e) as Box<dyn ChallengeConfig>)
                .collect::<Vec<_>>(),
            Err(err) => {
                eprintln!(
                    "Got an error while listing challenges for Advent of Code 2019: {}",
                    err
                );
                return &vec![];
            }
        };
        return &challenges;
    }
}
