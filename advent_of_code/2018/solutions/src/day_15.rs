use std::{collections::HashMap, fmt::Display};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    row: u32,
    col: u32,
}
impl Point {
    fn neighbours(&self, rows: usize, cols: usize) -> Vec<Point> {
        let mut res = Vec::new();
        // up
        if self.row > 0 {
            res.push(Point {
                row: self.row - 1,
                col: self.col,
            });
        }
        // right
        if self.col < cols as u32 - 1 {
            res.push(Point {
                row: self.row,
                col: self.col + 1,
            })
        }
        // down
        if self.row < rows as u32 - 1 {
            res.push(Point {
                row: self.row + 1,
                col: self.col,
            })
        }
        // left
        if self.col > 0 {
            res.push(Point {
                row: self.row,
                col: self.col - 1,
            })
        }
        res
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Goblin,
    Elf,
}

#[derive(Debug, Clone)]
struct Mob {
    kind: Kind,
    hp: u32,
}

#[derive(Debug, Clone)]
enum Tile {
    Wall,
    Open,
    Unit(Mob),
}
#[derive(Debug, Clone)]
pub struct Data {
    rows: usize,
    cols: usize,
    map: HashMap<Point, Tile>,
}
// Combat proceeds in rounds;
// in each round, each unit that is still alive takes a turn,
// resolving all of its actions before the next unit's turn begins.
// on each unit's turn, it tries to move into range of an enemy
// (if it isn't already) and then attack (if it is in range).
fn round(map: &mut HashMap<Point, Tile>) {
    let units = reading_order(map);
    // All units are very disciplined and always follow very strict combat rules.
    // Units never move or attack diagonally, as doing so would be dishonorable.
    for point in units {
        take_turn(point, map);
    }
}

// returns false if combat ended
// returns true if turn was ended
fn take_turn(unit: Point, map: &mut HashMap<Point, Tile>) -> bool {
    let rows = map.keys().map(|p| p.row).max().unwrap() as usize + 1;
    let cols = map.keys().map(|p| p.col).max().unwrap() as usize + 1;
    let mut pos = unit;
    let attacker_kind = if let Some(Tile::Unit(attacker)) = map.get(&unit) {
        attacker.kind.clone()
    } else {
        panic!("invalid unit tried to take turn");
    };
    // Each unit begins its turn by identifying all possible targets (enemy units).
    let targets: Vec<Point> = alive_units(map)
        .into_iter()
        .filter(|p| matches!(&map[p], Tile::Unit(mob) if mob.kind != attacker_kind))
        .collect();
    // If no targets remain, combat ends.
    if targets.is_empty() {
        return false;
    }
    // Then, the unit identifies all of the open squares (.) that are in range of each target;
    // these are the squares which are adjacent (immediately up, down, left, or right)
    // to any target and which aren't already occupied by a wall or another unit.
    // Alternatively, the unit might already be in range of a target.
    let already_in_range = pos
        .neighbours(rows, cols)
        .iter()
        .any(|n| targets.contains(n));
    if already_in_range {
        // If the unit is already in range of a target, it does not move,
        // but continues its turn with an attack.
    } else {
        let open_squares: Vec<Point> = targets
            .iter()
            .flat_map(|t| t.neighbours(rows, cols))
            .filter(|p| matches!(map[p], Tile::Open))
            .collect();
        // If the unit is not already in range of a target, and there are no open squares which are in range of a target,
        // the unit ends its turn.
        if open_squares.is_empty() {
            return true;
        }
        // Otherwise, since it is not in range of a target, it moves.
    }
    true
}

fn alive_units(map: &HashMap<Point, Tile>) -> Vec<Point> {
    // Get units that are still alive
    map.iter()
        .filter(|(_, tile)| matches!(tile, Tile::Unit(Mob { kind: _, hp }) if *hp > 0))
        .map(|(p, _)| *p)
        .collect()
}

fn reading_order(map: &HashMap<Point, Tile>) -> Vec<Point> {
    let mut units = alive_units(map);
    // When multiple choices are equally valid, ties are broken in reading order:
    // top-to-bottom, then left-to-right.
    // For instance,
    // the order in which units take their turns within a round
    // is the reading order of their starting positions in that round,
    // regardless of the type of unit
    // or whether other units have moved after the round started
    units.sort_unstable_by(|a, b| {
        a.row
            .cmp(&b.row)
            .then_with(|| a.col.cmp(&b.col))
    });
    units
}

/// the sum of the hit points of all remaining units
fn sum_hp(map: &HashMap<Point, Tile>) -> u32 {
    alive_units(map)
        .iter()
        .filter_map(|p| match &map[p] {
            Tile::Wall => None,
            Tile::Open => None,
            Tile::Unit(mob) => Some(mob.hp),
        })
        .sum()
}

fn map_to_charlists(map: &HashMap<Point, Tile>) -> Vec<Vec<char>> {
    let max_row = map.keys().map(|p| p.row).max().unwrap();
    let max_col = map.keys().map(|p| p.col).max().unwrap();
    let mut res = Vec::new();
    for row in 0..=max_row {
        let mut line = Vec::new();
        for col in 0..=max_col {
            let point = Point { row, col };
            let c = match &map[&point] {
                Tile::Wall => '#',
                Tile::Open => '.',
                Tile::Unit(mob) => match mob.kind {
                    Kind::Goblin => 'G',
                    Kind::Elf => 'E',
                },
            };
            line.push(c);
        }
        res.push(line);
    }
    res
}

fn show(map: &HashMap<Point, Tile>) {
    let charlists = map_to_charlists(map);
    for line in charlists {
        for c in line {
            print!("{c}");
        }
        println!();
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut map = HashMap::new();
        let rows = input.lines().count();
        let cols = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .count();
        for (row_idx, line) in input.lines().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                let point = Point {
                    row: row_idx as u32,
                    col: col_idx as u32,
                };
                let tile = match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Open,
                    'G' => Tile::Unit(Mob {
                        kind: Kind::Goblin,
                        hp: 200,
                    }),
                    'E' => Tile::Unit(Mob {
                        kind: Kind::Elf,
                        hp: 200,
                    }),
                    _ => panic!("Invalid input"),
                };
                map.insert(point, tile);
            }
        }
        Ok(Self { rows, cols, map })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut map = self.map.clone();
        let mut round_num = 0;
        loop {
            if round_num == 10 {
                break;
            }
            round(&mut map);
            round_num += 1;
        }
        Ok(round_num * sum_hp(&map))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok("")
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn correct_reading_order() {
        let input = "#######
#.G.E.#
#E.G.E#
#.G.E.#
#######";
        let data = Data::try_new(input).unwrap();
        let mut charlists = map_to_charlists(&data.map);
        let units = reading_order(&data.map);
        for (count, point) in units.iter().enumerate() {
            charlists[point.row as usize][point.col as usize] = ((count + 1) as u8 + b'0') as char;
        }
        let result = charlists
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n");
        assert_eq!(
            result,
            "#######
#.1.2.#
#3.4.5#
#.6.7.#
#######"
        );
    }

    #[test]
    fn part_1() {
        let input = "";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "");
    }

    #[test]
    fn part_2() {
        let input = "";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "");
    }
}
