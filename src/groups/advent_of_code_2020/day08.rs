use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

use anyhow::Result;

use super::super::challenge_config::ChallengeConfig;
use super::super::challenge_config::ChallengeError;

struct Instruction {
    jump: isize,
    acc: isize,
}

impl Instruction {
    fn parse(line: &str) -> Result<Self> {
        let count = line.split_whitespace().nth(1).ok_or(ChallengeError::new(
            format!("Couldn't find second element for {}", line).as_str(),
        ))?;
        let multiplier = match count.chars().nth(0) {
            Some('+') => 1,
            Some('-') => -1,
            op => {
                return Err(ChallengeError::new(
                    format!("Invalid operator: {} for line: {}", op.unwrap_or(' '), line).as_str(),
                )
                .into())
            }
        };
        let count_int = count[1..].parse::<isize>()? * multiplier;

        let mut acc = 0;
        let mut jmp = 1;
        match line.split_whitespace().nth(0).unwrap_or("default") {
            "jmp" => jmp = count_int,
            "acc" => acc = count_int,
            "nop" => {}
            op => {
                return Err(ChallengeError::new(
                    format!("Invalid instruction: {} for line: {}", op, line).as_str(),
                )
                .into())
            }
        };

        return Ok(Self {
            jump: jmp,
            acc: acc,
        });
    }
}

pub struct Day8 {}

impl Day8 {
    fn solve_part_one(&self, variables: HashMap<&str, &str>) -> Result<isize> {
        let instructions: Vec<Instruction> = match variables["instructions"]
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(Instruction::parse)
            .collect()
        {
            Ok(result) => result,
            Err(e) => return Err(e),
        };

        let mut position: isize = 0;
        let mut acc: isize = 0;
        let mut run_positions: HashSet<isize> = HashSet::new();
        while !run_positions.contains(&position)
            && position >= 0
            && position < instructions.len().try_into().unwrap()
        {
            run_positions.insert(position);
            let position_u: usize = position.try_into().unwrap();
            let instruction: &Instruction = &instructions[position_u];
            position += instruction.jump;
            acc += instruction.acc;
        }
        return Ok(acc);
    }

    fn solve_part_two(&self, variables: HashMap<&str, &str>) -> Result<isize> {
        let lines: Vec<String> = variables["instructions"]
            .split("\n")
            .map(|x| String::from(x.trim()))
            .filter(|x| !x.is_empty())
            .collect();

        for (i, line) in lines.iter().enumerate() {
            let mut new_lines = lines.clone();
            if line.contains("nop") {
                new_lines[i] = line.replace("jmp", "nop");
            } else if line.contains("jmp") {
                new_lines[i] = line.replace("jmp", "nop");
            } else {
                continue;
            }

            let instructions: Vec<Instruction> =
                match new_lines.iter().map(|x| Instruction::parse(x)).collect() {
                    Ok(result) => result,
                    Err(e) => return Err(e),
                };
            let max_len: isize = instructions.len().try_into().unwrap();

            let mut position: isize = 0;
            let mut acc: isize = 0;
            let mut run_positions: HashSet<isize> = HashSet::new();
            while !run_positions.contains(&position) && position >= 0 && position < max_len {
                run_positions.insert(position);
                let position_u: usize = position.try_into().unwrap();
                let instruction: &Instruction = &instructions[position_u];
                position += instruction.jump;
                acc += instruction.acc;
            }
            if position == max_len {
                return Ok(acc);
            };
        }

        return Err(ChallengeError::new("Unable to find a solution for part 2").into());
    }
}

impl ChallengeConfig for Day8 {
    fn title(&self) -> &str {
        return "Day 8: Handheld Halting";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["instructions".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String, ChallengeError> {
        let part_one = match self.solve_part_one(variables.clone()) {
            Ok(result) => result,
            Err(e) => return Err(e.downcast().unwrap()),
        };
        let part_two = match self.solve_part_two(variables.clone()) {
            Ok(result) => result,
            Err(e) => return Err(e.downcast().unwrap()),
        };

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
            "nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6",
            "Part 1: 5\nPart 2: 8"
        )
    )]
    fn solve(instructions: &str, expected: &str) {
        let day = Day8 {};
        assert_eq!(
            day.solve(hashmap! {"instructions" => instructions})
                .unwrap(),
            expected
        );
    }
}
