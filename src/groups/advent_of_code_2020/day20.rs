use std::collections::{HashMap, HashSet};

use anyhow::Result;
use ndarray::{Array, Array2, Axis};
use num_integer::Roots;
use rand::seq::SliceRandom;

use super::super::challenge_config::ChallengeConfig;

pub struct Day20 {}
trait FlipRotate {
    fn rotate(&mut self);
    fn flip_horizontal(&mut self);
    fn flip_vertical(&mut self);
}

impl FlipRotate for Array2<bool> {
    fn rotate(&mut self) {
        let clone = self.clone();
        for ((x, y), item) in clone.indexed_iter() {
            self[(y, clone.nrows() - x - 1)] = *item;
        }
    }

    fn flip_horizontal(&mut self) {
        let clone = self.clone();
        for ((x, y), item) in clone.indexed_iter() {
            self[(x, clone.nrows() - y - 1)] = *item;
        }
    }

    fn flip_vertical(&mut self) {
        let clone = self.clone();
        for ((x, y), item) in clone.indexed_iter() {
            self[(clone.nrows() - x - 1, y)] = *item;
        }
    }
}

#[derive(Clone)]
struct Tile {
    id: usize,
    array: Array2<bool>,
    adjacent: [Option<usize>; 4],
}

impl Tile {
    fn parse(tile: &str) -> Result<Self> {
        let array_lines: Vec<&str> = tile.split(":").nth(1).unwrap().trim().split("\n").collect();
        let n = array_lines.len();
        let mut array: Array2<bool> = Array2::from_shape_fn((n, n), |_| false);
        for (i, mut row) in array.axis_iter_mut(Axis(0)).enumerate() {
            row.assign(&Array::from(
                array_lines[i]
                    .trim()
                    .chars()
                    .map(|x| match x {
                        '#' => true,
                        _ => false,
                    })
                    .collect::<Vec<bool>>(),
            ));
        }
        return Ok(Self {
            id: tile.split(":").nth(0).unwrap().trim().parse::<usize>()?,
            array: array,
            adjacent: [None, None, None, None],
        });
    }

    // top, right, bottom, left
    fn get_sides(&self) -> Vec<Vec<bool>> {
        return [
            self.array.row(0),
            self.array.column(self.array.ncols() - 1),
            self.array.row(self.array.nrows() - 1),
            self.array.column(0),
        ]
        .iter()
        .map(|x| x.iter().map(|e| *e).collect::<Vec<bool>>())
        .collect();
    }

    fn rotate(&mut self) {
        self.array.rotate();
        self.adjacent = [
            self.adjacent[3],
            self.adjacent[0],
            self.adjacent[1],
            self.adjacent[2],
        ];
    }

    fn flip_vertical(&mut self) {
        self.array.flip_vertical();
        self.adjacent = [
            self.adjacent[2],
            self.adjacent[1],
            self.adjacent[0],
            self.adjacent[3],
        ];
    }

    fn flip_horizontal(&mut self) {
        self.array.flip_horizontal();
        self.adjacent = [
            self.adjacent[0],
            self.adjacent[3],
            self.adjacent[2],
            self.adjacent[1],
        ];
    }
}

