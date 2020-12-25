use std::collections::HashMap;

use anyhow::Result;
use regex::Regex;

use crate::groups::challenge_config::{ChallengeConfig, ChallengeError};

pub struct Day14 {}

impl Day14 {}

impl ChallengeConfig for Day14 {
    fn title(&self) -> &str {
        return "Day 14: Docking Data";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["instructions".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        let instructions: Vec<&str> = variables["instructions"]
            .split("\n")
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .collect();

        let mut mask = "X".repeat(36);
        let mut part_one_values: HashMap<usize, usize> = HashMap::new();
        let mut part_two_values: HashMap<usize, usize> = HashMap::new();
        for instruction in instructions {
            lazy_static! {
                static ref MASK_REGEX: Regex = Regex::new(r"^mask = ([X01]{36})$").unwrap();
                static ref ASSIGN_REGEX: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
            }
            let mask_captures = MASK_REGEX.captures(instruction);
            let assign_captures = ASSIGN_REGEX.captures(instruction);
            if mask_captures.iter().count() > 0 {
                for caps in mask_captures {
                    mask = caps.get(1).unwrap().as_str().to_string();
                }
            } else if assign_captures.iter().count() > 0 {
                for caps in assign_captures {
                    let address = caps.get(1).unwrap().as_str().parse::<usize>()?;
                    let value = caps.get(2).unwrap().as_str().parse::<usize>()?;

                    let mut part_one_value = value.clone();
                    let mut part_two_addresses: Vec<usize> = vec![address.clone()];
                    for (i, c) in mask.chars().rev().enumerate() {
                        let mask_value = (2 as usize).pow(i as u32);
                        match c {
                            '1' => {
                                part_one_value = part_one_value | mask_value;
                                for (i, address) in part_two_addresses.clone().iter().enumerate() {
                                    part_two_addresses[i] = address | mask_value;
                                }
                            }
                            '0' => part_one_value = part_one_value & (68719476735 - mask_value),
                            'X' => {
                                let addresses_to_double = part_two_addresses.clone();
                                part_two_addresses.clear();
                                for address in addresses_to_double.iter() {
                                    part_two_addresses.push(address | mask_value);
                                    part_two_addresses.push(address & (68719476735 - mask_value));
                                }
                            }
                            _ => {}
                        }
                    }
                    part_one_values.insert(address, part_one_value);
                    for address in part_two_addresses {
                        part_two_values.insert(address, value);
                    }
                }
            } else {
                return Err(ChallengeError::new(
                    format!("Unable to find a mask or an assign in {}", instruction).as_str(),
                )
                .into());
            }
        }
        let part_one: usize = part_one_values.values().sum();
        let part_two: usize = part_two_values.values().sum();

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use rstest::rstest;

    use super::*;

    #[rstest(
        instructions,
        expected,
        case(
            "mask = 000000000000000000000000000001XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0",
            "Part 1: 165\nPart 2: 3232"
        ),
        case(
            "mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1",
            "Part 1: 51\nPart 2: 208"
        )
    )]
    fn solve(instructions: &str, expected: &str) {
        let day = Day14 {};
        assert_eq!(
            day.solve(hashmap! {"instructions" => instructions})
                .unwrap(),
            expected
        );
    }
}
