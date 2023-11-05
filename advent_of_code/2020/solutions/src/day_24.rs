use std::{collections::HashMap, fmt::Display};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Point>);

enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(PartialEq, Copy, Clone)]
enum Color {
    White,
    Black,
}

fn flip(color: &mut Color) -> Color {
    match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    }
}

fn lay_input_tiles(data: &Vec<Point>) -> HashMap<Point, Color> {
    data.iter()
        .fold(HashMap::new(), |mut acc, &point| {
            let color = acc.entry(point).or_insert(Color::White);
            *color = flip(color);
            acc
        })
}

fn tick(tiles: HashMap<Point, Color>) -> HashMap<Point, Color> {
    let mut new_tiles = HashMap::new();
    let expanded_lobby = expand_lobby(tiles);
    for (point, color) in &expanded_lobby {
        let neighbours = get_neighbours(&point);
        let black_neighbours = count_color(&neighbours, &expanded_lobby, Color::Black);
        let new_color = get_new_color(*color, black_neighbours);
        new_tiles.insert(*point, new_color);
    }
    new_tiles
}

fn get_neighbours(point: &Point) -> [Point; 6] {
    let nw = Point {
        x: point.x,
        y: point.y + 1,
        z: point.z - 1,
    };
    let ne = Point {
        x: point.x + 1,
        y: point.y,
        z: point.z - 1,
    };
    let e = Point {
        x: point.x + 1,
        y: point.y - 1,
        z: point.z,
    };
    let se = Point {
        x: point.x,
        y: point.y - 1,
        z: point.z + 1,
    };
    let sw = Point {
        x: point.x - 1,
        y: point.y,
        z: point.z + 1,
    };
    let w = Point {
        x: point.x - 1,
        y: point.y + 1,
        z: point.z,
    };
    [nw, ne, e, se, sw, w]
}

fn expand_lobby(tiles: HashMap<Point, Color>) -> HashMap<Point, Color> {
    let mut expanded_lobby = tiles.clone();
    for (point, _) in &tiles {
        let neighbours = get_neighbours(&point);
        for point in &neighbours {
            if !expanded_lobby.contains_key(point) {
                expanded_lobby.insert(*point, Color::White);
            }
        }
    }
    expanded_lobby
}

fn count_color(points: &[Point], tiles: &HashMap<Point, Color>, target: Color) -> usize {
    points
        .iter()
        .map(|point| {
            // account for tiles that are not already in the map
            if let Some(color) = tiles.get(point) {
                *color
            } else {
                Color::White
            }
        })
        .filter(|&color| color == target)
        .count()
}

fn get_new_color(color: Color, num_black: usize) -> Color {
    match color {
        Color::Black => {
            if num_black == 0 || num_black > 2 {
                Color::White
            } else {
                Color::Black
            }
        }
        Color::White => {
            if num_black == 2 {
                Color::Black
            } else {
                Color::White
            }
        }
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        // use the cube grid system to represent hexagonal coordinates
        // https://www.redblobgames.com/grids/hexagons/

        fn parse_line(line: &str) -> Point {
            // turn string of unseperated instructions into vec of Direction variants
            let directions = to_directions(line);
            // turn a series of instructions from origin into a point on the cube grid
            parse_directions(directions)
        }

        fn to_directions(input: &str) -> Vec<Direction> {
            let mut input = input.to_owned();
            let mut directions = Vec::new();
            while input.len() > 0 {
                let current = &input[0..1];
                if current == "n" || current == "s" {
                    // 2 letter direction
                    let current = &input[0..2];
                    match current {
                        "ne" => directions.push(Direction::NE),
                        "nw" => directions.push(Direction::NW),
                        "se" => directions.push(Direction::SE),
                        "sw" => directions.push(Direction::SW),
                        _ => panic!("invalid direction found in input"),
                    }
                    // remove 2 letters
                    input.remove(0);
                    input.remove(0);
                } else {
                    match current {
                        "e" => directions.push(Direction::E),
                        "w" => directions.push(Direction::W),
                        _ => panic!("invalid direction found in input"),
                    }
                    // remove 1 letter
                    input.remove(0);
                }
            }
            directions
        }

        fn parse_directions(directions: Vec<Direction>) -> Point {
            let mut point = Point { x: 0, y: 0, z: 0 };
            for direction in directions {
                match direction {
                    Direction::E => {
                        point.x += 1;
                        point.y -= 1;
                    }
                    Direction::SE => {
                        point.y -= 1;
                        point.z += 1;
                    }
                    Direction::SW => {
                        point.x -= 1;
                        point.z += 1;
                    }
                    Direction::W => {
                        point.x -= 1;
                        point.y += 1;
                    }
                    Direction::NW => {
                        point.y += 1;
                        point.z -= 1;
                    }
                    Direction::NE => {
                        point.x += 1;
                        point.z -= 1;
                    }
                }
            }
            point
        }

        Ok(Self(
            input
                .lines()
                .map(|line| parse_line(line))
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let tiles = lay_input_tiles(&self.0);
        let result = tiles
            .iter()
            .filter(|(_, &color)| color == Color::Black)
            .count();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut tiles = lay_input_tiles(&self.0);
        for _ in 0..100 {
            tiles = tick(tiles);
        }
        let result = tiles
            .iter()
            .filter(|(_, &color)| color == Color::Black)
            .count();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
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
wseweeenwnesenwwwswnew";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "10");
    }

    #[test]
    fn part_2() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
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
wseweeenwnesenwwwswnew";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "2208");
    }
}
