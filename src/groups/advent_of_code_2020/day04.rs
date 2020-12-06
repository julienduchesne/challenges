use std::collections::HashMap;

use super::super::challenge_config::ChallengeConfig;
use super::super::challenge_config::ChallengeError;
use super::super::challenge_config::VariableType;
use maplit::hashmap;
use regex::Regex;

pub struct Day4 {}

impl Day4 {
    fn count_valid(&self, passports: Vec<HashMap<&str, &str>>, ok_if_present: bool) -> usize {
        let mut valid = 0;
        for passport in passports {
            let mut is_valid = true;

            for field in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
                is_valid = is_valid && passport.contains_key(field);
            }
            if !ok_if_present && is_valid {
                // byr
                match passport.get("byr").unwrap().parse::<usize>() {
                    Ok(byr) => is_valid = is_valid && byr >= 1920 && byr <= 2002,
                    Err(_) => is_valid = false,
                }
                // iyr
                match passport.get("iyr").unwrap().parse::<usize>() {
                    Ok(iyr) => is_valid = is_valid && iyr >= 2010 && iyr <= 2020,
                    Err(_) => is_valid = false,
                }
                // eyr
                match passport.get("eyr").unwrap().parse::<usize>() {
                    Ok(eyr) => is_valid = is_valid && eyr >= 2020 && eyr <= 2030,
                    Err(_) => is_valid = false,
                }
                // hgt
                let hgt_field = passport.get("hgt").unwrap();
                if hgt_field.contains("cm") {
                    match hgt_field.replace("cm", "").parse::<usize>() {
                        Ok(hgt) => is_valid = is_valid && hgt >= 150 && hgt <= 193,
                        Err(_) => is_valid = false,
                    }
                } else {
                    match hgt_field.replace("in", "").parse::<usize>() {
                        Ok(hgt) => is_valid = is_valid && hgt >= 59 && hgt <= 76,
                        Err(_) => is_valid = false,
                    }
                }
                // hcl
                is_valid = is_valid
                    && Regex::new(r"^#[0-9a-f]{6}$")
                        .unwrap()
                        .is_match(passport.get("hcl").unwrap());
                // ecl
                is_valid = is_valid
                    && (&["amb", "blu", "brn", "gry", "grn", "hzl", "oth"])
                        .contains(passport.get("ecl").unwrap());
                // pid
                is_valid = is_valid
                    && Regex::new(r"^\d{9}$")
                        .unwrap()
                        .is_match(passport.get("pid").unwrap());
            }

            if is_valid {
                valid += 1;
            }
        }
        return valid;
    }
}

impl ChallengeConfig for Day4 {
    fn title(&self) -> &str {
        return "Day 4: Passport Processing";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> HashMap<&str, crate::groups::challenge_config::VariableType> {
        return hashmap! {"passports" => VariableType::MultiLineString};
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String, ChallengeError> {
        let input_without_double_spaces = variables["passports"].replace("  ", "");
        let blocks: Vec<&str> = input_without_double_spaces
            .split("\n\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect();
        let passports: Vec<HashMap<&str, &str>> = blocks
            .iter()
            .map(|x| {
                x.split_whitespace()
                    .into_iter()
                    .map(|f| (f.split(":").nth(0).unwrap(), f.split(":").nth(1).unwrap()))
                    .collect()
            })
            .collect();

        let part_one = self.count_valid(passports.clone(), true);
        let part_two = self.count_valid(passports.clone(), false);

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        passports,
        expected,
        case(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm
            
            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929
            
            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm
            
            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in",
            "Part 1: 2\nPart 2: 2"
        ),
        // Invalid for part 2
        case(
            "eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
            
            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946
            
            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
            
            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007",
            "Part 1: 4\nPart 2: 0"
        ),
        // Valid for part 2
        case(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f
            
            eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
            
            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022
            
            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
            "Part 1: 4\nPart 2: 4"
        )
    )]
    fn solve(passports: &str, expected: &str) {
        let day = Day4 {};
        assert_eq!(
            day.solve(hashmap! {"passports" => passports}).unwrap(),
            expected
        );
    }
}
