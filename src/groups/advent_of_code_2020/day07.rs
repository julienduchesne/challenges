use std::collections::{HashMap, HashSet};

use maplit::hashmap;
use regex::Regex;

use super::super::challenge_config::ChallengeConfig;
use super::super::challenge_config::ChallengeError;
use super::super::challenge_config::VariableType;

pub struct Day7 {}

impl Day7 {
    fn parse_rules(
        &self,
        input: &str,
    ) -> Result<HashMap<String, HashMap<String, usize>>, ChallengeError> {
        let mut rules = HashMap::new();
        for item in input
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
        {
            lazy_static! {
                static ref BAG_RE: Regex = Regex::new(r"^([a-z\s]+)s\s").unwrap();
                static ref CONTAINS_RE: Regex = Regex::new(r"(\d)\s([a-z\s]+bag)").unwrap();
            }
            let mut bag_name = "";
            for caps in BAG_RE.captures_iter(item) {
                if bag_name != "" {
                    return Err(ChallengeError::new(
                        format!("Found two bag names in {}", item).as_str(),
                    ));
                }
                bag_name = caps.get(1).unwrap().as_str();
            }
            if bag_name == "" {
                return Err(ChallengeError::new(
                    format!("Can't find bag name in {}", item).as_str(),
                ));
            }

            let mut bag_rules: HashMap<String, usize> = HashMap::new();
            for caps in CONTAINS_RE.captures_iter(item) {
                let count: usize = match caps.get(1).unwrap().as_str().parse::<usize>() {
                    Ok(count) => count,
                    Err(_) => {
                        return Err(ChallengeError::new(
                            format!("Couldn't parse count in {}", item).as_str(),
                        ))
                    }
                };
                bag_rules.insert(caps.get(2).unwrap().as_str().to_owned(), count);
            }

            rules.insert(bag_name.to_owned(), bag_rules);
        }

        return Ok(rules);
    }

    fn solve_part_one(&self, rules: HashMap<String, HashMap<String, usize>>, bag: &str) -> usize {
        // Part 1: Calculate all bags that can eventually contain a gold bag
        // This code is not efficient, the maps should be reversed beforehand for performance
        let mut all_colors: HashSet<&str> = HashSet::new();
        let mut new_items: HashSet<&str> = [bag].iter().cloned().collect();
        while new_items.len() > 0 {
            let mut container_bags: HashSet<&str> = HashSet::new();
            for new_item in new_items {
                container_bags.extend(
                    rules
                        .iter()
                        .filter(|x| x.1.contains_key(&new_item.to_owned()))
                        .map(|x| x.0.as_str())
                        .collect::<HashSet<_>>(),
                )
            }
            all_colors.extend(container_bags.clone());
            new_items = container_bags;
        }

        return all_colors.len();
    }

    fn solve_part_two(&self, rules: HashMap<String, HashMap<String, usize>>, bag: &str) -> usize {
        // Part 2: Calculate the total number of bags a color will contain, this one will be more efficient
        let mut total_count = 0;
        let mut new_items: Vec<String> = [bag.to_owned()].iter().cloned().collect();
        while new_items.len() > 0 {
            let mut contained_bags: Vec<String> = vec![];
            for new_item in new_items {
                for (contained_bag, count) in rules[&new_item].clone() {
                    contained_bags.extend((0..count).map(|_| contained_bag.clone()));
                    total_count += count;
                }
            }
            new_items = contained_bags;
        }
        return total_count;
    }
}

impl ChallengeConfig for Day7 {
    fn title(&self) -> &str {
        return "Day 7: Handy Haversacks";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> HashMap<String, VariableType> {
        return hashmap! {"rules".to_owned() => VariableType::MultiLineString};
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String, ChallengeError> {
        let rules = match self.parse_rules(variables["rules"]) {
            Ok(rules) => rules,
            Err(e) => return Err(e),
        };
        let part_one = self.solve_part_one(rules.clone(), "shiny gold bag");
        let part_two = self.solve_part_two(rules.clone(), "shiny gold bag");

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        rules,
        expected,
        case(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.",
            "Part 1: 4\nPart 2: 32"
        ),
        case(
            "shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.",
            "Part 1: 0\nPart 2: 126"
        )
    )]
    fn solve(rules: &str, expected: &str) {
        let day = Day7 {};
        assert_eq!(day.solve(hashmap! {"rules" => rules}).unwrap(), expected);
    }
}
