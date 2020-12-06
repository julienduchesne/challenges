use std::collections::HashMap;

use super::super::challenge_config::ChallengeConfig;
use super::super::challenge_config::ChallengeError;
use super::super::challenge_config::VariableType;
use maplit::hashmap;
use regex::Regex;

pub struct Day2 {}
#[derive(Clone)]
struct Line {
    min_chars: usize,
    max_chars: usize,
    letter: char,
    password: String,
}

impl Line {
    fn parse(line: &str) -> Result<Line, ChallengeError> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?P<min>\d+)-(?P<max>\d+)\s(?P<letter>\w{1}):\s(?P<password>\w+)")
                    .unwrap();
        }
        for caps in RE.captures_iter(line) {
            return Ok(Line {
                min_chars: (&caps["min"]).parse::<usize>().unwrap(),
                max_chars: (&caps["max"]).parse::<usize>().unwrap(),
                letter: (&caps["letter"]).to_owned().chars().nth(0).unwrap(),
                password: (&caps["password"]).to_owned(),
            });
        }
        return Err(ChallengeError::new(
            ("Could not parse line: ".to_owned() + line).as_str(),
        ));
    }
}

impl Day2 {
    fn solve_part_one(&self, lines: Vec<Line>) -> Result<i32, ChallengeError> {
        let mut total = 0;
        for line in lines {
            let char_count = line.password.matches(line.letter).count();
            if line.min_chars <= char_count && char_count <= line.max_chars {
                total = total + 1;
            }
        }
        return Ok(total);
    }

    fn solve_part_two(&self, lines: Vec<Line>) -> Result<i32, ChallengeError> {
        let mut total = 0;
        for line in lines {
            if (line.password.chars().nth(line.min_chars - 1).unwrap() == line.letter)
                ^ (line.password.chars().nth(line.max_chars - 1).unwrap() == line.letter)
            {
                total = total + 1;
            }
        }
        return Ok(total);
    }
}

impl ChallengeConfig for Day2 {
    fn id(&self) -> &str {
        return "Day 2";
    }

    fn title(&self) -> &str {
        return "Password Philosophy";
    }

    fn description(&self) -> &str {
        return "test2";
    }

    fn variables(&self) -> HashMap<&str, crate::groups::challenge_config::VariableType> {
        return hashmap! {"passwords" => VariableType::MultiLineString};
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String, ChallengeError> {
        let report: &str = variables["passwords"];
        let lines: Result<Vec<_>, _> = report
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(Line::parse)
            .collect();
        if lines.is_err() {
            return Err(ChallengeError::new(
                format!(
                    "Error parsing the input: {}",
                    lines.err().unwrap().to_string()
                )
                .as_str(),
            ));
        }

        let part_one = self.solve_part_one(lines.clone().unwrap());
        if part_one.is_err() {
            return Err(part_one.unwrap_err());
        }

        let part_two = self.solve_part_two(lines.clone().unwrap());
        if part_two.is_err() {
            return Err(part_two.unwrap_err());
        }
        return Ok(format!(
            "Part 1: {}\nPart 2: {}",
            part_one.unwrap(),
            part_two.unwrap()
        )
        .to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        input,
        expected,
        case(
            "1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc",
            "Part 1: 2\nPart 2: 1"
        )
    )]
    fn solve(input: &str, expected: &str) {
        let day = Day2 {};
        assert_eq!(
            day.solve(hashmap! {"passwords" => input}).unwrap(),
            expected
        );
    }
}
