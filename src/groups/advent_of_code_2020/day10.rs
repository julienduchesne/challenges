use std::collections::{HashMap, HashSet};

use anyhow::Result;
use petgraph::{
    algo::{all_simple_paths, toposort},
    graphmap::DiGraphMap,
};

use super::super::challenge_config::ChallengeConfig;
pub struct Day10 {}

impl ChallengeConfig for Day10 {
    fn title(&self) -> &str {
        return "Day 10: Adapter Array";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["adapters".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        let mut numbers: Vec<usize> = variables["adapters"]
            .split_whitespace()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;
        numbers.push(0);
        let max = numbers.iter().max().unwrap() + 3;
        numbers.push(max);

        let mut pairs: HashSet<(usize, usize)> = HashSet::new();
        for first in numbers.clone() {
            for second in numbers.clone() {
                if first == second {
                    continue;
                }
                let min = first.min(second);
                let max = first.max(second);
                if max - min <= 3 {
                    pairs.insert((min, max));
                }
            }
        }

        let graph = DiGraphMap::<usize, ()>::from_edges(pairs);
        let node_map = toposort(&graph, None).unwrap();

        let mut jump1 = 0;
        let mut jump3 = 0;
        let mut previous = 0;
        let mut part_two = 1;
        for i in 1..node_map.len() {
            match node_map[i] - node_map[i - 1] {
                1 => jump1 += 1,
                3 => {
                    jump3 += 1;
                    part_two = part_two
                        * all_simple_paths::<Vec<usize>, _>(&graph, previous, node_map[i], 1, None)
                            .count()
                            .max(1);
                    previous = node_map[i];
                }
                _ => {}
            }
        }

        let part_one = jump1 * jump3;

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use rstest::rstest;

    use super::*;

    #[rstest(
        adapters,
        expected,
        case(
            "16
            10
            15
            5
            1
            11
            7
            19
            6
            12
            4",
            "Part 1: 35\nPart 2: 8"
        ),
        case(
            "28
            33
            18
            42
            31
            14
            46
            20
            48
            47
            24
            23
            49
            45
            19
            38
            39
            11
            1
            32
            25
            35
            8
            17
            7
            9
            4
            2
            34
            10
            3",
            "Part 1: 220\nPart 2: 19208"
        )
    )]
    fn solve(adapters: &str, expected: &str) {
        let day = Day10 {};
        assert_eq!(
            day.solve(hashmap! {"adapters" => adapters}).unwrap(),
            expected
        );
    }
}
