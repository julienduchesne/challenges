use anyhow::Result;

use crate::groups::challenge_config::ChallengeConfig;

pub struct Day12 {}

impl Day12 {
    fn solve_part_one(&self, moves: Vec<(char, isize)>) -> isize {
        let possible_directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];
        let mut direction_index = 1000000; // To avoid doing arithmetic (other than mod)
        let mut position: (isize, isize) = (0, 0);
        for mov in moves {
            match mov.0 {
                'N' => position.0 += mov.1,
                'E' => position.1 += mov.1,
                'W' => position.1 -= mov.1,
                'S' => position.0 -= mov.1,
                'L' => direction_index -= (mov.1 / 90) as usize,
                'R' => direction_index += (mov.1 / 90) as usize,
                'F' => {
                    let direction = possible_directions[direction_index % 4];
                    position.0 += direction.0 * mov.1;
                    position.1 += direction.1 * mov.1;
                }
                _ => {}
            }
        }
        return position.0.abs() + position.1.abs();
    }

    fn rotate_coords(coords: (isize, isize), clockwise: bool, times: isize) -> (isize, isize) {
        let mut new_coords = coords.clone();
        for _ in 0..times {
            if clockwise {
                new_coords = (-new_coords.1, new_coords.0);
            } else {
                new_coords = (new_coords.1, -new_coords.0);
            }
        }
        return new_coords;
    }

    fn solve_part_two(&self, moves: Vec<(char, isize)>) -> isize {
        let mut waypoint: (isize, isize) = (1, 10);
        let mut position: (isize, isize) = (0, 0);
        for mov in moves {
            match mov.0 {
                'N' => waypoint.0 += mov.1,
                'E' => waypoint.1 += mov.1,
                'W' => waypoint.1 -= mov.1,
                'S' => waypoint.0 -= mov.1,
                'L' => waypoint = Self::rotate_coords(waypoint, false, mov.1 / 90),
                'R' => waypoint = Self::rotate_coords(waypoint, true, mov.1 / 90),
                'F' => {
                    position.0 += waypoint.0 * mov.1;
                    position.1 += waypoint.1 * mov.1;
                }
                _ => {}
            }
        }
        return position.0.abs() + position.1.abs();
    }
}

impl ChallengeConfig for Day12 {
    fn title(&self) -> &str {
        return "Day 12: Rain Risk";
    }

    fn solve(&self, input: &str) -> Result<String> {
        let moves: Vec<(char, isize)> = input
            .split_whitespace()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| {
                (
                    x.chars().nth(0).unwrap(),
                    x[1..].parse::<isize>().unwrap_or(0),
                )
            })
            .collect();

        let part_one = self.solve_part_one(moves.clone());
        let part_two = self.solve_part_two(moves.clone());

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        moves,
        expected,
        case(
            "F10
            N3
            F7
            R90
            F11",
            "Part 1: 25\nPart 2: 286"
        )
    )]
    fn solve(moves: &str, expected: &str) {
        let day = Day12 {};
        assert_eq!(day.solve(moves).unwrap(), expected);
    }
}
