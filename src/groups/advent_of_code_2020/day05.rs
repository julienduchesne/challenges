use std::collections::HashMap;

use super::super::challenge_config::ChallengeConfig;
use super::super::challenge_config::ChallengeError;
use super::super::challenge_config::VariableType;
use itertools::Itertools;
use maplit::hashmap;

fn slice_max_down(min: usize, max: usize) -> usize {
    return max - (max + 1 - min) / 2;
}

fn slice_min_up(min: usize, max: usize) -> usize {
    return min + (max + 1 - min) / 2;
}

struct BoardingPass {
    letters: Vec<char>,
}

impl BoardingPass {
    fn get_row(&self) -> Result<usize, ChallengeError> {
        let mut min = 0;
        let mut max = 127;
        let row_chars = &self.letters[0..7];
        for c in row_chars {
            match c {
                'F' => max = slice_max_down(min, max),
                'B' => min = slice_min_up(min, max),
                _ => (),
            }
        }
        if min != max {
            return Err(ChallengeError::new("Unable to determine row number"));
        }
        return Ok(min);
    }
    fn get_col(&self) -> Result<usize, ChallengeError> {
        let mut min = 0;
        let mut max = 7;
        let col_chars = &self.letters[7..10];
        for c in col_chars {
            match c {
                'L' => max = slice_max_down(min, max),
                'R' => min = slice_min_up(min, max),
                _ => (),
            }
        }
        if min != max {
            return Err(ChallengeError::new("Unable to determine col number"));
        }
        return Ok(min);
    }
    fn get_seat_id(&self) -> Result<usize, ChallengeError> {
        let col = match self.get_col() {
            Ok(col) => col,
            Err(e) => return Err(e),
        };

        let row = match self.get_row() {
            Ok(row) => row,
            Err(e) => return Err(e),
        };

        Ok((row * 8) + col)
    }
}

pub struct Day5 {}

impl Day5 {}

impl ChallengeConfig for Day5 {
    fn title(&self) -> &str {
        return "Day 5: Binary Boarding";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> HashMap<String, crate::groups::challenge_config::VariableType> {
        return hashmap! {"passes".to_owned() => VariableType::MultiLineString};
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String, ChallengeError> {
        let passes: Vec<BoardingPass> = variables["passes"]
            .replace("  ", "")
            .split_whitespace()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| BoardingPass {
                letters: x.chars().collect(),
            })
            .collect();

        let part_one_results: Result<Vec<usize>, _> =
            passes.iter().map(|p| p.get_seat_id()).collect();
        let taken_seats: Vec<usize> = match part_one_results.clone() {
            Ok(results) => results.iter().map(|x| *x).sorted().collect(),
            Err(e) => return Err(e),
        };

        let part_one = taken_seats.iter().max().unwrap_or(&(0 as usize));

        let mut part_two: usize = 0;
        let mut last_taken: usize = 0;
        for seat in taken_seats.clone() {
            if last_taken + 2 == seat {
                part_two = seat - 1;
                break;
            }
            last_taken = seat;
        }

        if part_two == 0 {
            return Err(ChallengeError::new("Unable to find an answer for part two"));
        }

        return Ok(format!("Part 1: {}\nPart 2: {:?}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        letters,
        expected_row,
        expected_col,
        case("BFFFBBFRRR", 70, 7),
        case("FFFBBBFRRR", 14, 7),
        case("BBFFBBFRLL", 102, 4)
    )]
    fn get_row_and_col(letters: &str, expected_row: usize, expected_col: usize) {
        let pass = BoardingPass {
            letters: letters.chars().collect::<Vec<char>>(),
        };
        assert_eq!(pass.get_col().unwrap(), expected_col);
        assert_eq!(pass.get_row().unwrap(), expected_row);
    }

    #[rstest(
        passes,
        expected,
        case(
            "BFFFBBFRRR
            FFFBBBFRRR
            BBFFBBFRLL
            BBFFBBFLRL",
            "Part 1: 820\nPart 2: 819"
        )
    )]
    fn solve(passes: &str, expected: &str) {
        let day = Day5 {};
        assert_eq!(day.solve(hashmap! {"passes" => passes}).unwrap(), expected);
    }
}
