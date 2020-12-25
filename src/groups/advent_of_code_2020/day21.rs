use std::collections::{HashMap, HashSet};

use anyhow::Result;
use itertools::Itertools;

use crate::groups::challenge_config::ChallengeConfig;
pub struct Day21 {}

impl ChallengeConfig for Day21 {
    fn title(&self) -> &str {
        return "Day 21: Allergen Assessment";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["Foods".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        let foods: Vec<&str> = variables["Foods"]
            .split("\n")
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .collect();

        let mut possible_allergens: HashMap<&str, HashSet<&str>> = HashMap::new();
        let mut all_ingredients: Vec<&str> = vec![];
        for food in foods {
            let ingredients: HashSet<&str> = food
                .split("(")
                .nth(0)
                .unwrap()
                .split_whitespace()
                .map(str::trim)
                .filter(|x| !x.is_empty())
                .collect();
            all_ingredients.append(&mut ingredients.clone().iter().map(|x| *x).collect());
            let allergens: Vec<&str> = food
                .strip_suffix(")")
                .unwrap()
                .split("contains")
                .nth(1)
                .unwrap()
                .split(",")
                .map(str::trim)
                .filter(|x| !x.is_empty())
                .collect();

            for allergen in allergens {
                if !possible_allergens.contains_key(allergen) {
                    possible_allergens.insert(
                        allergen,
                        ingredients.iter().map(|x| *x).collect::<HashSet<&str>>(),
                    );
                } else {
                    let possible_ingredients = possible_allergens.get(allergen).unwrap().clone();
                    possible_allergens.insert(
                        allergen,
                        possible_ingredients
                            .intersection(&ingredients)
                            .map(|x| *x)
                            .collect(),
                    );
                }
            }
        }
        let mut changed = true;
        while changed {
            changed = false;
            for (allergen, foods) in possible_allergens.clone() {
                if foods.len() == 1 {
                    let food = *foods.iter().next().unwrap();
                    for (other_allergen, foods) in possible_allergens.iter_mut() {
                        if *other_allergen != allergen && foods.contains(food) {
                            changed = true;
                            foods.remove(food);
                        }
                    }
                }
            }
        }

        let found_allergens: HashSet<&str> = possible_allergens
            .iter()
            .filter(|(_, v)| v.len() == 1)
            .map(|(_, v)| *v.iter().next().unwrap())
            .collect();
        all_ingredients.retain(|x| !found_allergens.contains(x));
        let part_one: usize = all_ingredients.len();

        let part_two: String = possible_allergens
            .iter()
            .sorted_by_key(|(k, _)| **k)
            .map(|(_, v)| *v.iter().next().unwrap())
            .join(",");

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use rstest::rstest;

    use super::*;

    #[rstest(
        foods,
        expected,
        case(
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
            trh fvjkl sbzzf mxmxvkd (contains dairy)
            sqjhc fvjkl (contains soy)
            sqjhc mxmxvkd sbzzf (contains fish)",
            "Part 1: 5\nPart 2: mxmxvkd,sqjhc,fvjkl"
        )
    )]
    fn solve(foods: &str, expected: &str) {
        let day = Day21 {};
        assert_eq!(day.solve(hashmap! {"Foods" => foods}).unwrap(), expected);
    }
}
