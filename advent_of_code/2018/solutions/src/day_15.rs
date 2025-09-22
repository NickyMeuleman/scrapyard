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
        // ensure reading order
        res.sort_unstable_by(|a, b| {
            a.row
                .cmp(&b.row)
                .then(a.col.cmp(&b.col))
        });
        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

fn sum_hp(units: &[&Mob]) -> u32 {
    units.iter().map(|unit| unit.hp).sum()
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
        // #######       #######
        // #E.G#.#       #G.G#.#   G(200), G(98)
        // #.#G..#       #.#G..#   G(200)
        // #G.#.G#  -->  #..#..#
        // #G..#.#       #...#G#   G(95)
        // #...E.#       #...G.#   G(200)
        // #######       #######
        //
        // Combat ends after 35 full rounds
        // Goblins win with 793 total hit points left
        // Outcome: 35 * 793 = 27755
        // let input = "#######
        // #E.G#.#
        // #.#G..#
        // #G.#.G#
        // #G..#.#
        // #...E.#
        // #######";
        //
        // #######       #######
        // #E.G#.#       #G.G#.#   G(200), G(98)
        // #.#G..#       #.#G..#   G(200)
        // #G.#.G#  -->  #..#..#
        // #G..#.#       #...#G#   G(95)
        // #...E.#       #...G.#   G(200)
        // #######       #######
        //
        // Combat ends after 35 full rounds
        // Goblins win with 793 total hit points left
        // Outcome: 35 * 793 = 27755
        //
        //         let input = "#######
        // #E.G#.#
        // #.#G..#
        // #G.#.G#
        // #G..#.#
        // #...E.#
        // #######";
        //
        // #######       #######
        // #.E...#       #.....#
        // #.#..G#       #.#G..#   G(200)
        // #.###.#  -->  #.###.#
        // #E#G#G#       #.#.#.#
        // #...#G#       #G.G#G#   G(98), G(38), G(200)
        // #######       #######
        //
        // Combat ends after 54 full rounds
        // Goblins win with 536 total hit points left
        // Outcome: 54 * 536 = 28944
        //         let input = "#######
        // #.E...#
        // #.#..G#
        // #.###.#
        // #E#G#G#
        // #...#G#
        // #######";

        // #########       #########
        // #G......#       #.G.....#   G(137)
        // #.E.#...#       #G.G#...#   G(200), G(200)
        // #..##..G#       #.G##...#   G(200)
        // #...##..#  -->  #...##..#
        // #...#...#       #.G.#...#   G(200)
        // #.G...G.#       #.......#
        // #.....G.#       #.......#
        // #########       #########
        //
        // Combat ends after 20 full rounds
        // Goblins win with 937 total hit points left
        // Outcome: 20 * 937 = 18740
//         let input = "#########
// #G......#
// #.E.#...#
// #..##..G#
// #...##..#
// #...#...#
// #.G...G.#
// #.....G.#
// #########";
//         let data = Data::try_new(input).unwrap();
        let map = &self.map;
        let mut units = self.units.clone();
        let rows = map.keys().map(|p| p.row).max().unwrap() as usize + 1;
        let cols = map.keys().map(|p| p.col).max().unwrap() as usize + 1;

        let mut round = 1;

        'combat: loop {
            let alive: Vec<u32> = units
                .values()
                .filter_map(|unit| (unit.hp > 0).then_some(unit.id))
                .collect();

            // ties are broken in reading order.
            // For instance, the order in which units take their turns within a round
            // is the reading order of their starting positions in that round,
            // regardless of the type of unit or whether other units have moved after the round started.
            let order: Vec<u32> = alive
                .into_iter()
                .sorted_by(|a, b| {
                    let a = &units[a];
                    let b = &units[b];
                    a.pos
                        .row
                        .cmp(&b.pos.row)
                        .then_with(|| a.pos.col.cmp(&b.pos.col))
                })
                .collect();

            for id in &order {
                let unit = &units[id];
                // skip if unit is dead
                if unit.hp == 0 {
                    continue;
                }
                let attacker_kind = unit.kind;

                // Each unit begins its turn by identifying all possible targets
                let targets: Vec<u32> = units
                    .values()
                    .filter_map(|unit| {
                        (unit.hp > 0 && unit.kind != attacker_kind).then_some(unit.id)
                    })
                    .collect();
                // If no targets remain, combat ends.
                if targets.is_empty() {
                    break 'combat;
                }
                // the unit identifies all of the open squares (.) that are in range of each target;
                // these are the squares which are adjacent to any target
                // and which aren't already occupied by a wall or another unit.
                let in_range: Vec<Point> = targets
                    .iter()
                    .flat_map(|target_id| {
                        let target = &units[target_id];
                        let neighbours = target.pos.neighbours(rows, cols);
                        neighbours.into_iter().filter(|point| {
                            let open = matches!(map[point], Tile::Open);
                            let occupied = units
                                .values()
                                .any(|unit| unit.hp > 0 && unit.pos == *point);
                            open && !occupied
                        })
                    })
                    .collect();
                // the unit might already be in range of a target.
                let already_in_range = unit
                    .pos
                    .neighbours(rows, cols)
                    .iter()
                    .any(|&neighbour| {
                        targets
                            .iter()
                            .any(|target_id| units[target_id].pos == neighbour)
                    });
                // If the unit is not already in range of a target,
                // and there are no open squares which are in range of a target, the unit ends its turn.
                if !already_in_range && in_range.is_empty() {
                    continue;
                }
                // If the unit is already in range of a target, it does not move,
                // but continues its turn with an attack.
                // Otherwise, since it is not in range of a target, it moves.
                if !already_in_range {
                    // To move, the unit first considers the squares that are in range
                    // and determines which of those squares it could reach in the fewest steps.
                    // A step is a single movement to any adjacent (immediately up, down, left, or right) open (.) square.
                    // Units cannot move into walls or other units.
                    // The unit does this while considering the current positions of units
                    // and does not do any prediction about where units will be later.
                    // If the unit cannot reach (find an open path to) any of the squares that are in range,
                    // it ends its turn.
                    // If multiple squares are in range and tied for being reachable in the fewest steps,
                    // the square which is first in reading order is chosen.
                    let mut q = VecDeque::new();
                    let mut visited = HashSet::new();
                    let mut first_step_map: HashMap<Point, Point> = HashMap::new();
                    let mut found_targets: Vec<Point> = Vec::new();
                    let mut min_dist = None;

                    q.push_back((unit.pos, 0));
                    visited.insert(unit.pos);

                    while let Some((pos, dist)) = q.pop_front() {
                        // Stop if we've exceeded the min dist
                        if let Some(num) = min_dist {
                            if dist > num {
                                break;
                            }
                        }

                        // save pos if it's an in_range point
                        if in_range.contains(&pos) {
                            if min_dist.is_none() {
                                min_dist = Some(dist);
                            }
                            found_targets.push(pos);
                            // don't expand from target points
                            continue;
                        }

                        // explore in reading order
                        let neighbours_in_order = pos
                            .neighbours(rows, cols)
                            .into_iter()
                            .filter(|p| {
                                let open = matches!(map[p], Tile::Open);
                                let occupied = units
                                    .values()
                                    .any(|unit| unit.hp > 0 && unit.pos == *p);
                                open && !occupied
                            });
                        for n in neighbours_in_order {
                            if !visited.insert(n) {
                                // skip already visited neighbours
                                continue;
                            }
                            if pos == unit.pos {
                                first_step_map.insert(n, n);
                            } else {
                                let first = first_step_map[&pos];
                                first_step_map.insert(n, first);
                            }
                            q.push_back((n, dist + 1));
                        }
                    }

                    if !found_targets.is_empty() {
                        // Choose the target square in reading order
                        found_targets.sort_by(|a, b| {
                            a.row
                                .cmp(&b.row)
                                .then(a.col.cmp(&b.col))
                        });
                        let chosen_target = found_targets[0];

                        // The unit then takes a single step toward the chosen square along the shortest path to that square.
                        // If multiple steps would put the unit equally closer to its destination,
                        // the unit chooses the step which is first in reading order.
                        // (This requires knowing when there is more than one shortest path
                        // so that you can consider the first step of each such path.)
                        //
                        // Among all paths to the chosen target, pick the first step in reading order
                        let mut first_steps: Vec<Point> = first_step_map
                            .iter()
                            .filter_map(|(pos, &first)| {
                                if *pos == chosen_target {
                                    Some(first)
                                } else {
                                    None
                                }
                            })
                            .collect();
                        first_steps.sort_by(|a, b| {
                            a.row
                                .cmp(&b.row)
                                .then(a.col.cmp(&b.col))
                        });

                        if let Some(step) = first_steps.first() {
                            // Move the unit one step
                            units.get_mut(id).unwrap().pos = *step;
                        }
                    }
                }
                // After moving (or if the unit began its turn in range of a target), the unit attacks.
                // To attack, the unit first determines all of the targets that are in range of it
                // by being immediately adjacent to it.
                let attacker_pos = units[id].pos;
                let mut adjacent_targets: Vec<u32> = targets
                    .iter()
                    .filter_map(|target_id| {
                        let target = &units[target_id];
                        attacker_pos
                            .neighbours(rows, cols)
                            .contains(&target.pos)
                            .then_some(*target_id)
                    })
                    .collect();
                // If there are no such targets, the unit ends its turn.
                if adjacent_targets.is_empty() {
                    continue;
                }
                // Otherwise, the adjacent target with the fewest hit points is selected;
                // in a tie, the adjacent target with the fewest hit points which is first in reading order is selected.
                adjacent_targets.sort_by(|a, b| {
                    let ua = &units[a];
                    let ub = &units[b];
                    ua.hp
                        .cmp(&ub.hp)
                        .then_with(|| ua.pos.row.cmp(&ub.pos.row))
                        .then_with(|| ua.pos.col.cmp(&ub.pos.col))
                });
                let target_id = adjacent_targets[0];
                // The unit deals damage equal to its attack power to the selected target,
                // reducing its hit points by that amount.
                // If this reduces its hit points to 0 or fewer,
                // the selected target dies: its square becomes . and it takes no further turns.
                units
                    .entry(target_id)
                    .and_modify(|target_unit| target_unit.hp = target_unit.hp.saturating_sub(3));
            }

            round += 1;
        }
        round -= 1;
        let (elves, goblins): (Vec<_>, Vec<_>) = units
            .values()
            .filter(|unit| unit.hp > 0)
            .partition(|unit| unit.kind == Kind::Elf);
        let elf_hp = sum_hp(&elves);
        let goblin_hp = sum_hp(&goblins);
        dbg!(round, elf_hp, goblin_hp, elf_hp * round, goblin_hp * round);

        Ok(1)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
