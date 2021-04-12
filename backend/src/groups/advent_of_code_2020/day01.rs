use anyhow::Result;

use crate::groups::challenge_config::{ChallengeConfig, ChallengeError};

pub struct Day1 {}

impl Day1 {
    fn solve_part_one(&self, numbers: Vec<isize>) -> Result<isize> {
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
        return Err(ChallengeError::new("Could not find a solution for part one").into());
    }

    fn solve_part_two(&self, numbers: Vec<isize>) -> Result<isize> {
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
        return Err(ChallengeError::new("Could not find a solution for part two").into());
    }
}

impl ChallengeConfig for Day1 {
    fn title(&self) -> &str {
        return "Day 1: Report Repair";
    }

    fn description(&self) -> &str {
        return "Part 1: Find the two entries that sum to 2020; what do you get if you multiply them together?
Part 2: What is the product of the three entries that sum to 2020?";
    }

    fn solve(&self, input: &str) -> Result<String> {
        let numbers: Vec<isize> = input
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<isize>())
            .collect::<Result<Vec<isize>, _>>()?;

        let part_one = self.solve_part_one(numbers.clone())?;
        let part_two = self.solve_part_two(numbers.clone())?;
        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        input,
        expected,
        case(vec![2019, 1], 2019),
        case(vec![2018, 1, 2], 4036),
        case(vec![1721, 979, 366, 299, 675, 1456], 514579),
    )]
    fn solve_part_one(input: Vec<isize>, expected: isize) {
        let day1 = Day1 {};
        assert_eq!(day1.solve_part_one(input).unwrap(), expected);
    }

    #[rstest(
        input,
        expected,
        case(vec![1721, 979, 366, 299, 675, 1456], 241861950),
    )]
    fn solve_part_two(input: Vec<isize>, expected: isize) {
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
        assert_eq!(day1.solve(input).unwrap(), expected);
    }

    #[test]
    fn parsing_error() {
        let day1 = Day1 {};
        assert_eq!(
            day1.solve("abc").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