impl ChallengeConfig for Day20 {
    fn title(&self) -> &str {
        return "Day 20: Jurassic Jigsaw";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["Tiles".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        let mut tiles: Vec<Tile> = variables["Tiles"]
            .split("Tile")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(Tile::parse)
            .collect::<Result<_, _>>()?;

        // Find adjacent tiles
        let tiles_clone = tiles.clone();
        for tile in tiles.iter_mut() {
            for other_tile in &tiles_clone {
                for (i, side) in tile.get_sides().iter().enumerate() {
                    if other_tile.id == tile.id {
                        continue;
                    }
                    let other_sides = other_tile.get_sides();
                    if other_sides.contains(&side.iter().rev().map(|x| *x).collect())
                        || other_sides.contains(side)
                    {
                        tile.adjacent[i] = Some(other_tile.id);
                    }
                }
            }
        }

        let corner_tiles: Vec<&Tile> = tiles
            .iter()
            .filter(|t| t.adjacent.iter().filter(|x| x.is_none()).count() == 2)
            .collect();

        let part_one: usize = corner_tiles.iter().map(|x| x.id).product::<usize>();

        // Includes corners
        let side_tiles: Vec<&Tile> = tiles
            .iter()
            .filter(|t| t.adjacent.iter().filter(|x| x.is_none()).count() >= 1)
            .collect();

        // Calculate array of array
        let size = tiles.len().sqrt();
        let mut array_of_tiles: Array2<usize> = Array2::zeros((size, size));
        let mut handled: HashSet<usize> = HashSet::new();
        for x in 0..size {
            for y in 0..size {
                let new_value: usize;
                if x == 0 && y == 0 {
                    new_value = corner_tiles.iter().next().unwrap().id;
                } else if x == 0 {
                    let previous = array_of_tiles.get((0, y - 1)).unwrap().clone();
                    new_value = side_tiles
                        .iter()
                        .find(|t| !handled.contains(&t.id) && t.adjacent.contains(&Some(previous)))
                        .unwrap()
                        .id;
                } else if y == 0 {
                    let previous_x = array_of_tiles.get((x - 1, y)).unwrap().clone();
                    new_value = side_tiles
                        .iter()
                        .find(|t| {
                            !handled.contains(&t.id) && t.adjacent.contains(&Some(previous_x))
                        })
                        .unwrap()
                        .id;
                } else {
                    let previous_x = array_of_tiles.get((x - 1, y)).unwrap().clone();
                    let previous_y = array_of_tiles.get((x, y - 1)).unwrap().clone();
                    new_value = tiles
                        .iter()
                        .find(|t| {
                            !handled.contains(&t.id)
                                && t.adjacent.contains(&Some(previous_x))
                                && t.adjacent.contains(&Some(previous_y))
                        })
                        .unwrap()
                        .id;
                }
                handled.insert(new_value);
                array_of_tiles[(x, y)] = new_value;
            }
        }

        // Flip and rotate tiles in the array of tiles
        for ((x, y), tile_id) in array_of_tiles.indexed_iter() {
            // top, right, bottom, left
            let mut previous_x = size;
            let mut previous_y = size;
            if x != 0 {
                previous_x = x - 1;
            }
            if y != 0 {
                previous_y = y - 1;
            }
            let expected = [
                array_of_tiles.get((previous_x, y)),
                array_of_tiles.get((x, y + 1)),
                array_of_tiles.get((x + 1, y)),
                array_of_tiles.get((x, previous_y)),
            ];
            let tile = tiles.iter_mut().find(|t| t.id == *tile_id).unwrap();
            while (0..3).any(|x| tile.adjacent[x].as_ref() != expected[x]) {
                // Random flip and rotate till it works :shrug:
                match (vec![0, 1, 2]).choose(&mut rand::thread_rng()).unwrap() {
                    0 => tile.flip_horizontal(),
                    1 => tile.flip_vertical(),
                    2 => tile.rotate(),
                    _ => {}
                };
            }
        }

        let tile_size = tiles[0].array.ncols();
        let final_array_size = size * (tile_size - 2);
        let mut final_array: Array2<bool> =
            Array2::from_shape_fn((final_array_size, final_array_size), |_| false);
        for ((tile_x, tile_y), tile_id) in array_of_tiles.indexed_iter() {
            let tile = tiles.iter().find(|t| t.id == *tile_id).unwrap();
            for ((x, y), item) in tile.array.indexed_iter() {
                if x == 0 || x == tile_size - 1 || y == 0 || y == tile_size - 1 {
                    continue;
                }
                final_array[(
                    tile_x * (tile_size - 2) + x - 1,
                    tile_y * (tile_size - 2) + y - 1,
                )] = *item;
            }
        }

        let mut sea_monsters = 0;
        let mut monster_shape: Array2<bool> = Array2::from_shape_fn((3, 20), |_| false);
        monster_shape[(0, 18)] = true;
        monster_shape[(1, 0)] = true;
        monster_shape[(1, 5)] = true;
        monster_shape[(1, 6)] = true;
        monster_shape[(1, 11)] = true;
        monster_shape[(1, 12)] = true;
        monster_shape[(1, 17)] = true;
        monster_shape[(1, 18)] = true;
        monster_shape[(1, 19)] = true;
        monster_shape[(2, 1)] = true;
        monster_shape[(2, 4)] = true;
        monster_shape[(2, 7)] = true;
        monster_shape[(2, 10)] = true;
        monster_shape[(2, 13)] = true;
        monster_shape[(2, 16)] = true;

        while sea_monsters == 0 {
            for ((x, y), item) in final_array.indexed_iter() {
                if x == 0 || *item == false || y + 20 > final_array.ncols() {
                    continue;
                }

                let mut good = true;
                for ((m_x, m_y), m_val) in monster_shape.indexed_iter() {
                    if *m_val == false {
                        continue;
                    }
                    if final_array[(x + m_x - 1, y + m_y)] != true {
                        good = false;
                        break;
                    }
                }

                if good {
                    sea_monsters += 1;
                }
            }
            if sea_monsters == 0 {
                match (vec![0, 1, 2]).choose(&mut rand::thread_rng()).unwrap() {
                    0 => final_array.rotate(),
                    1 => final_array.flip_vertical(),
                    2 => final_array.flip_horizontal(),
                    _ => {}
                };
            }
        }

        let part_two: usize = final_array.iter().filter(|x| **x).count() - sea_monsters * 15;

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use ndarray::arr2;
    use rstest::rstest;

    use super::*;

    #[rstest(
        tiles,
        expected,
        case(
            "Tile 2311:
            ..##.#..#.
            ##..#.....
            #...##..#.
            ####.#...#
            ##.##.###.
            ##...#.###
            .#.#.#..##
            ..#....#..
            ###...#.#.
            ..###..###
            
            Tile 1951:
            #.##...##.
            #.####...#
            .....#..##
            #...######
            .##.#....#
            .###.#####
            ###.##.##.
            .###....#.
            ..#.#..#.#
            #...##.#..
            
            Tile 1171:
            ####...##.
            #..##.#..#
            ##.#..#.#.
            .###.####.
            ..###.####
            .##....##.
            .#...####.
            #.##.####.
            ####..#...
            .....##...
            
            Tile 1427:
            ###.##.#..
            .#..#.##..
            .#.##.#..#
            #.#.#.##.#
            ....#...##
            ...##..##.
            ...#.#####
            .#.####.#.
            ..#..###.#
            ..##.#..#.
            
            Tile 1489:
            ##.#.#....
            ..##...#..
            .##..##...
            ..#...#...
            #####...#.
            #..#.#.#.#
            ...#.#.#..
            ##.#...##.
            ..##.##.##
            ###.##.#..
            
            Tile 2473:
            #....####.
            #..#.##...
            #.##..#...
            ######.#.#
            .#...#.#.#
            .#########
            .###.#..#.
            ########.#
            ##...##.#.
            ..###.#.#.
            
            Tile 2971:
            ..#.#....#
            #...###...
            #.#.###...
            ##.##..#..
            .#####..##
            .#..####.#
            #..#.#..#.
            ..####.###
            ..#.#.###.
            ...#.#.#.#
            
            Tile 2729:
            ...#.#.#.#
            ####.#....
            ..#.#.....
            ....#..#.#
            .##..##.#.
            .#.####...
            ####.#.#..
            ##.####...
            ##..#.##..
            #.##...##.
            
            Tile 3079:
            #.#.#####.
            .#..######
            ..#.......
            ######....
            ####.#..#.
            .#...#.##.
            #.#####.##
            ..#.###...
            ..#.......
            ..#.###...",
            "Part 1: 20899048083289\nPart 2: 273"
        )
    )]
    fn solve(tiles: &str, expected: &str) {
        let day = Day20 {};
        assert_eq!(day.solve(hashmap! {"Tiles" => tiles}).unwrap(), expected);
    }

