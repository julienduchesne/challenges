use std::collections::HashMap;

use anyhow::Result;

use super::super::challenge_config::ChallengeConfig;
use crate::groups::challenge_config::ChallengeError;

pub struct Day18 {}

impl Day18 {
    fn calculate(equation: &str, additions_first: bool) -> Result<usize> {
        let mut to_calc = equation.to_owned();

        while to_calc.contains(" ") {
            if to_calc.contains("(") {
                let mut open = 0;
                let mut close = 0;
                for (i, c) in to_calc.chars().enumerate() {
                    match c {
                        '(' => open = i,
                        ')' => {
                            close = i;
                            break;
                        }
                        _ => {}
                    }
                }
                to_calc.replace_range(
                    open..close + 1,
                    Self::calculate(&to_calc[open + 1..close], additions_first)?
                        .to_string()
                        .as_str(),
                );
                continue;
            }
            let split: Vec<String> = to_calc.split_whitespace().map(str::to_string).collect();
            let mut i = 0;
            let plus_index = split.iter().position(|r| *r == "+".to_string());
            if additions_first && plus_index.is_some() {
                i = plus_index.unwrap() - 1;
            }
            let first_number = split[i].parse::<usize>()?;
            let second_number = split[i + 2].parse::<usize>()?;
            let result = match split[i + 1].as_str() {
                "+" => first_number + second_number,
                "*" => first_number * second_number,
                _ => return Err(ChallengeError::new("Invalid operator").into()),
            };
            let mut new_value: Vec<String> = split[..i].to_vec();
            new_value.append(&mut vec![result.to_string()]);
            new_value.append(&mut split[i + 3..].to_vec());
            to_calc = new_value.join(" ");
        }
        return Ok(to_calc.parse::<usize>()?);
    }
}

impl ChallengeConfig for Day18 {
    fn title(&self) -> &str {
        return "Day 18: Operation Order";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["Homework".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        let lines: Vec<&str> = variables["Homework"]
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect();

        let mut part_one: usize = 0;
        let mut part_two: usize = 0;
        for line in lines {
            part_one += Self::calculate(line, false)?;
            part_two += Self::calculate(line, true)?;
        }

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use rstest::rstest;

    use super::*;

    #[rstest(
        homework,
        expected,
        case("1 + (2 * 3) + (4 * (5 + 6))", "Part 1: 51\nPart 2: 51"),
        case("2 * 3 + (4 * 5)", "Part 1: 26\nPart 2: 46"),
        case("5 + (8 * 3 + 9 + 3 * 4 * 3)", "Part 1: 437\nPart 2: 1445"),
        case(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
            "Part 1: 12240\nPart 2: 669060"
        ),
        case(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
            "Part 1: 13632\nPart 2: 23340"
        )
    )]
    fn solve(homework: &str, expected: &str) {
        let day = Day18 {};
        assert_eq!(
            day.solve(hashmap! {"Homework" => homework}).unwrap(),
            expected
        );
    }
}
