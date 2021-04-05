use anyhow::Result;
use modinverse::egcd;
use num_integer::Integer;

use crate::groups::challenge_config::{ChallengeConfig, ChallengeError};

pub struct Day13 {}

impl Day13 {
    fn align_phases(a_period: i128, a_phase: i128, b_period: i128, b_phase: i128) -> (i128, i128) {
        let (gcd, s, _t) = egcd(a_period, b_period);
        let phase_difference = a_phase - b_phase;
        let (pd_mult, pd_remainder) = phase_difference.div_rem(&gcd);
        if pd_remainder != 0 {
            panic!("Reference points never synchronize.");
        }
        let combined_period = a_period.div_floor(&(gcd)) * b_period;
        let combined_phase = (a_phase - s * pd_mult * a_period) % combined_period;
        return (combined_period, combined_phase);
    }
}

impl ChallengeConfig for Day13 {
    fn title(&self) -> &str {
        return "Day 13: Shuttle Search";
    }

    fn solve(&self, input: &str) -> Result<String> {
        let (first, second) = input
            .split_once("\n")
            .ok_or(ChallengeError::new("Unable to split into two parts"))?;
        let earliest_time = first.parse::<i128>()?;
        let buses: Vec<i128> = second
            .split(",")
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .map(|x| {
                if x == "x" {
                    return "0";
                }
                return x;
            })
            .map(str::parse::<i128>)
            .collect::<Result<_, _>>()?;

        let mut part_one = 0;
        let mut time = earliest_time.clone();
        while part_one == 0 {
            for bus in buses.clone().iter().filter(|x| **x > 0) {
                if time % bus == 0 {
                    part_one = bus * (time - earliest_time);
                }
            }
            time += 1;
        }

        let mut part_two = 0;
        let mut lcm = 1;
        for (i, bus) in buses.iter().enumerate() {
            if *bus == 0 {
                continue;
            }
            let (period, mut phase) = Self::align_phases(lcm, part_two, *bus, -(i as i128));
            while phase < 0 {
                phase += period;
            }
            part_two = phase;
            lcm = period;
        }

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        input,
        expected,
        case("0\n9,x,x,15", "Part 1: 81\nPart 2: 27"),
        case("0\n9,x,x,15,x,x,x,14", "Part 1: 81\nPart 2: 567"),
        case("0\n17,x,13", "Part 1: 169\nPart 2: 102"),
        case("0\n17,x,13,19", "Part 1: 169\nPart 2: 3417"),
        case("0\n67,7,59,61", "Part 1: 49\nPart 2: 754018"),
        case("0\n67,x,7,59,61", "Part 1: 49\nPart 2: 779210"),
        case("0\n67,7,x,59,61", "Part 1: 49\nPart 2: 1261476"),
        case("0\n1789,37,47,1889", "Part 1: 1369\nPart 2: 1202161486"),
        case("939\n7,13,x,x,59,x,31,19", "Part 1: 295\nPart 2: 1068781")
    )]
    fn solve(input: &str, expected: &str) {
        let day = Day13 {};
        assert_eq!(day.solve(input).unwrap(), expected);
    }
}
