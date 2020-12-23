use std::collections::HashMap;

use anyhow::Result;

use super::super::challenge_config::ChallengeConfig;
pub struct Day23 {}

impl Day23 {
    fn play(cups: Vec<usize>, iterations: usize, cup_count: Option<usize>) -> Vec<usize> {
        let mut cups = cups.clone();
        let max = if cup_count.is_none() {
            9
        } else {
            for i in 10..=cup_count.unwrap() {
                cups.push(i);
            }
            cup_count.unwrap()
        };

        // An array that points value -> next value
        let mut link_list = vec![0; max + 1];
        (0..max).for_each(|i| link_list[cups[i]] = cups[(i + 1) % max]);

        let mut cur_cup = cups[0];
        for _ in 0..iterations {
            // Remove elements from link list
            let p1 = link_list[cur_cup];
            let p2 = link_list[p1];
            let p3 = link_list[p2];
            link_list[cur_cup] = link_list[p3];

            // Get destination value
            let mut dest_cup = cur_cup;
            while [p1, p2, p3, cur_cup].contains(&dest_cup) || dest_cup < 1 || dest_cup > max {
                dest_cup = if dest_cup > 1 { dest_cup - 1 } else { max };
            }

            // Insert elements in new location
            let tmp = link_list[dest_cup];
            link_list[dest_cup] = p1;
            link_list[p1] = p2;
            link_list[p2] = p3;
            link_list[p3] = tmp;

            cur_cup = link_list[cur_cup];
        }

        // Build result vec
        let mut result = vec![1];
        let mut i = 1;
        while link_list[i] != 1 {
            i = link_list[i];
            result.push(i);
        }
        return result;
    }
}

impl ChallengeConfig for Day23 {
    fn title(&self) -> &str {
        return "Day 23: Crab Cups";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["Cups".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        let cups: Vec<usize> = variables["Cups"]
            .chars()
            .map(|c| c.to_string().parse::<usize>())
            .collect::<Result<_, _>>()?;

        let first_result = Self::play(cups.clone(), 100, None);
        let part_one = first_result[1..]
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join("");

        let second_result = Self::play(cups.clone(), 10000000, Some(1000000));
        let part_two = second_result[1] * second_result[2];

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use rstest::rstest;

    use super::*;

    #[rstest(
        cups,
        expected,
        case("389125467", "Part 1: 67384529\nPart 2: 149245887792")
    )]
    fn solve(cups: &str, expected: &str) {
        let day = Day23 {};
        assert_eq!(day.solve(hashmap! {"Cups" => cups }).unwrap(), expected);
    }
}
