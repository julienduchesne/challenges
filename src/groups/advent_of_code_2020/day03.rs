use anyhow::Result;
use ndarray::{Array, Array2, Axis};

use crate::groups::challenge_config::ChallengeConfig;

pub struct Day3 {}

impl Day3 {
    fn solve_path(&self, array: Array2<usize>, path: (i32, i32)) -> usize {
        let mut trees_hit = 0;
        let mut current_row = 0;
        let mut current_col = 0;
        let row_count = array.len_of(Axis(0)) as i32;
        let col_count = array.len_of(Axis(1)) as i32;
        while current_row < row_count - 1 {
            current_row -= path.0;
            current_col = (current_col + path.1) % col_count; // Possible to overflow
            if array.get((current_row as usize, current_col as usize)) == Some(&1) {
                trees_hit += 1;
            }
        }

        return trees_hit;
    }
}

impl ChallengeConfig for Day3 {
    fn title(&self) -> &str {
        return "Day 3: Toboggan Trajectory";
    }

    fn solve(&self, input: &str) -> Result<String> {
        let lines: Vec<&str> = input
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
                    .map(|x| (x == '#') as usize)
                    .collect::<Vec<usize>>(),
            ));
        }

        let part_one_vecs = vec![(-1, 3)];
        let part_two_vecs = vec![(-1, 1), (-1, 3), (-1, 5), (-1, 7), (-2, 1)];

        let mut result = "".to_string();
        for (index, vecs) in vec![part_one_vecs, part_two_vecs].iter().enumerate() {
            let mut line_results = vec![];
            for path_vec in vecs.iter() {
                let line_result = self.solve_path(array.clone(), path_vec.clone());
                line_results.push(line_result);
            }
            result.push_str(
                format!(
                    "Part {}: {} = {}\n",
                    index + 1,
                    line_results
                        .iter()
                        .map(usize::to_string)
                        .collect::<Vec<String>>()
                        .join(" * "),
                    line_results.iter().product::<usize>()
                )
                .as_str(),
            );
        }

        return Ok(String::from(result.trim_end()));
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        map,
        expected,
        case(
            "..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#",
            "Part 1: 7 = 7\nPart 2: 2 * 7 * 3 * 4 * 2 = 336"
        )
    )]
    fn solve(map: &str, expected: &str) {
        let day = Day3 {};
        assert_eq!(day.solve(map).unwrap(), expected);
    }
}
