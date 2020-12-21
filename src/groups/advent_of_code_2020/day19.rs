use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use anyhow::Result;
use lru::LruCache;

use super::super::challenge_config::ChallengeConfig;
pub struct Day19 {}
struct Rules {
    rules: HashMap<usize, String>,
    resolved_rules: LruCache<usize, HashSet<String>>,
}

impl Rules {
    fn new(rules: &str) -> Result<Self> {
        return Ok(Rules {
            rules: rules
                .split("\n")
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(Rules::rule_to_tuple)
                .collect::<Result<_, _>>()?,
            resolved_rules: LruCache::new(rules.len()),
        });
    }

    fn rule_to_tuple(rule: &str) -> Result<(usize, String)> {
        let id: usize = rule.split(":").nth(0).unwrap().parse::<usize>()?;
        let rule = rule.split(":").nth(1).unwrap().trim();
        return Ok((id, rule.to_owned()));
    }

    fn resolve_id(&mut self, id: usize) -> Result<HashSet<String>> {
        let mut resolved = self.resolved_rules.get(&id);
        if resolved.is_none() {
            let rule = self.rules.get(&id).unwrap().clone();
            let calculated = self.resolve(&rule)?;
            self.resolved_rules.put(id, calculated);
        }

        resolved = self.resolved_rules.get(&id);
        return Ok(HashSet::from_iter(
            resolved.unwrap().iter().map(|s| s.as_str().to_owned()),
        ));
    }

    fn resolve(&mut self, rule: &str) -> Result<HashSet<String>> {
        if rule.contains("\"") {
            let mut values: HashSet<String> = HashSet::new();
            values.insert(rule.split("\"").nth(1).unwrap().to_owned());
            return Ok(values);
        }
        if rule.contains("|") {
            let mut values: HashSet<String> = HashSet::new();
            for part in rule.split("|") {
                values.extend(self.resolve(part)?);
            }
            return Ok(values);
        }
        let ids: Vec<usize> = rule
            .split_whitespace()
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .map(str::parse::<usize>)
            .collect::<Result<_, _>>()?;
        let mut values: HashSet<String> = HashSet::new();
        for id in ids {
            if values.is_empty() {
                values = self.resolve_id(id)?;
                continue;
            }
            let mut merged_values: HashSet<String> = HashSet::new();
            for value in values {
                for new_value in self.resolve_id(id)? {
                    merged_values.insert(value.clone() + new_value.as_str());
                }
            }
            values = merged_values;
        }
        return Ok(values);
    }

    // With the replacements, the messages will always be in this format: 42{2,inf} 31+
    fn matches_forty_two_and_thirty_one(&mut self, message: &str) -> Result<bool> {
        let forty_two = self.resolve_id(42)?;

        let mut message = message.clone();
        let thirty_one = self.resolve_id(31)?;
        let mut thirty_one_count = 0;
        loop {
            let suffix = thirty_one.iter().find(|r| message.ends_with(r.as_str()));
            if suffix.is_none() {
                break;
            }
            message = message.strip_suffix(suffix.unwrap()).unwrap();
            thirty_one_count += 1;
        }

        let mut forty_two_count = 0;
        loop {
            let prefix = forty_two.iter().find(|r| message.starts_with(r.as_str()));
            if prefix.is_none() {
                break;
            }
            message = message.strip_prefix(prefix.unwrap()).unwrap();
            forty_two_count += 1;
        }

        return Ok(message.is_empty()
            && forty_two_count >= 2
            && thirty_one_count >= 1
            && forty_two_count > thirty_one_count);
    }
}

impl ChallengeConfig for Day19 {
    fn title(&self) -> &str {
        return "Day 19: Monster Messages";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["Rules".to_owned(), "Messages".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        let mut rules = Rules::new(variables["Rules"])?;
        let messages: Vec<&str> = variables["Messages"]
            .split_whitespace()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect();
        let valid = rules.resolve_id(0)?;

        let part_one: usize = messages.iter().filter(|m| valid.contains(**m)).count();
        let part_two_matches: Vec<&&str> = messages
            .iter()
            .filter(|m| rules.matches_forty_two_and_thirty_one(m).unwrap_or(false))
            .collect();
        let part_two: usize = part_two_matches.len();

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use rstest::rstest;

    use super::*;

    #[rstest(
        rules,
        messages,
        expected,
        case(
            "42: 9 14 | 10 1
            9: 14 27 | 1 26
            10: 23 14 | 28 1
            1: \"a\"
            11: 42 31
            5: 1 14 | 15 1
            19: 14 1 | 14 14
            12: 24 14 | 19 1
            16: 15 1 | 14 14
            31: 14 17 | 1 13
            6: 14 14 | 1 14
            2: 1 24 | 14 4
            0: 8 11
            13: 14 3 | 1 12
            15: 1 | 14
            17: 14 2 | 1 7
            23: 25 1 | 22 14
            28: 16 1
            4: 1 1
            20: 14 14 | 1 15
            3: 5 14 | 16 1
            27: 1 6 | 14 18
            14: \"b\"
            21: 14 1 | 1 14
            25: 1 1 | 1 14
            22: 14 14
            8: 42
            26: 14 22 | 1 20
            18: 15 15
            7: 14 5 | 1 21
            24: 14 1",
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
            bbabbbbaabaabba
            babbbbaabbbbbabbbbbbaabaaabaaa
            aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            bbbbbbbaaaabbbbaaabbabaaa
            bbbababbbbaaaaaaaabbababaaababaabab
            ababaaaaaabaaab
            ababaaaaabbbaba
            baabbaaaabbaaaababbaababb
            abbbbabbbbaaaababbbbbbaaaababb
            aaaaabbaabaaaaababaa
            aaaabbaaaabbaaa
            aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            babaaabbbaaabaababbaabababaaab
            aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
            "Part 1: 3\nPart 2: 12"
        )
    )]
    fn solve(rules: &str, messages: &str, expected: &str) {
        let day = Day19 {};
        assert_eq!(
            day.solve(hashmap! {"Rules" => rules, "Messages" => messages})
                .unwrap(),
            expected
        );
    }
}
