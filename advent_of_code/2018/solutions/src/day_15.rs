use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Display,
};

use itertools::Itertools;

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
    map: HashMap<Point, Tile>,
}
// Combat proceeds in rounds;
// in each round, each unit that is still alive takes a turn,
// resolving all of its actions before the next unit's turn begins.
// on each unit's turn, it tries to move into range of an enemy
// (if it isn't already) and then attack (if it is in range).
fn round(map: &mut HashMap<Point, Tile>) {
    let mut units = alive_units(map);
    units = reading_order(units);
    // All units are very disciplined and always follow very strict combat rules.
    // Units never move or attack diagonally, as doing so would be dishonorable.
    for point in units {
        take_turn(point, map);
    }
}

// returns false if combat ended
// returns true if turn was ended
fn take_turn(pos: Point, map: &mut HashMap<Point, Tile>) -> bool {
    let rows = map.keys().map(|p| p.row).max().unwrap() as usize + 1;
    let cols = map.keys().map(|p| p.col).max().unwrap() as usize + 1;
    let attacker_kind = if let Some(Tile::Unit(attacker)) = map.get(&pos) {
        attacker.kind.clone()
    } else {
        panic!("invalid unit tried to take turn");
    };
    // Each unit begins its turn by identifying all possible targets (enemy units).
    let targets: Vec<Point> = targets(map, &attacker_kind);
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
        let in_range = in_range(&targets, map);
        // If the unit is not already in range of a target, and there are no open squares which are in range of a target,
        // the unit ends its turn.
        if in_range.is_empty() {
            return true;
        }
        // Otherwise, since it is not in range of a target, it moves.
        // To move, the unit first considers the squares that are in range
        // and determines which of those squares it could reach in the fewest steps.
        // A step is a single movement to any adjacent (immediately up, down, left, or right) open (.) square.
        // Units cannot move into walls or other units.
        // The unit does this while considering the current positions of units
        // and does not do any prediction about where units will be later.
        // If the unit cannot reach (find an open path to) any of the squares that are in range, it ends its turn.
        // If multiple squares are in range and tied for being reachable in the fewest steps,
        // the square which is first in reading order is chosen.
    }
    true
}

fn chosen_and_first(
    from: Point,
    in_range: Vec<Point>,
    map: &HashMap<Point, Tile>,
) -> (Point, Point) {
    let mut nearest_with_fists = nearest_with_firsts(from, in_range, map);
    // sort in reading order of nearest point
    nearest_with_fists.sort_unstable_by(|(a, _), (b, _)| {
        a.row
            .cmp(&b.row)
            .then_with(|| a.col.cmp(&b.col))
    });

    let (chosen, mut firsts) = nearest_with_fists
        .into_iter()
        .next()
        .unwrap();
    // sort in reading order of firsts
    firsts = reading_order(firsts);

    let first = firsts.into_iter().next().unwrap();
    (chosen, first)
}
/// returns points for units of the opposite team
fn targets(map: &HashMap<Point, Tile>, attacker_kind: &Kind) -> Vec<Point> {
    alive_units(map)
        .into_iter()
        .filter(|p| matches!(&map[p], Tile::Unit(mob) if mob.kind != *attacker_kind))
        .collect()
}

/// returns points around the given points that are open
fn in_range(points: &[Point], map: &HashMap<Point, Tile>) -> Vec<Point> {
    let rows = map.keys().map(|p| p.row).max().unwrap() as usize + 1;
    let cols = map.keys().map(|p| p.col).max().unwrap() as usize + 1;
    points
        .iter()
        .flat_map(|t| t.neighbours(rows, cols))
        .filter(|p| matches!(map[p], Tile::Open))
        .collect()
}

/// returns all nearest reachable points
fn nearest_with_firsts(
    from: Point,
    in_range: Vec<Point>,
    map: &HashMap<Point, Tile>,
) -> Vec<(Point, Vec<Point>)> {
    in_range
        .into_iter()
        .map(|to| (to, shortest(from, to, map)))
        .min_set_by_key(|(_, (cost, _))| *cost)
        .into_iter()
        .map(|(to, (_, firsts))| (to, firsts))
        .collect()
}

