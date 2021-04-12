use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::groups::challenge_config::{ChallengeConfig, ChallengeError};

#[derive(Copy, Clone)]
enum Move {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

fn remove_first(s: &mut String) -> Option<char> {
    let mut chars = s.chars();
    let popped = chars.next();
    *s = chars.as_str().to_string();
    return popped;
}

impl Move {
    fn take_first(tiles: &mut String) -> Result<Move> {
        let first = match remove_first(tiles) {
            Some(x) => x,
            None => {
                return Err(ChallengeError::new("No tiles remaining for first character").into())
            }
        };

        let tile = match first {
            'e' => Move::East,
            'w' => Move::West,
            's' | 'n' => {
                let second = match remove_first(tiles) {
                    Some(x) => x,
                    None => {
                        return Err(
                            ChallengeError::new("No tiles remaining for second character").into(),
                        )
                    }
                };
                match second {
                    'e' => {
                        if first == 'n' {
                            Move::NorthEast
                        } else {
                            Move::SouthEast
                        }
                    }
                    'w' => {
                        if first == 'n' {
                            Move::NorthWest
                        } else {
                            Move::SouthWest
                        }
                    }
                    _ => return Err(ChallengeError::new("Invalid second tile character").into()),
                }
            }
            _ => return Err(ChallengeError::new("Invalid first tile character").into()),
        };
        return Ok(tile);
    }

    fn adjacent_coords(tile: (i32, i32)) -> [(i32, i32); 6] {
        return [
            (tile.0 - 1, tile.1 - 1), // nw
            (tile.0 + 1, tile.1 - 1), // sw
            (tile.0 - 1, tile.1 + 1), // ne
            (tile.0 + 1, tile.1 + 1), // se
            (tile.0, tile.1 - 2),     // w
            (tile.0, tile.1 + 2),     // e
        ];
    }

    fn to_coords(moves: Vec<Move>) -> (i32, i32) {
        let mut coords = (0, 0);
        for m in moves {
            coords.0 += match m {
                Move::SouthEast | Move::SouthWest => 1,
                Move::NorthEast | Move::NorthWest => -1,
                _ => 0,
            };
            coords.1 += match m {
                Move::East => 2,
                Move::SouthEast | Move::NorthEast => 1,
                Move::SouthWest | Move::NorthWest => -1,
                Move::West => -2,
            };
        }
        return coords;
    }
}

pub struct Day24 {}

impl ChallengeConfig for Day24 {
    fn title(&self) -> &str {
        return "Day 24: Lobby Layout";
    }

    fn solve(&self, input: &str) -> Result<String> {
        let tiles: Vec<Vec<Move>> = input
            .split_whitespace()
            .map(|s| {
                let mut moves = vec![];
                let mut to_parse = s.trim().to_string();
                while !to_parse.is_empty() {
                    moves.push(Move::take_first(&mut to_parse));
                }
                moves
                    .iter()
                    .map(|r| {
                        let result: Result<_> = match r {
                            Ok(o) => Ok(*o),
                            Err(e) => Err(ChallengeError::new(
                                format!("Caught an error while parsing {}: {}", s, e).as_str(),
                            )
                            .into()),
                        };
                        return result;
                    })
                    .collect::<Result<Vec<Move>, _>>()
            })
            .collect::<Result<_, _>>()?;

        let mut black_tiles: HashSet<(i32, i32)> = HashSet::new();
        for tile in tiles {
            let to_flip = Move::to_coords(tile);
            if black_tiles.contains(&to_flip) {
                black_tiles.remove(&to_flip);
            } else {
                black_tiles.insert(to_flip);
            }
        }

        let part_one = black_tiles.len();

        for _ in 1..=100 {
            let mut adjacent_black_count: HashMap<(i32, i32), u32> = HashMap::new();
            for tile in black_tiles.iter() {
                for adjacent in Move::adjacent_coords(*tile).iter() {
                    adjacent_black_count.insert(
                        *adjacent,
                        adjacent_black_count.get(adjacent).unwrap_or(&0) + 1,
                    );
                }
            }

            let to_add: Vec<(i32, i32)> = adjacent_black_count
                .iter()
                .filter(|(coords, &count)| !black_tiles.contains(coords) && count == 2)
                .map(|(&coords, _)| coords)
                .collect();

            for tile in black_tiles.clone().iter() {
                let adjacent_count = *adjacent_black_count.get(&tile).unwrap_or(&0);
                if adjacent_count == 0 || adjacent_count > 2 {
                    black_tiles.remove(&tile);
                }
            }
            for tile in to_add {
                black_tiles.insert(tile);
            }
        }

        let part_two = black_tiles.len();

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        tiles,
        expected,
        case(
            "sesenwnenenewseeswwswswwnenewsewsw
            neeenesenwnwwswnenewnwwsewnenwseswesw
            seswneswswsenwwnwse
            nwnwneseeswswnenewneswwnewseswneseene
            swweswneswnenwsewnwneneseenw
            eesenwseswswnenwswnwnwsewwnwsene
            sewnenenenesenwsewnenwwwse
            wenwwweseeeweswwwnwwe
            wsweesenenewnwwnwsenewsenwwsesesenwne
            neeswseenwwswnwswswnw
            nenwswwsewswnenenewsenwsenwnesesenew
            enewnwewneswsewnwswenweswnenwsenwsw
            sweneswneswneneenwnewenewwneswswnese
            swwesenesewenwneswnwwneseswwne
            enesenwswwswneneswsenwnewswseenwsese
            wnwnesenesenenwwnenwsewesewsesesew
            nenewswnwewswnenesenwnesewesw
            eneswnwswnwsenenwnwnwwseeswneewsenese
            neswnwewnwnwseenwseesewsenwsweewe
            wseweeenwnesenwwwswnew",
            "Part 1: 10\nPart 2: 2208"
        )
    )]
    fn solve(tiles: &str, expected: &str) {
        let day = Day24 {};
        assert_eq!(day.solve(tiles).unwrap(), expected);
    }
}
