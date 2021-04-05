use anyhow::Result;

use crate::groups::challenge_config::ChallengeConfig;

pub struct Day15 {}

impl Day15 {}

impl ChallengeConfig for Day15 {
    fn title(&self) -> &str {
        return "Day 15: Rambunctious Recitation";
    }

    fn solve(&self, input: &str) -> Result<String> {
        let numbers: Vec<usize> = input
            .split(",")
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .map(str::parse::<usize>)
            .collect::<Result<_, _>>()?;

        let mut results = vec![];
        for (part, nth_number) in [("Part 1", 2020 as usize), ("Part 2", 30000000)].iter() {
            let mut last_spoken: Vec<Option<usize>> = std::iter::repeat(None)
                .take(*nth_number)
                .collect::<Vec<_>>();
            for i in 0..numbers.len() {
                last_spoken[numbers[i]] = Some(i);
            }

            let mut last: usize = *numbers.last().unwrap();
            for i in numbers.len()..*nth_number {
                let new_number = match last_spoken[last] {
                    Some(index) => i - index - 1,
                    _ => 0,
                };
                last_spoken[last] = Some(i - 1);
                last = new_number;
            }

            results.push(format!("{}: {}", part, last).to_owned());
        }

        return Ok(results.join("\n"));
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        input,
        expected,
        case("1,3,2", "Part 1: 1\nPart 2: 2578"),
        case("2,1,3", "Part 1: 10\nPart 2: 3544142"),
        case("1,2,3", "Part 1: 27\nPart 2: 261214"),
        case("2,3,1", "Part 1: 78\nPart 2: 6895259"),
        case("3,2,1", "Part 1: 438\nPart 2: 18"),
        case("3,1,2", "Part 1: 1836\nPart 2: 362")
    )]
    fn solve(input: &str, expected: &str) {
        let day = Day15 {};
        assert_eq!(day.solve(input).unwrap(), expected);
    }
}
