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

fn open_around(
    unit_ids: &[u32],
    units: &HashMap<u32, Mob>,
    map: &HashMap<Point, Tile>,
) -> Vec<Point> {
    let rows = map.keys().map(|p| p.row).max().unwrap() as usize + 1;
    let cols = map.keys().map(|p| p.col).max().unwrap() as usize + 1;
    unit_ids
        .iter()
        .flat_map(|id| {
            let target = &units[id];
            target.pos.neighbours(rows, cols)
        })
        .filter(|p| {
            let open = matches!(map[p], Tile::Open);
            let occupied = units
                .values()
                .any(|unit| unit.hp > 0 && unit.pos == *p);
            open && !occupied
        })
        .collect()
}

fn nearest_with_firsts(
    from: Point,
    in_range: Vec<Point>,
    map: &HashMap<Point, Tile>,
    units: &HashMap<u32, Mob>,
) -> Vec<(Point, Vec<Point>)> {
    in_range
        .into_iter()
        .map(|to| (to, shortest(from, to, map, units)))
        .min_set_by_key(|(_, (cost, _))| *cost)
        .into_iter()
        .map(|(to, (_, firsts))| (to, firsts))
        .collect()
}

fn chosen_and_first(
    from: Point,
    in_range: Vec<Point>,
    map: &HashMap<Point, Tile>,
    units: &HashMap<u32, Mob>,
) -> (Point, Option<Point>) {
    let mut nearest_with_fists = nearest_with_firsts(from, in_range, map, units);
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
    dbg!(&chosen, &firsts.len());
    // sort in reading order of firsts
    firsts.sort_unstable_by(|a, b| {
        a.row
            .cmp(&b.row)
            .then_with(|| a.col.cmp(&b.col))
    });
    let first = firsts.into_iter().next();
    (chosen, first)
}

fn round(map: &HashMap<Point, Tile>, units: &mut HashMap<u32, Mob>) {
    // let rows = map.keys().map(|p| p.row).max().unwrap() as usize + 1;
    // let cols = map.keys().map(|p| p.col).max().unwrap() as usize + 1;

    let unit_ids: Vec<_> = units.keys().copied().collect();
    let alive_ids = alive_units(&unit_ids, units);
    let order = reading_order(&alive_ids, units);

    for &id in &order {
        // let unit = units.get(&id).unwrap();
        // let alive_ids = alive_units(&unit_ids, units);
        // let target_ids = targets(&unit.kind, &alive_ids, units);
        // let already_in_range = unit
        //     .pos
        //     .neighbours(rows, cols)
        //     .iter()
        //     .any(|n| {
        //         target_ids
        //             .iter()
        //             .map(|id| units.get(id).unwrap().pos)
        //             .contains(n)
        //     });
        // if already_in_range {
        //     attack(id, map, units);
        //     continue;
        // }
        // if try_move(id, map, units).is_some() {
        //     attack(id, map, units);
        // }
        try_move(id, map, units);
        attack(id, map, units);
    }
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
    // println!("Try moving unit {:?}", unit);

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

    let open_around = open_around(&target_ids, units, map);
    if open_around.is_empty() {
        return None;
    }
    let pos_after = pos_after_move(unit.pos, open_around, map, units);

    units
        .entry(id)
        .and_modify(|unit| unit.pos = pos_after);
    Some(pos_after)
}

fn adjacent(
    id: u32,
    target_ids: &[u32],
    map: &HashMap<Point, Tile>,
    units: &HashMap<u32, Mob>,
) -> Vec<u32> {
    let rows = map.keys().map(|p| p.row).max().unwrap() as usize + 1;
    let cols = map.keys().map(|p| p.col).max().unwrap() as usize + 1;
    let unit = &units[&id];

    target_ids
        .iter()
        .filter(|target_id| {
            let target = &units[*target_id];
            target
                .pos
                .neighbours(rows, cols)
                .contains(&unit.pos)
        })
        .copied()
        .collect()
}

fn fewest_hitpoints(ids: &[u32], units: &HashMap<u32, Mob>) -> Vec<u32> {
    ids.iter()
        .copied()
        .min_set_by_key(|id| &units[id].hp)
}

// returns true if target attacked, false if turn ended
fn attack(id: u32, map: &HashMap<Point, Tile>, units: &mut HashMap<u32, Mob>) -> bool {
    let unit = units.get(&id).unwrap();
    // the unit first determines all of the targets
    // that are in range of it by being immediately adjacent to it.
    let unit_ids: Vec<_> = units.keys().copied().collect();
    let alive_ids = alive_units(&unit_ids, units);
    let target_ids = targets(&unit.kind, &alive_ids, units);
    let adjacent = adjacent(id, &target_ids, map, units);
    // If there are no such targets, the unit ends its turn.
    if adjacent.is_empty() {
        return false;
    }
    // Otherwise, the adjacent target with the fewest hit points is selected;
    let fewest_hp = fewest_hitpoints(&adjacent, units);
    // in a tie, the adjacent target with the fewest hit points which is first in reading order is selected.
    let selected_id = reading_order(&fewest_hp, units)[0];
    // The unit deals damage equal to its attack power to the selected target,
    // reducing its hit points by that amount.
    // If this reduces its hit points to 0 or fewer,
    // the selected target dies:
    // its square becomes . and it takes no further turns.
    units
        .entry(selected_id)
        .and_modify(|unit| unit.hp = unit.hp.saturating_sub(3));
    true
}

