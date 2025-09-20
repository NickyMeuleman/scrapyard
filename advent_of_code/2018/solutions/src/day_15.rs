use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Kind {
    Goblin,
    Elf,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Mob {
    id: u32,
    pos: Point,
    kind: Kind,
    hp: u32,
}

#[derive(Debug, Clone)]
enum Tile {
    Wall,
    Open,
}

#[derive(Debug, Clone)]
pub struct Data {
    map: HashMap<Point, Tile>,
    units: HashMap<u32, Mob>,
}

fn alive_units(unit_ids: &[u32], units: &HashMap<u32, Mob>) -> Vec<u32> {
    unit_ids
        .iter()
        .filter(|&id| units[id].hp > 0)
        .copied()
        .collect()
}

fn reading_order(unit_ids: &[u32], units: &HashMap<u32, Mob>) -> Vec<u32> {
    unit_ids
        .iter()
        .sorted_unstable_by(|&a, &b| {
            let a = &units[a];
            let b = &units[b];
            a.pos
                .row
                .cmp(&b.pos.row)
                .then_with(|| a.pos.col.cmp(&b.pos.col))
        })
        .copied()
        .collect()
}

fn targets(attacker_kind: &Kind, unit_ids: &[u32], units: &HashMap<u32, Mob>) -> Vec<u32> {
    unit_ids
        .iter()
        .filter(|&id| units[id].kind != *attacker_kind)
        .copied()
        .collect()
}

fn in_range(unit_ids: &[u32], units: &HashMap<u32, Mob>, map: &HashMap<Point, Tile>) -> Vec<Point> {
    let rows = map.keys().map(|p| p.row).max().unwrap() as usize + 1;
    let cols = map.keys().map(|p| p.col).max().unwrap() as usize + 1;
    unit_ids
        .iter()
        .flat_map(|id| {
            let target = &units[id];
            target.pos.neighbours(rows, cols)
        })
        // ensure open map tile
        .filter(|p| matches!(map[p], Tile::Open))
        // ensure tile not currently occupied
        .filter(|p| {
            units
                .values()
                .all(|unit| unit.pos != *p)
        })
        .collect()
}

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
    firsts.sort_unstable_by(|a, b| {
        a.row
            .cmp(&b.row)
            .then_with(|| a.col.cmp(&b.col))
    });
    let first = firsts.into_iter().next().unwrap();
    (chosen, first)
}

// returns false if combat ended
// returns true if turn was ended
// fn take_turn(pos: Point, map: &mut HashMap<Point, Tile>) -> bool {
//     let rows = map.keys().map(|p| p.row).max().unwrap() as usize + 1;
//     let cols = map.keys().map(|p| p.col).max().unwrap() as usize + 1;
//     let attacker_kind = if let Some(Tile::Unit(attacker)) = map.get(&pos) {
//         attacker.kind.clone()
//     } else {
//         panic!("invalid unit tried to take turn");
//     };
//     let targets: Vec<Point> = targets(map, &attacker_kind);
//     if targets.is_empty() {
//         return false;
//     }
//     let already_in_range = pos
//         .neighbours(rows, cols)
//         .iter()
//         .any(|n| targets.contains(n));
//     if already_in_range {
//     } else {
//         let in_range = in_range(&targets, map);
//         if in_range.is_empty() {
//             return true;
//         }
//         let new_pos = pos_after_move(pos, in_range, map);
//         debug_assert!(
//             matches!(map.get(&new_pos), Some(Tile::Open)),
//             "Expected destination {:?} to be open, but found {:?}",
//             new_pos,
//             map.get(&new_pos)
//         );
//         let unit_tile = std::mem::replace(map.get_mut(&pos).unwrap(), Tile::Open);
//         *map.get_mut(&new_pos).unwrap() = unit_tile;
//     }
//     true
// }
//
// return new position if unit moved, else returns None
fn try_move(id: u32, map: &HashMap<Point, Tile>, units: &mut HashMap<u32, Mob>) -> Option<Point> {
    let rows = map.keys().map(|p| p.row).max().unwrap() as usize + 1;
    let cols = map.keys().map(|p| p.col).max().unwrap() as usize + 1;

    let unit = units.get(&id).unwrap();
    println!("Try moving unit {:?}", unit);

    let unit_ids: Vec<_> = units.keys().copied().collect();
    let alive_ids = alive_units(&unit_ids, units);
    let target_ids = targets(&unit.kind, &alive_ids, units);

    // If the unit is already in range of a target,
    // it does not move
    let already_in_range = unit
        .pos
        .neighbours(rows, cols)
        .iter()
        .any(|n| {
            target_ids
                .iter()
                .map(|id| units.get(id).unwrap().pos)
                .contains(n)
        });
    if already_in_range {
        return None;
    }

    let in_range = in_range(&target_ids, units, map);
    // nothing in range
    if in_range.is_empty() {
        return None;
    }
    let pos_after = pos_after_move(unit.pos, in_range, map);

    // do the actual move
    units
        .entry(id)
        .and_modify(|unit| unit.pos = pos_after);
    Some(pos_after)
}