#[derive(PartialEq, Eq)]
struct Node {
    cost: u32,
    pos: Point,
    first: Option<Point>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
/// returns a tuple of (min cost, Vec<first_step>) to a single point
fn shortest(from: Point, to: Point, map: &HashMap<Point, Tile>) -> (u32, Vec<Point>) {
    let rows = map.keys().map(|p| p.row).max().unwrap() as usize + 1;
    let cols = map.keys().map(|p| p.col).max().unwrap() as usize + 1;

    let mut q = BinaryHeap::new();
    let mut cost_map = HashMap::new();
    let mut all_firsts = Vec::new();
    let mut min_cost = u32::MAX;

    q.push(Node {
        cost: 0,
        pos: from,
        first: None,
    });
    cost_map.insert(from, 0);

    while let Some(Node { cost, pos, first }) = q.pop() {
        if pos == to {
            match cost {
                cost if cost == min_cost => {
                    // add to firsts
                    all_firsts.push(first.unwrap());
                }
                cost if cost < min_cost => {
                    // update min_cost and restart firsts
                    min_cost = cost;
                    all_firsts = vec![first.unwrap()];
                }
                _ => continue,
            }
        }

        // Check if we've found a shorter path already
        if cost > *cost_map.get(&pos).unwrap_or(&u32::MAX) {
            continue;
        }

        for n in pos
            .neighbours(rows, cols)
            .into_iter()
            .filter(|p| matches!(&map[p], Tile::Open))
        {
            let new_cost = cost + 1;
            if new_cost <= *cost_map.get(&n).unwrap_or(&u32::MAX) {
                cost_map.insert(n, new_cost);
                q.push(Node {
                    cost: new_cost,
                    pos: n,
                    first: if first.is_some() { first } else { Some(n) },
                });
            }
        }
    }

    (min_cost, all_firsts)
}

fn alive_units(map: &HashMap<Point, Tile>) -> Vec<Point> {
    // Get units that are still alive
    map.iter()
        .filter(|(_, tile)| matches!(tile, Tile::Unit(Mob { kind: _, hp }) if *hp > 0))
        .map(|(p, _)| *p)
        .collect()
}

fn reading_order(mut points: Vec<Point>) -> Vec<Point> {
    // When multiple choices are equally valid, ties are broken in reading order:
    // top-to-bottom, then left-to-right.
    // For instance,
    // the order in which units take their turns within a round
    // is the reading order of their starting positions in that round,
    // regardless of the type of unit
    // or whether other units have moved after the round started
    points.sort_unstable_by(|a, b| {
        a.row
            .cmp(&b.row)
            .then_with(|| a.col.cmp(&b.col))
    });
    points
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

fn make_2d_vec(map: &HashMap<Point, Tile>) -> Vec<Vec<char>> {
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
    let vec_2d = make_2d_vec(map);
    for line in vec_2d {
        for c in line {
            print!("{c}");
        }
        println!();
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut map = HashMap::new();
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
        Ok(Self { map })
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
        // Ok(round_num * sum_hp(&map))
        Ok(1)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(2)
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    fn vec2d_to_string(vec_2d: Vec<Vec<char>>) -> String {
        vec_2d
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n")
    }

    use super::*;
    //                  would take their
    // These units:   turns in this order:
    //   #######           #######
    //   #.G.E.#           #.1.2.#
    //   #E.G.E#           #3.4.5#
    //   #.G.E.#           #.6.7.#
    //   #######           #######
    //
    // Targets:      In range:     Reachable:    Nearest:      Chosen:
    // #######       #######       #######       #######       #######
    // #E..G.#       #E.?G?#       #E.@G.#       #E.!G.#       #E.+G.#
    // #...#.#  -->  #.?.#?#  -->  #.@.#.#  -->  #.!.#.#  -->  #...#.#
    // #.G.#G#       #?G?#G#       #@G@#G#       #!G.#G#       #.G.#G#
    // #######       #######       #######       #######       #######
    //
    // In range:     Nearest:      Chosen:       Distance:     Step:
    // #######       #######       #######       #######       #######
    // #.E...#       #.E...#       #.E...#       #4E212#       #..E..#
    // #...?.#  -->  #...!.#  -->  #...+.#  -->  #32101#  -->  #.....#
    // #..?G?#       #..!G.#       #...G.#       #432G2#       #...G.#
    // #######       #######       #######       #######       #######
    #[test]
    fn correct_reading_order() {
        let input = "#######
#.G.E.#
#E.G.E#
#.G.E.#
#######";
        let data = Data::try_new(input).unwrap();
        let mut vec_2d = make_2d_vec(&data.map);
        let mut units = alive_units(&data.map);
        units = reading_order(units);
        for (count, point) in units.iter().enumerate() {
            vec_2d[point.row as usize][point.col as usize] = ((count + 1) as u8 + b'0') as char;
        }
        let result = vec2d_to_string(vec_2d);
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
    fn correct_in_range() {
        let input = "#######
#E..G.#
#...#.#
#.G.#G#
#######";
        let data = Data::try_new(input).unwrap();
        let targets = targets(&data.map, &Kind::Elf);
        let in_range = in_range(&targets, &data.map);
        let mut vec_2d = make_2d_vec(&data.map);
        for p in in_range {
            vec_2d[p.row as usize][p.col as usize] = '?';
        }
        let result = vec2d_to_string(vec_2d);
        assert_eq!(
            result,
            "#######
#E.?G?#
#.?.#?#
#?G?#G#
#######"
        );
    }

    #[test]
    fn correct_nearest() {
        let input = "#######
#E..G.#
#...#.#
#.G.#G#
#######";
        let data = Data::try_new(input).unwrap();
        let targets = targets(&data.map, &Kind::Elf);
        let in_range = in_range(&targets, &data.map);
        let elf = Point { row: 1, col: 1 };
        let nearest: Vec<_> = nearest_with_firsts(elf, in_range, &data.map)
            .into_iter()
            .map(|(p, _)| p)
            .collect();
        let mut vec_2d = make_2d_vec(&data.map);
        for p in nearest {
            vec_2d[p.row as usize][p.col as usize] = '!';
        }
        let result = vec2d_to_string(vec_2d);
        assert_eq!(
            result,
            "#######
#E.!G.#
#.!.#.#
#!G.#G#
#######"
        );
    }

    #[test]
    fn correct_chosen() {
        let input = "#######
#E..G.#
#...#.#
#.G.#G#
#######";
        let data = Data::try_new(input).unwrap();
        let targets = targets(&data.map, &Kind::Elf);
        let in_range = in_range(&targets, &data.map);
        let elf = Point { row: 1, col: 1 };
        let (chosen, _) = chosen_and_first(elf, in_range, &data.map);
        let mut vec_2d = make_2d_vec(&data.map);
        vec_2d[chosen.row as usize][chosen.col as usize] = '+';
        let result = vec2d_to_string(vec_2d);
        assert_eq!(
            result,
            "#######
#E.+G.#
#...#.#
#.G.#G#
#######"
        );
    }

    #[test]
    fn correct_in_range_2() {
        let input = "#######
#.E...#
#.....#
#...G.#
#######";
        let data = Data::try_new(input).unwrap();
        let targets = targets(&data.map, &Kind::Elf);
        let in_range = in_range(&targets, &data.map);
        let mut vec_2d = make_2d_vec(&data.map);
        for p in in_range {
            vec_2d[p.row as usize][p.col as usize] = '?';
        }
        let result = vec2d_to_string(vec_2d);
        assert_eq!(
            result,
            "#######
#.E...#
#...?.#
#..?G?#
#######"
        );
    }
    #[test]
    fn correct_nearest_2() {
        let input = "#######
#.E...#
#.....#
#...G.#
#######";
        let data = Data::try_new(input).unwrap();
        let targets = targets(&data.map, &Kind::Elf);
        let in_range = in_range(&targets, &data.map);
        let elf = Point { row: 1, col: 1 };
        let nearest: Vec<_> = nearest_with_firsts(elf, in_range, &data.map)
            .into_iter()
            .map(|(p, _)| p)
            .collect();
        let mut vec_2d = make_2d_vec(&data.map);
        for p in nearest {
            vec_2d[p.row as usize][p.col as usize] = '!';
        }
        let result = vec2d_to_string(vec_2d);
        assert_eq!(
            result,
            "#######
#.E...#
#...!.#
#..!G.#
#######"
        );
    }

    #[test]
    fn correct_chosen_2() {
        let input = "#######
#.E...#
#.....#
#...G.#
#######";
        let data = Data::try_new(input).unwrap();
        let targets = targets(&data.map, &Kind::Elf);
        let in_range = in_range(&targets, &data.map);
        let elf = Point { row: 1, col: 1 };
        let (chosen, _) = chosen_and_first(elf, in_range, &data.map);
        let mut vec_2d = make_2d_vec(&data.map);
        vec_2d[chosen.row as usize][chosen.col as usize] = '+';
        let result = vec2d_to_string(vec_2d);
        assert_eq!(
            result,
            "#######
#.E...#
#...+.#
#...G.#
#######"
        );
    }

    #[test]
    fn part_1() {
        let input = "";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "1");
    }

    #[test]
    fn part_2() {
        let input = "";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "2");
    }
}
