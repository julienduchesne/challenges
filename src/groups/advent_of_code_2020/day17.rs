use anyhow::Result;
use ndarray::{Array, Array2, Array3, Array4, Axis};

use crate::groups::challenge_config::ChallengeConfig;

pub struct Day17 {}

impl Day17 {
    fn adjacent_3d(array: Array3<usize>, pos: (usize, usize, usize)) -> usize {
        let mut total = 0;
        for x in (pos.0 - 1)..=(pos.0 + 1) {
            for y in (pos.1 - 1)..=(pos.1 + 1) {
                for z in (pos.2 - 1)..=(pos.2 + 1) {
                    if (x, y, z) != pos {
                        total += array.get((x, y, z)).unwrap();
                    }
                }
            }
        }
        return total;
    }

    fn adjacent_4d(array: Array4<usize>, pos: (usize, usize, usize, usize)) -> usize {
        let mut total = 0;
        for x in (pos.0 - 1)..=(pos.0 + 1) {
            for y in (pos.1 - 1)..=(pos.1 + 1) {
                for z in (pos.2 - 1)..=(pos.2 + 1) {
                    for w in (pos.3 - 1)..=(pos.3 + 1) {
                        if (x, y, z, w) != pos {
                            total += array.get((x, y, z, w)).unwrap();
                            if total == 4 {
                                // Optimization, if we're up to 4, no need to count anymore
                                return total;
                            }
                        }
                    }
                }
            }
        }
        return total;
    }
}

impl ChallengeConfig for Day17 {
    fn title(&self) -> &str {
        return "Day 17: Conway Cubes";
    }

    fn solve(&self, input: &str) -> Result<String> {
        let lines: Vec<&str> = input
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect();
        let iterations = 6;

        let n = lines[0].len().max(lines.len()) + (iterations + 1) * 2;
        let mut array_3d: Array3<usize> = Array3::zeros((n, n, n));
        let mut array_4d: Array4<usize> = Array4::zeros((n, n, n, n));
        let mut assigned_slice: Array2<usize> = Array2::zeros((n, n));
        for (i, mut row) in assigned_slice.axis_iter_mut(Axis(0)).enumerate() {
            if i <= iterations || i > iterations + lines.len() {
                continue;
            }
            let mut line: Vec<usize> = (0..=iterations).map(|_| 0).collect();
            for char in lines[i - iterations - 1].chars() {
                line.push(match char {
                    '#' => 1,
                    _ => 0,
                });
            }
            (0..=iterations).for_each(|_| line.push(0));
            row.assign(&Array::from(line));
        }

        for (i, mut row) in array_3d.axis_iter_mut(Axis(0)).enumerate() {
            if i == iterations + 1 {
                row.assign(&assigned_slice);
            }
        }
        for (i, mut row) in array_4d.axis_iter_mut(Axis(0)).enumerate() {
            if i == iterations + 1 {
                row.assign(&array_3d.clone());
            }
        }

        for iteration in 1..=iterations {
            // Limit the number of indices to check, the active states are only as far as the number of iterations
            let min_index = iterations - iteration;
            let max_index = n - (iterations - iteration) - 1;
            let copied_array_3d = array_3d.clone();
            for ((x, y, z), item) in array_3d.indexed_iter_mut() {
                if x <= min_index
                    || y <= min_index
                    || z <= min_index
                    || x >= max_index
                    || y >= max_index
                    || z >= max_index
                {
                    continue;
                }
                let adjacent = Self::adjacent_3d(copied_array_3d.clone(), (x, y, z));
                if *item == 1 && adjacent != 2 && adjacent != 3 {
                    *item = 0;
                } else if *item == 0 && adjacent == 3 {
                    *item = 1;
                }
            }
            let copied_array_4d = array_4d.clone();
            for ((x, y, z, w), item) in array_4d.indexed_iter_mut() {
                if x <= min_index
                    || y <= min_index
                    || z <= min_index
                    || w <= min_index
                    || x >= max_index
                    || y >= max_index
                    || z >= max_index
                    || w >= max_index
                {
                    continue;
                }
                let adjacent = Self::adjacent_4d(copied_array_4d.clone(), (x, y, z, w));
                if *item == 1 && adjacent != 2 && adjacent != 3 {
                    *item = 0;
                } else if *item == 0 && adjacent == 3 {
                    *item = 1;
                }
            }
        }

        let part_one: usize = array_3d.iter().filter(|x| **x == 1).count();
        let part_two: usize = array_4d.iter().filter(|x| **x == 1).count();

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        initial_layer,
        expected,
        case(
            ".#.
            ..#
            ###",
            "Part 1: 112\nPart 2: 848"
        )
    )]
    fn solve(initial_layer: &str, expected: &str) {
        let day = Day17 {};
        assert_eq!(day.solve(initial_layer).unwrap(), expected);
    }
}
