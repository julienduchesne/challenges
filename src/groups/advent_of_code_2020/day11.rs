use std::collections::HashMap;

use anyhow::Result;
use ndarray::{Array, Array2, Axis};

use crate::groups::challenge_config::ChallengeConfig;

const FLOOR: usize = 0;
const FREE: usize = 1;
const OCCUPIED: usize = 2;

pub struct Day11 {}

impl Day11 {
    fn adjacent(array: &Array2<usize>, row: usize, col: usize) -> usize {
        let mut adj_count = 0;
        let row_count = array.len_of(Axis(0));
        let col_count = array.len_of(Axis(1));

        let mut lower_col_bound = 0;
        if col != 0 {
            lower_col_bound = col - 1;
        }

        let mut lower_row_bound = 0;
        if row != 0 {
            lower_row_bound = row - 1;
        }

        for i in lower_row_bound..row_count.min(row + 2) {
            for j in lower_col_bound..col_count.min(col + 2) {
                if !(i == row && j == col) && array[(i, j)] == OCCUPIED {
                    adj_count += 1;
                }
            }
        }
        return adj_count;
    }

    fn seen(array: &Array2<usize>, row: isize, col: isize) -> usize {
        let mut seen_count = 0;
        let row_count = array.len_of(Axis(0)) as isize;
        let col_count = array.len_of(Axis(1)) as isize;

        for i_var in -1..=1 {
            for j_var in -1..=1 {
                if i_var == 0 && j_var == 0 {
                    continue;
                }
                let mut i = row + i_var;
                let mut j = col + j_var;
                while i >= 0 && i < row_count && j >= 0 && j < col_count {
                    match array[(i as usize, j as usize)] {
                        OCCUPIED => {
                            seen_count += 1;
                            break;
                        }
                        FREE => break,
                        _ => {}
                    }
                    i += i_var;
                    j += j_var;
                }
            }
        }
        return seen_count;
    }
}

impl ChallengeConfig for Day11 {
    fn title(&self) -> &str {
        return "Day 11: Seating System";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["Seats".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        let lines: Vec<&str> = variables["Seats"]
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect();
        let ncols = lines[0].len();
        let nrows = lines.len();
        let mut array: Array2<usize> = Array2::zeros((nrows, ncols));
        for (i, mut row) in array.axis_iter_mut(Axis(0)).enumerate() {
            // Perform calculations and assign to `row`; this is a trivial example:
            row.assign(&Array::from(
                lines[i]
                    .chars()
                    .map(|x| match x {
                        'L' => FREE,
                        _ => FLOOR,
                    })
                    .collect::<Vec<usize>>(),
            ));
        }

        // Part one
        let mut changed = true;
        let mut part_one_array = array.clone();
        while changed {
            let cloned_array = part_one_array.clone();
            changed = false;
            for ((row, col), value) in part_one_array.indexed_iter_mut() {
                if value == &FLOOR {
                    continue;
                }
                let adjacent_count = Day11::adjacent(&cloned_array, row, col);
                if *value == FREE && adjacent_count == 0 {
                    *value = OCCUPIED;
                    changed = true;
                } else if *value == OCCUPIED && adjacent_count >= 4 {
                    *value = FREE;
                    changed = true;
                }
            }
        }
        let part_one: usize = part_one_array.iter().filter(|x| **x == OCCUPIED).count();

        // Part two
        changed = true;
        let mut part_two_array = array.clone();
        while changed {
            let cloned_array = part_two_array.clone();
            changed = false;
            for ((row, col), value) in part_two_array.indexed_iter_mut() {
                if value == &FLOOR {
                    continue;
                }
                let seen_count = Day11::seen(&cloned_array, row as isize, col as isize);
                if *value == FREE && seen_count == 0 {
                    *value = OCCUPIED;
                    changed = true;
                } else if *value == OCCUPIED && seen_count >= 5 {
                    *value = FREE;
                    changed = true;
                }
            }
        }
        let part_two: usize = part_two_array.iter().filter(|x| **x == OCCUPIED).count();

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use rstest::rstest;

    use super::*;

    #[rstest(
        seats,
        expected,
        case(
            "L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL",
            "Part 1: 37\nPart 2: 26"
        )
    )]
    fn solve(seats: &str, expected: &str) {
        let day = Day11 {};
        assert_eq!(day.solve(hashmap! {"Seats" => seats}).unwrap(), expected);
    }
}
