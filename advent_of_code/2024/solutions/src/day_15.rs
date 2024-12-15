// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2024-day15/

use crate::{AoCData, AoCResult};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(self.row + other.row, self.col + other.col)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Robot,
    Box,
    BoxLeft,
    BoxRight,
}

fn embiggen_line(line: impl IntoIterator<Item = char>) -> impl Iterator<Item = char> {
    line.into_iter().flat_map(|c| {
        let replacement = match c {
            '#' => "##",
            'O' => "[]",
            '.' => "..",
            '@' => "@.",
            _ => panic!("at the disco"),
        };
        replacement.chars()
    })
}

fn parse_grid(input: &str, p2: bool) -> HashMap<Point, Tile> {
    let mut grid = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        // this looks cursed, I don't like it, it works though 🤷
        let chars = if p2 {
            Box::new(embiggen_line(line.chars())) as Box<dyn std::iter::Iterator<Item = char>>
        } else {
            Box::new(line.chars())
        };
        for (col, c) in chars.enumerate() {
            let point = Point::new(row as i32, col as i32);
            let tile = match c {
                '@' => Tile::Robot,
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                'O' => Tile::Box,
                '[' => Tile::BoxLeft,
                ']' => Tile::BoxRight,
                _ => panic!("at the disco"),
            };
            grid.insert(point, tile);
        }
    }
    grid
}

fn parse_instructions(input: &str) -> Vec<Point> {
    let mut instructions = Vec::new();
    for c in input.chars() {
        let point = match c {
            '^' => Point::new(-1, 0),
            '>' => Point::new(0, 1),
            'v' => Point::new(1, 0),
            '<' => Point::new(0, -1),
            _ => continue,
        };
        instructions.push(point);
    }
    instructions
}

fn do_instructions(
    mut grid: HashMap<Point, Tile>,
    instructions: Vec<Point>,
) -> HashMap<Point, Tile> {
    let mut robot = grid
        .iter()
        .find_map(|(point, tile)| (*tile == Tile::Robot).then_some(*point))
        .unwrap();

    'outer: for inst in instructions {
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();
        q.push_back(robot);

        while let Some(point) = q.pop_front() {
            if !seen.insert(point) {
                continue;
            }
            let new = point.add(&inst);
            let new_tile = grid[&new];
            match new_tile {
                Tile::Robot => continue,
                Tile::Empty => continue,
                Tile::Wall => continue 'outer,
                Tile::Box => q.push_back(new),
                Tile::BoxLeft => {
                    q.push_back(new);
                    let right = Point::new(new.row, new.col + 1);
                    q.push_back(right);
                }
                Tile::BoxRight => {
                    q.push_back(new);
                    let left = Point::new(new.row, new.col - 1);
                    q.push_back(left);
                }
            }
        }

        while !seen.is_empty() {
            for point in seen.clone() {
                let new = point.add(&inst);
                if !seen.contains(&new) {
                    *grid.entry(new).or_insert(Tile::Empty) = grid[&point];
                    *grid.entry(point).or_insert(Tile::Empty) = Tile::Empty;
                    seen.remove(&point);
                }
            }
        }

        robot = robot.add(&inst);
    }

    grid
}

fn gps(grid: &HashMap<Point, Tile>, needle: Tile) -> i32 {
    grid.iter()
        .filter_map(|(point, &tile)| (tile == needle).then_some(100 * point.row + point.col))
        .sum()
}

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let (grid, instructions) = self.0.split_once("\n\n").unwrap();
        let mut grid = parse_grid(grid, false);
        let instructions = parse_instructions(instructions);
        grid = do_instructions(grid, instructions);
        Ok(gps(&grid, Tile::Box))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let (grid, instructions) = self.0.split_once("\n\n").unwrap();
        let mut grid = parse_grid(grid, true);
        let instructions = parse_instructions(instructions);
        grid = do_instructions(grid, instructions);
        Ok(gps(&grid, Tile::BoxLeft))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "10092");
    }

    #[test]
    fn part_2() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "9021");
    }
}
