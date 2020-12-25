use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

use crate::groups::challenge_config::ChallengeConfig;

pub struct Day9 {}

impl ChallengeConfig for Day9 {
    fn title(&self) -> &str {
        return "Day 9: Encoding Error";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["numbers".to_owned(), "preamble length".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        let preamble_length: usize = variables["preamble length"].parse::<usize>()?;
        let numbers: Vec<usize> = variables["numbers"]
            .split_whitespace()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>())
            .collect::<Result<_, _>>()?;

        // Part 1: Find the number that doesn't have two other numbers before it that sum to it
        let mut part_one = 0;
        for i in preamble_length..numbers.len() - 1 {
            let mut found = false;
            let to_find = numbers[i];
            for x in (i - preamble_length)..i {
                if found {
                    break;
                }
                for y in (i - preamble_length)..i {
                    if found {
                        break;
                    }
                    let first = numbers[x];
                    let second = numbers[y];
                    if first == second {
                        continue;
                    }
                    if first + second == to_find {
                        found = true;
                    }
                }
            }
            if !found {
                part_one = to_find;
                break;
            }
        }

        // Part 2: Find contiguous sequence that sums to the invalid number
        let mut part_two = 0;
        let mut active_sequences: Vec<Vec<usize>> = vec![];
        for i in 0..numbers.len() - 1 {
            let new_number = numbers[i];
            let mut indices_to_remove: Vec<usize> = vec![];
            for (s_index, sequence) in active_sequences.clone().iter().enumerate() {
                let mut new_sequence = sequence.clone();
                let sum: usize = new_sequence.iter().sum();
                if sum == part_one {
                    // We're done!
                    part_two =
                        new_sequence.iter().min().unwrap() + new_sequence.iter().max().unwrap();
                    break;
                } else if sum < part_one {
                    // Replace the sequence, we're keeping it
                    new_sequence.push(new_number.clone());
                    active_sequences[s_index] = new_sequence.clone();
                } else if sum > part_one {
                    indices_to_remove.push(s_index);
                }
            }
            if part_two != 0 {
                break;
            }

            for i in indices_to_remove.iter().sorted().rev() {
                active_sequences.remove(*i);
            }

            active_sequences.push(vec![new_number]);
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
        numbers,
        preamble_length,
        expected,
        case(
            "35
            20
            15
            25
            47
            40
            62
            55
            65
            95
            102
            117
            150
            182
            127
            219
            299
            277
            309
            576",
            "5",
            "Part 1: 127\nPart 2: 62"
        )
    )]
    fn solve(numbers: &str, preamble_length: &str, expected: &str) {
        let day = Day9 {};
        assert_eq!(
            day.solve(hashmap! {"numbers" => numbers, "preamble length" => preamble_length})
                .unwrap(),
            expected
        );
    }
}
