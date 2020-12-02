use std::{collections::HashMap, error::Error};

use super::super::challenge_config::ChallengeConfig;
use super::super::challenge_config::ChallengeError;
use super::super::challenge_config::VariableType;
use maplit::hashmap;

pub struct Day1 {}

impl Day1 {
    fn solve_part_one(&self, numbers: Vec<i64>) -> Result<i64, ChallengeError> {
        for (x, first) in numbers.iter().enumerate() {
            for (y, second) in numbers.iter().enumerate() {
                if x == y {
                    continue;
                }
                if first + second == 2020 {
                    return Ok(first * second);
                }
            }
        }
        return Err(ChallengeError);
    }

    fn solve_part_two(&self, numbers: Vec<i64>) -> Result<i64, ChallengeError> {
        for (x, first) in numbers.iter().enumerate() {
            for (y, second) in numbers.iter().enumerate() {
                for (z, third) in numbers.iter().enumerate() {
                    if x == y || y == z || x == z {
                        continue;
                    }
                    if first + second + third == 2020 {
                        return Ok(first * second * third);
                    }
                }
            }
        }
        return Err(ChallengeError);
    }
}

impl ChallengeConfig for Day1 {
    fn id(&self) -> &str {
        return "Day 1";
    }

    fn title(&self) -> &str {
        return "Test1";
    }

    fn description(&self) -> &str {
        return "Something1";
    }

    fn variables(&self) -> HashMap<&str, crate::groups::challenge_config::VariableType> {
        return hashmap! {"report" => VariableType::MultiLineString};
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String, ChallengeError> {
        let report: &str = variables["report"];
        let numbers: Result<Vec<_>, _> = report
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i64>())
            .collect();
        if numbers.is_err() {
            return Err(ChallengeError);
        }

        let part_one = self.solve_part_one(numbers.clone().unwrap());
        let part_two = self.solve_part_two(numbers.clone().unwrap());
        if part_one.is_err() || part_two.is_err() {
            return Err(ChallengeError);
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
        case(vec![2019, 1], 2019),
        case(vec![2018, 1, 2], 4036),
        case(vec![1721, 979, 366, 299, 675, 1456], 514579),
    )]
    fn solve_part_one(input: Vec<i64>, expected: i64) {
        let day1 = Day1 {};
        assert_eq!(day1.solve_part_one(input).unwrap(), expected);
    }

    #[rstest(
        input,
        expected,
        case(vec![1721, 979, 366, 299, 675, 1456], 241861950),
    )]
    fn solve_part_two(input: Vec<i64>, expected: i64) {
        let day1 = Day1 {};
        assert_eq!(day1.solve_part_two(input).unwrap(), expected);
    }

    #[rstest(
        input,
        expected,
        case(
            "1721\n979\n366\n 299\n \n675\n1456\n",
            "Part 1: 514579\nPart 2: 241861950"
        )
    )]
    fn solve(input: &str, expected: &str) {
        let day1 = Day1 {};
        assert_eq!(day1.solve(hashmap! {"report" => input}).unwrap(), expected);
    }
}