    #[rstest(
        tile,
        expected,
        case(
            Tile{id: 1, adjacent: [Some(2), Some(3), None, None], array: arr2(&[[true, false, false], [false, false, false], [true, true, true]])},
            Tile{id: 1, adjacent: [None, Some(3), Some(2), None], array: arr2(&[[true, true, true], [false, false, false], [true, false, false]])},
        )
    )]
    fn flip_vertical(tile: Tile, expected: Tile) {
        let mut test = tile.clone();
        test.flip_vertical();
        assert_eq!(test.adjacent, expected.adjacent);
        assert_eq!(test.array, expected.array);
    }

    #[rstest(
        tile,
        expected,
        case(
            Tile{id: 1, adjacent: [Some(2), Some(3), None, None], array: arr2(&[[true, false, false], [false, false, false], [true, true, true]])},
            Tile{id: 1, adjacent: [Some(2), None, None, Some(3)], array: arr2(&[[false, false, true], [false, false, false], [true, true, true]])},
        )
    )]
    fn flip_horizontal(tile: Tile, expected: Tile) {
        let mut test = tile.clone();
        test.flip_horizontal();
        assert_eq!(test.adjacent, expected.adjacent);
        assert_eq!(test.array, expected.array);
    }

    #[rstest(
        tile,
        expected,
        case(
            Tile{id: 1, adjacent: [Some(2), Some(3), None, None], array: arr2(&[[true, false, false], [false, false, false], [true, true, true]])},
            Tile{id: 1, adjacent: [None, Some(2), Some(3), None], array: arr2(&[[true, false, true], [true, false, false], [true, false, false]])},
        )
    )]
    fn rotate(tile: Tile, expected: Tile) {
        let mut test = tile.clone();
        test.rotate();
        assert_eq!(test.adjacent, expected.adjacent);
        assert_eq!(test.array, expected.array);
    }
}
