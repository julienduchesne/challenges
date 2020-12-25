use std::collections::HashMap;

use anyhow::Result;

use crate::groups::challenge_config::ChallengeConfig;

pub struct Day22 {}

impl Day22 {
    fn parse_deck(input: &str) -> Result<Vec<usize>> {
        return Ok(input
            .split_whitespace()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::parse::<usize>)
            .collect::<Result<_, _>>()?);
    }

    fn count_winner(deck: Vec<usize>) -> usize {
        let mut result: usize = 0;
        for (i, item) in deck.iter().rev().enumerate() {
            result += (i + 1) * item
        }
        return result;
    }

    fn solve_regular(deck1: Vec<usize>, deck2: Vec<usize>) -> usize {
        let mut deck1 = deck1.clone();
        let mut deck2 = deck2.clone();
        while deck1.len() > 0 && deck2.len() > 0 {
            let first = deck1.remove(0);
            let second = deck2.remove(0);
            if first > second {
                deck1.append(&mut vec![first, second]);
            } else {
                deck2.append(&mut vec![second, first]);
            }
        }

        let mut winning_deck = deck1;
        if deck2.len() > 0 {
            winning_deck = deck2;
        }
        return Self::count_winner(winning_deck);
    }

    fn solve_recursive(deck1: Vec<usize>, deck2: Vec<usize>) -> usize {
        return Self::count_winner(Self::play_recursive(deck1, deck2).1);
    }

    fn play_recursive(deck1: Vec<usize>, deck2: Vec<usize>) -> (bool, Vec<usize>) {
        let mut deck1 = deck1.clone();
        let mut deck2 = deck2.clone();
        let mut cont = true;
        let mut previous_rounds: Vec<[Vec<usize>; 2]> = vec![];
        while cont {
            if previous_rounds
                .iter()
                .any(|previous| deck1 == previous[0] && deck2 == previous[1])
            {
                return (true, deck1);
            }
            previous_rounds.push([deck1.clone(), deck2.clone()]);

            let first = deck1.remove(0);
            let second = deck2.remove(0);
            let player_one_wins: bool;
            if deck1.len() >= first && deck2.len() >= second {
                player_one_wins = Self::play_recursive(
                    deck1.clone()[..first].to_vec(),
                    deck2.clone()[..second].to_vec(),
                )
                .0;
            } else {
                player_one_wins = first > second;
            }
            if player_one_wins {
                deck1.append(&mut vec![first, second]);
            } else {
                deck2.append(&mut vec![second, first]);
            }
            cont = deck1.len() > 0 && deck2.len() > 0
        }
        if deck1.len() > 0 {
            return (true, deck1);
        } else {
            return (false, deck2);
        }
    }
}

impl ChallengeConfig for Day22 {
    fn title(&self) -> &str {
        return "Day 22: Crab Combat";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["Player 1's deck".to_owned(), "Player 2's deck".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        let deck1 = Self::parse_deck(variables["Player 1's deck"])?;
        let deck2 = Self::parse_deck(variables["Player 2's deck"])?;

        let part_one = Self::solve_regular(deck1.clone(), deck2.clone());
        let part_two = Self::solve_recursive(deck1.clone(), deck2.clone());

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use rstest::rstest;

    use super::*;

    #[rstest(
        one,
        two,
        expected,
        case(
            "9
            2
            6
            3
            1",
            "5
            8
            4
            7
            10",
            "Part 1: 306\nPart 2: 291"
        )
    )]
    fn solve(one: &str, two: &str, expected: &str) {
        let day = Day22 {};
        assert_eq!(
            day.solve(hashmap! {"Player 1's deck" => one, "Player 2's deck" => two})
                .unwrap(),
            expected
        );
    }
}