// return new position for point after potential move
fn pos_after_move(pos: Point, in_range: Vec<Point>, map: &HashMap<Point, Tile>) -> Point {
    let (_, first) = chosen_and_first(pos, in_range, map);
    first
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

/// the sum of the hit points of all remaining units
fn sum_hp(units: &Vec<Mob>) -> u32 {
    units.iter().map(|unit| unit.hp).sum()
}

fn make_2d_vec(map: &HashMap<Point, Tile>, units: &HashMap<u32, Mob>) -> Vec<Vec<char>> {
    let max_row = map.keys().map(|p| p.row).max().unwrap();
    let max_col = map.keys().map(|p| p.col).max().unwrap();
    let mut res = Vec::new();
    // make map
    for row in 0..=max_row {
        let mut line = Vec::new();
        for col in 0..=max_col {
            let point = Point { row, col };
            let c = match &map[&point] {
                Tile::Wall => '#',
                Tile::Open => '.',
            };
            line.push(c);
        }
        res.push(line);
    }
    // insert units
    for unit in units.values() {
        let c = if unit.kind == Kind::Goblin { 'G' } else { 'E' };
        let Point { row, col } = unit.pos;
        res[row as usize][col as usize] = c;
    }
    res
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut id = 0;
        let mut map = HashMap::new();
        let mut units = HashMap::new();
        for (row_idx, line) in input.lines().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                let point = Point {
                    row: row_idx as u32,
                    col: col_idx as u32,
                };
                let tile = match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Open,
                    'G' | 'E' => {
                        id += 1;
                        let kind = if c == 'G' { Kind::Goblin } else { Kind::Elf };
                        units.insert(
                            id,
                            Mob {
                                id,
                                pos: point,
                                kind,
                                hp: 200,
                            },
                        );
                        Tile::Open
                    }
                    _ => panic!("Invalid input"),
                };
                map.insert(point, tile);
            }
        }
        Ok(Self { map, units })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        // let mut map = self.map.clone();
        // let mut round_num = 0;
        // loop {
        //     if round_num == 10 {
        //         break;
        //     }
        //     round(&mut map);
        //     round_num += 1;
        // }
        // Ok(round_num * sum_hp(&map))
        Ok(1)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(2)
    }
}

fn vec2d_to_string(vec_2d: Vec<Vec<char>>) -> String {
    use itertools::Itertools;
    vec_2d
        .iter()
        .map(|row| row.iter().collect::<String>())
        .join("\n")
}

#[cfg(test)]
mod test {
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
        let mut vec_2d = make_2d_vec(&data.map, &data.units);

        let unit_ids: Vec<_> = data.units.keys().copied().collect();
        let alive_ids = alive_units(&unit_ids, &data.units);
        let order = reading_order(&alive_ids, &data.units);

