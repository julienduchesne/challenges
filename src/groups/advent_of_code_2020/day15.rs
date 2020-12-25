use std::collections::HashMap;

use anyhow::Result;

use crate::groups::challenge_config::ChallengeConfig;

pub struct Day15 {}

impl Day15 {}

impl ChallengeConfig for Day15 {
    fn title(&self) -> &str {
        return "Day 15: Rambunctious Recitation";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["starting_numbers".to_owned(), "nth_number".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        let nth_number: usize = variables["nth_number"].parse::<usize>()?;
        let numbers: Vec<usize> = variables["starting_numbers"]
            .split(",")
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .map(str::parse::<usize>)
            .collect::<Result<_, _>>()?;

        let mut last_spoken: HashMap<usize, usize> = HashMap::new();
        for i in 0..numbers.len() - 1 {
            last_spoken.insert(numbers[i], i);
        }

        let mut last: usize = *numbers.last().unwrap();
        for i in numbers.len()..nth_number {
            let new_number = match last_spoken.get(&last) {
                Some(index) => i - index - 1,
                _ => 0,
            };
            last_spoken.insert(last, i - 1);
            last = new_number;
        }

        return Ok(format!("Result: {}", last).to_string());
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use rstest::rstest;

    use super::*;

    #[rstest(
        nth_number,
        starting_numbers,
        expected,
        case("2020", "0,3,6", "Result: 436"),
        case("2020", "1,3,2", "Result: 1"),
        case("2020", "2,1,3", "Result: 10"),
        case("2020", "1,2,3", "Result: 27"),
        case("2020", "2,3,1", "Result: 78"),
        case("2020", "3,2,1", "Result: 438"),
        case("2020", "3,1,2", "Result: 1836"),
        case("2020", "14,8,16,0,1,17", "Result: 240")
    )]
    fn solve(nth_number: &str, starting_numbers: &str, expected: &str) {
        let day = Day15 {};
        assert_eq!(
            day.solve(
                hashmap! {"nth_number" => nth_number, "starting_numbers" => starting_numbers}
            )
            .unwrap(),
            expected
        );
    }
}
