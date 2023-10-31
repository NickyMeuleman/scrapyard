use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<Coord>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Rock,
    Sand,
    Air,
}

impl Coord {
    fn neighbours(&self) -> [Coord; 3] {
        let down = Coord {
            x: self.x,
            y: self.y + 1,
        };
        let down_left = Coord {
            x: self.x - 1,
            y: self.y + 1,
        };
        let down_right = Coord {
            x: self.x + 1,
            y: self.y + 1,
        };

        [down, down_left, down_right]
    }

    /// returns Some(Coord) of this coords first Coord it can move to, none if it is static
    fn next(&self, cave: &[Vec<Tile>], floor_y: Option<i32>) -> Option<Coord> {
        if let Some(y) = floor_y {
            if (self.y + 1) == y {
                // hit floor
                return None;
            }
        }
        // first available position in neighbours (down, left-down, right-down)
        self.neighbours()
            .into_iter()
            .find(|p| cave[p.y as usize][p.x as usize] == Tile::Air)
    }
}

fn simulate(rocks: &HashSet<Coord>, floor_y: Option<i32>) -> usize {
    let start = Coord { x: 500, y: 0 };
    let max_y = rocks.iter().map(|p| p.y).max().unwrap();
    // the width is a guessing game, in the puzzle it's infinite
    let width = 500 + max_y + 2;

    // start cave filled with air
    let mut cave: Vec<Vec<Tile>> = vec![vec![Tile::Air; width as usize]; (max_y + 2) as usize];
    // add rocks to cave
    for pos in rocks {
        cave[pos.y as usize][pos.x as usize] = Tile::Rock;
    }

    // subsequent pieces of sand flow in exactly the same path as the previous one if it's not blocked,
    let mut last_path_cache = vec![start];

    for i in 0.. {
        let mut sand = start;
        // try to reuse the path of the previous block of sand
        while let Some(pos) = last_path_cache.pop() {
            if cave[pos.y as usize][pos.x as usize] == Tile::Air {
                sand = pos;
                break;
            }
        }

        // add current position of sand to cache
        // sand coordinate is guaranteed to be unblocked at this point
        last_path_cache.push(sand);

        // the sand falls until it can't anymore and next returns None
        while let Some(next_air_coord) = sand.next(&cave, floor_y) {
            sand = next_air_coord;
            // record empty positions as sand falls so they can be filled in the future
            last_path_cache.push(sand);
            if floor_y.is_none() && sand.y > max_y {
                return i;
            }
        }

        // insert final coord into the cave as sand tile
        cave[sand.y as usize][sand.x as usize] = Tile::Sand;

        if floor_y.is_some() && sand == start {
            return i + 1;
        }
    }

    unreachable!()
}

fn rocks_in_cave(rock_lines: &[Vec<Coord>]) -> HashSet<Coord> {
    rock_lines
        .iter()
        .flat_map(|path| {
            path.iter()
                .tuple_windows()
                .flat_map(|(start, end)| {
                    let diff_x = end.x - start.x;
                    let diff_y = end.y - start.y;
                    let direction = Coord {
                        x: diff_x.signum(),
                        y: diff_y.signum(),
                    };
                    // one of two differences is always 0 because rock lines are vertical or horizontal
                    let amount = diff_x.abs().max(diff_y.abs()) as usize;

                    // generate Coord for every tile in a window
                    (0..=amount).map(move |amount| {
                        let diff_x = amount as i32 * direction.x;
                        let diff_y = amount as i32 * direction.y;

                        Coord {
                            x: start.x + diff_x,
                            y: start.y + diff_y,
                        }
                    })
                })
        })
        .collect()
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let edges = input
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(|coords| {
                        let (x, y) = coords.split_once(',').unwrap();
                        let x = x.parse().unwrap();
                        let y = y.parse().unwrap();
                        Coord { x, y }
                    })
                    .collect()
            })
            .collect();

        Ok(Self(edges))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let rocks = rocks_in_cave(&self.0);

        Ok(simulate(&rocks, None))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let rocks = rocks_in_cave(&self.0);
        let max_y = rocks.iter().map(|p| p.y).max().unwrap();

        Ok(simulate(&rocks, Some(max_y + 2)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "24");
    }

    #[test]
    fn part_2() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "93");
    }
}