// return new position for point after potential move
fn pos_after_move(
    pos: Point,
    in_range: Vec<Point>,
    map: &HashMap<Point, Tile>,
    units: &HashMap<u32, Mob>,
) -> Point {
    let (_, first) = chosen_and_first(pos, in_range, map, units);
    if let Some(new_pos) = first {
        new_pos
    } else {
        pos
    }
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
fn shortest(
    from: Point,
    to: Point,
    map: &HashMap<Point, Tile>,
    units: &HashMap<u32, Mob>,
) -> (u32, Vec<Point>) {
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
            .filter(|p| {
                let open = matches!(&map[p], Tile::Open);
                let occupied = units
                    .values()
                    .any(|unit| unit.hp > 0 && unit.pos == *p);
                open && !occupied
            })
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
        if unit.hp > 0 {
            res[row as usize][col as usize] = c;
        }
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
        //        HP:            HP:
        // G....  9       G....  9
        // ..G..  4       ..G..  4
        // ..EG.  2  -->  ..E..
        // ..G..  2       ..G..  2
        // ...G.  1       ...G.  1
        //         let input = "G....
        // ..G..
        // ..EG.
        // ..G..
        // ...G.";
        //         let mut data = Data::try_new(input).unwrap();
        // // set hp of each goblin
        // let unit_ids: Vec<_> = data.units.keys().copied().collect();
        // let goblins = targets(&Kind::Elf, &unit_ids, &data.units);
        // for (idx, id) in reading_order(&goblins, &data.units)
        //     .iter()
        //     .enumerate()
        // {
        //     let unit = data.units.get_mut(id).unwrap();
        //     let hp = match idx {
        //         0 => 9,
        //         1 => 4,
        //         2 => 2,
        //         3 => 2,
        //         4 => 1,
        //         _ => 200,
        //     };
        //     unit.hp = hp;
        // }
        // let elf_id = data
        //     .units
        //     .values()
        //     .find_map(|unit| match unit.kind {
        //         Kind::Goblin => None,
        //         Kind::Elf => Some(unit.id),
        //     })
        //     .unwrap();
        // attack(elf_id, &data.map, &mut data.units);
        // let mut vec_2d = make_2d_vec(&data.map, &data.units);
        // println!("{}", vec2d_to_string(vec_2d));
        // dbg!(data.units);
        // #######       #######
        // #G..#E#       #...#E#   E(200)
        // #E#E.E#       #E#...#   E(197)
        // #G.##.#  -->  #.E##.#   E(185)
        // #...#E#       #E..#E#   E(200), E(200)
        // #...E.#       #.....#
        // #######       #######
        //
        // Combat ends after 37 full rounds
        // Elves win with 982 total hit points left
        // Outcome: 37 * 982 = 36334
        let input = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        let mut data = Data::try_new(input).unwrap();
        for num in 1.. {
            dbg!(num);
            round(&data.map, &mut data.units);
            if [1, 2, 23, 24, 25, 26, 27, 28, 47].contains(&num) {
                let vec_2d = make_2d_vec(&data.map, &data.units);
                println!("Round {num}:");
                println!("{}", vec2d_to_string(vec_2d));
                println!();
            }
            if num == 47 {
                break;
            }
        }
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
        let in_range = open_around(&targets, &data.units, &data.map);

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
        let in_range = open_around(&targets, &data.units, &data.map);
        let nearest: Vec<_> = nearest_with_firsts(elf_pos, in_range, &data.map, &data.units)
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
        let in_range = open_around(&targets, &data.units, &data.map);
        let (chosen, _) = chosen_and_first(elf_pos, in_range, &data.map, &data.units);

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
        let in_range = open_around(&targets, &data.units, &data.map);

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
        let in_range = open_around(&targets, &data.units, &data.map);
        let nearest: Vec<_> = nearest_with_firsts(elf_pos, in_range, &data.map, &data.units)
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
        let in_range = open_around(&targets, &data.units, &data.map);
        let (chosen, _) = chosen_and_first(elf_pos, in_range, &data.map, &data.units);

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
        let in_range = open_around(&targets, &data.units, &data.map);
        let (_, first) = chosen_and_first(elf_pos, in_range, &data.map, &data.units);

        vec_2d[elf_pos.row as usize][elf_pos.col as usize] = '.';
        vec_2d[first.unwrap().row as usize][first.unwrap().col as usize] = 'E';
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