        for (count, id) in order.iter().enumerate() {
            let mob = &data.units[id];
            vec_2d[mob.pos.row as usize][mob.pos.col as usize] = ((count + 1) as u8 + b'0') as char;
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
        let mut vec_2d = make_2d_vec(&data.map, &data.units);

        let unit_ids: Vec<_> = data.units.keys().copied().collect();
        let targets = targets(&Kind::Elf, &unit_ids, &data.units);
        let in_range = in_range(&targets, &data.units, &data.map);

        for point in in_range {
            vec_2d[point.row as usize][point.col as usize] = '?';
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
        let mut vec_2d = make_2d_vec(&data.map, &data.units);

        let elf_pos = Point { row: 1, col: 1 };
        let unit_ids: Vec<_> = data.units.keys().copied().collect();
        let targets = targets(&Kind::Elf, &unit_ids, &data.units);
        let in_range = in_range(&targets, &data.units, &data.map);
        let nearest: Vec<_> = nearest_with_firsts(elf_pos, in_range, &data.map)
            .into_iter()
            .map(|(p, _)| p)
            .collect();

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
        let mut vec_2d = make_2d_vec(&data.map, &data.units);

        let elf_pos = Point { row: 1, col: 1 };
        let unit_ids: Vec<_> = data.units.keys().copied().collect();
        let targets = targets(&Kind::Elf, &unit_ids, &data.units);
        let in_range = in_range(&targets, &data.units, &data.map);
        let (chosen, _) = chosen_and_first(elf_pos, in_range, &data.map);

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
        let mut vec_2d = make_2d_vec(&data.map, &data.units);

        let unit_ids: Vec<_> = data.units.keys().copied().collect();
        let targets = targets(&Kind::Elf, &unit_ids, &data.units);
        let in_range = in_range(&targets, &data.units, &data.map);

        for point in in_range {
            vec_2d[point.row as usize][point.col as usize] = '?';
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
        let mut vec_2d = make_2d_vec(&data.map, &data.units);

        let elf_pos = Point { row: 1, col: 2 };
        let unit_ids: Vec<_> = data.units.keys().copied().collect();
        let targets = targets(&Kind::Elf, &unit_ids, &data.units);
        let in_range = in_range(&targets, &data.units, &data.map);
        let nearest: Vec<_> = nearest_with_firsts(elf_pos, in_range, &data.map)
            .into_iter()
            .map(|(p, _)| p)
            .collect();

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
        let mut vec_2d = make_2d_vec(&data.map, &data.units);

        let elf_pos = Point { row: 1, col: 2 };
        let unit_ids: Vec<_> = data.units.keys().copied().collect();
        let targets = targets(&Kind::Elf, &unit_ids, &data.units);
        let in_range = in_range(&targets, &data.units, &data.map);
        let (chosen, _) = chosen_and_first(elf_pos, in_range, &data.map);

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
    fn correct_step() {
        let input = "#######
#.E...#
#.....#
#...G.#
#######";
        let data = Data::try_new(input).unwrap();
        let mut vec_2d = make_2d_vec(&data.map, &data.units);

        let elf_pos = Point { row: 1, col: 2 };
        let unit_ids: Vec<_> = data.units.keys().copied().collect();
        let targets = targets(&Kind::Elf, &unit_ids, &data.units);
        let in_range = in_range(&targets, &data.units, &data.map);
        let (_, first) = chosen_and_first(elf_pos, in_range, &data.map);

        vec_2d[elf_pos.row as usize][elf_pos.col as usize] = '.';
        vec_2d[first.row as usize][first.col as usize] = 'E';
        let result = vec2d_to_string(vec_2d);
        assert_eq!(
            result,
            "#######
#..E..#
#.....#
#...G.#
#######"
        );
    }

    #[test]
    fn longer_movement() {
        let input = "#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########";
        let data = Data::try_new(input).unwrap();
        let mut units = data.units.clone();
        let mut movement = false;
        let unit_ids: Vec<_> = data.units.keys().copied().collect();
        let mut result = String::new();

        result.push_str("Initially:");
        result.push('\n');
        result.push_str(&vec2d_to_string(make_2d_vec(&data.map, &units)));

        for round in 1.. {
            let alive_ids = alive_units(&unit_ids, &units);
            let order = reading_order(&alive_ids, &units);

            for id in &order {
                if try_move(*id, &data.map, &mut units).is_none() {
                    continue;
                }
                movement = true;
            }

            if !movement {
                break;
            }

            result.push('\n');
            result.push('\n');
            result.push_str(&format!(
                "After {round} {}:",
                if round == 1 { "round" } else { "rounds" }
            ));
            result.push('\n');
            result.push_str(&vec2d_to_string(make_2d_vec(&data.map, &units)));

            movement = false;
        }
        let expected = "Initially:
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########

After 1 round:
#########
#.G...G.#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########

After 2 rounds:
#########
#..G.G..#
#...G...#
#.G.E.G.#
#.......#
#G..G..G#
#.......#
#.......#
#########

After 3 rounds:
#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########";
        assert_eq!(result, expected);
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
