use crate::{AoCData, AoCResult};
use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Kind {
    Goblin,
    Elf,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Mob {
    pos: Point,
    kind: Kind,
    hp: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
}

#[derive(Debug, Clone, Copy)]
struct Neighbours {
    pts: [Point; 4],
    len: u8,
}
impl Neighbours {
    fn iter(&self) -> impl Iterator<Item = Point> + '_ {
        self.pts[..self.len as usize]
            .iter()
            .copied()
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    map: Vec<Tile>,              // length = rows * cols
    neighbours: Vec<Neighbours>, // precomputed neighbours for each cell
    rows: usize,
    cols: usize,
    units: Vec<Mob>,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let lines: Vec<&str> = input.lines().collect();
        let rows = lines.len();
        let cols = lines
            .iter()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(0);

        let mut map = Vec::with_capacity(rows * cols);
        let mut units = Vec::new();

        for (r, line) in lines.iter().enumerate() {
            // pad short lines with walls to keep rectangular shape
            let mut chars: Vec<char> = line.chars().collect();
            if chars.len() < cols {
                chars.resize(cols, '#');
            }
            for (c, ch) in chars.into_iter().enumerate() {
                let p = Point { row: r, col: c };
                match ch {
                    '#' => map.push(Tile::Wall),
                    '.' => map.push(Tile::Open),
                    'G' => {
                        units.push(Mob {
                            pos: p,
                            kind: Kind::Goblin,
                            hp: 200,
                        });
                        map.push(Tile::Open);
                    }
                    'E' => {
                        units.push(Mob {
                            pos: p,
                            kind: Kind::Elf,
                            hp: 200,
                        });
                        map.push(Tile::Open);
                    }
                    _ => panic!("Invalid char"),
                }
            }
        }

        // Precompute neighbours in reading order
        let mut neighbours = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in 0..cols {
                let mut n = Neighbours {
                    pts: [Point { row: 0, col: 0 }; 4],
                    len: 0,
                };
                if r > 0 {
                    n.pts[n.len as usize] = Point { row: r - 1, col: c };
                    n.len += 1;
                }
                if c > 0 {
                    n.pts[n.len as usize] = Point { row: r, col: c - 1 };
                    n.len += 1;
                }
                if c + 1 < cols {
                    n.pts[n.len as usize] = Point { row: r, col: c + 1 };
                    n.len += 1;
                }
                if r + 1 < rows {
                    n.pts[n.len as usize] = Point { row: r + 1, col: c };
                    n.len += 1;
                }
                neighbours.push(n);
            }
        }

        Ok(Self {
            map,
            neighbours,
            rows,
            cols,
            units,
        })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut units = self.units.clone();
        let (_, outcome) = sim_combat(
            &self.map,
            &self.neighbours,
            self.rows,
            self.cols,
            &mut units,
            3,
            false,
        );
        Ok(outcome)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut buf = Vec::with_capacity(self.units.len());
        for ap in 4.. {
            buf.clear();
            buf.extend(self.units.iter().cloned());
            let (flawless, outcome) = sim_combat(
                &self.map,
                &self.neighbours,
                self.rows,
                self.cols,
                &mut buf,
                ap,
                true,
            );
            if flawless {
                return Ok(outcome);
            }
        }
        unreachable!()
    }
}

#[inline]
fn idx(p: Point, cols: usize) -> usize {
    p.row as usize * cols + p.col as usize
}

// fn sim_combat(
//     map: &[Tile],
//     rows: usize,
//     cols: usize,
//     units: &mut [Mob],
//     elf_attack_power: u32,
//     stop_on_elf_death: bool,
// ) -> (bool, u32) {
//     let mut occupied = vec![None; rows * cols];
//     // reusable vectors for shortest-path algorithm
//     let mut visited = vec![false; rows * cols];
//     let mut first_step = vec![None; rows * cols];
//     let mut round = 0;
//
//     loop {
//         // ties are broken in reading order.
//         units.sort_by(|a, b| {
//             a.pos
//                 .row
//                 .cmp(&b.pos.row)
//                 .then(a.pos.col.cmp(&b.pos.col))
//         });
//
//         // rebuild occupancy according to order
//         occupied.fill(None);
//         for (i, u) in units
//             .iter()
//             .enumerate()
//             .filter(|(_, unit)| unit.hp > 0)
//         {
//             occupied[idx(u.pos, cols)] = Some(i);
//         }
//
//         for i in 0..units.len() {
//             if units[i].hp == 0 {
//                 continue;
//             }
//             let attacker_kind = units[i].kind;
//
//             // Each unit begins its turn by identifying all possible targets
//             let targets: Vec<usize> = units
//                 .iter()
//                 .enumerate()
//                 .filter_map(|(idx, u)| (u.hp > 0 && u.kind != attacker_kind).then_some(idx))
//                 .collect();
//             // If no targets remain, combat ends.
//             if targets.is_empty() {
//                 let hp_sum: u32 = units
//                     .iter()
//                     .filter(|u| u.hp > 0)
//                     .map(|u| u.hp)
//                     .sum();
//                 let no_elf_died = units
//                     .iter()
//                     .all(|u| u.kind != Kind::Elf || u.hp > 0);
//                 return (no_elf_died, round * hp_sum);
//             }
//
//             // the unit identifies all of the open squares (.) that are in range of each target
//             let in_range: Vec<_> = targets
//                 .iter()
//                 .flat_map(|tid| {
//                     units[*tid]
//                         .pos
//                         .neighbours(rows, cols)
//                         .filter(|&n| {
//                             let tidx = idx(n, cols);
//                             matches!((&map[tidx], occupied[tidx]), (Tile::Open, None))
//                         })
//                 })
//                 .collect();
//
//             // the unit might already be in range of a target.
//             let already_in_range = units[i]
//                 .pos
//                 .neighbours(rows, cols)
//                 .any(|n| {
//                     targets
//                         .iter()
//                         .any(|&tid| units[tid].pos == n)
//                 });
//
//             if !already_in_range && in_range.is_empty() {
//                 continue;
//             }
//
//             // movement
//             if !already_in_range {
//                 visited.fill(false);
//                 first_step.fill(None);
//                 let mut q = VecDeque::new();
//                 let mut found_targets = Vec::new();
//                 let mut min_dist = None;
//
//                 q.push_back((units[i].pos, 0));
//                 visited[idx(units[i].pos, cols)] = true;
//
//                 while let Some((pos, dist)) = q.pop_front() {
//                     if min_dist.is_some_and(|md| dist > md) {
//                         break;
//                     }
//                     if in_range.contains(&pos) {
//                         min_dist.get_or_insert(dist);
//                         found_targets.push(pos);
//                         continue;
//                     }
//                     for n in pos.neighbours(rows, cols) {
//                         let ni = idx(n, cols);
//                         if !visited[ni] && map[ni] == Tile::Open && occupied[ni].is_none() {
//                             visited[ni] = true;
//                             first_step[ni] = if pos == units[i].pos {
//                                 Some(n)
//                             } else {
//                                 first_step[idx(pos, cols)]
//                             };
//                             q.push_back((n, dist + 1));
//                         }
//                     }
//                 }
//
//                 if let Some(&target) = found_targets.iter().min_by(|a, b| {
//                     a.row
//                         .cmp(&b.row)
//                         .then(a.col.cmp(&b.col))
//                 }) {
//                     if let Some(step) = first_step[idx(target, cols)] {
//                         // Move the unit one step
//                         occupied[idx(units[i].pos, cols)] = None;
//                         units[i].pos = step;
//                         occupied[idx(step, cols)] = Some(i);
//                     }
//                 }
//             }
//
//             // attack
//             let attacker_pos = units[i].pos;
//             let mut adjacent_targets: Vec<usize> = targets
//                 .into_iter()
//                 .filter(|&tid| {
//                     attacker_pos
//                         .neighbours(rows, cols)
//                         .any(|n| units[tid].pos == n)
//                 })
//                 .collect();
//             if adjacent_targets.is_empty() {
//                 continue;
//             }
//
//             adjacent_targets.sort_by(|&a, &b| {
//                 units[a]
//                     .hp
//                     .cmp(&units[b].hp)
//                     .then(units[a].pos.row.cmp(&units[b].pos.row))
//                     .then(units[a].pos.col.cmp(&units[b].pos.col))
//             });
//             let target_id = adjacent_targets[0];
//             let power = if units[i].kind == Kind::Elf {
//                 elf_attack_power
//             } else {
//                 3
//             };
//             if units[target_id].hp <= power {
//                 units[target_id].hp = 0;
//                 occupied[idx(units[target_id].pos, cols)] = None;
//                 if stop_on_elf_death && units[target_id].kind == Kind::Elf {
//                     return (false, 0);
//                 }
//             } else {
//                 units[target_id].hp -= power;
//             }
//         }
//         round += 1;
//     }
// }
fn sim_combat(
    map: &[Tile],
    neighbours: &[Neighbours], // precomputed neighbours for each cell
    rows: usize,
    cols: usize,
    units: &mut [Mob],
    elf_attack_power: u32,
    stop_on_elf_death: bool,
) -> (bool, u32) {
    let mut occupied = vec![None; rows * cols];
    let mut visited = vec![false; rows * cols];
    let mut first_step = vec![None; rows * cols];
    let mut in_range_mask = vec![false; rows * cols];
    let mut round = 0;

    loop {
        // reading order
        units.sort_by_key(|u| (u.pos.row, u.pos.col));

        // rebuild occupancy
        occupied.fill(None);
        for (i, u) in units
            .iter()
            .enumerate()
            .filter(|(_, u)| u.hp > 0)
        {
            occupied[idx(u.pos, cols)] = Some(i);
        }

        for i in 0..units.len() {
            if units[i].hp == 0 {
                continue;
            }
            let attacker_kind = units[i].kind;

            // mark in-range squares and check if any targets exist
            in_range_mask.fill(false);
            let mut has_target = false;
            let mut any_in_range = false;
            for target in units.iter() {
                if target.hp > 0 && target.kind != attacker_kind {
                    has_target = true;
                    for n in neighbours[idx(target.pos, cols)].iter() {
                        let ni = idx(n, cols);
                        if map[ni] == Tile::Open && occupied[ni].is_none() {
                            in_range_mask[ni] = true;
                            any_in_range = true;
                        }
                    }
                }
            }
            if !has_target {
                let hp_sum: u32 = units
                    .iter()
                    .filter(|u| u.hp > 0)
                    .map(|u| u.hp)
                    .sum();
                let no_elf_died = units
                    .iter()
                    .all(|u| u.kind != Kind::Elf || u.hp > 0);
                return (no_elf_died, round * hp_sum);
            }

            // already in range?
            let already_in_range = neighbours[idx(units[i].pos, cols)]
                .iter()
                .any(|n| {
                    occupied[idx(n, cols)]
                        .map(|tid| units[tid].kind != attacker_kind)
                        .unwrap_or(false)
                });
            if !already_in_range && !any_in_range {
                continue;
            }

            // movement: BFS to nearest in-range square, tie-break by reading order
            if !already_in_range {
                visited.fill(false);
                first_step.fill(None);
                let mut q = VecDeque::new();
                let mut min_dist: Option<u32> = None;
                let mut best_target: Option<Point> = None;

                q.push_back((units[i].pos, 0));
                visited[idx(units[i].pos, cols)] = true;

                while let Some((pos, dist)) = q.pop_front() {
                    if min_dist.is_some_and(|md| dist > md) {
                        break;
                    }

                    let pi = idx(pos, cols);
                    if in_range_mask[pi] {
                        match best_target {
                            None => {
                                best_target = Some(pos);
                                min_dist.get_or_insert(dist);
                            }
                            Some(bt) => {
                                if pos.row < bt.row || (pos.row == bt.row && pos.col < bt.col) {
                                    best_target = Some(pos);
                                }
                            }
                        }
                        continue;
                    }

                    for n in neighbours[pi].iter() {
                        let ni = idx(n, cols);
                        if !visited[ni] && map[ni] == Tile::Open && occupied[ni].is_none() {
                            visited[ni] = true;
                            first_step[ni] = if pos == units[i].pos {
                                Some(n)
                            } else {
                                first_step[pi]
                            };
                            q.push_back((n, dist + 1));
                        }
                    }
                }

                if let Some(target) = best_target {
                    if let Some(step) = first_step[idx(target, cols)] {
                        occupied[idx(units[i].pos, cols)] = None;
                        units[i].pos = step;
                        occupied[idx(step, cols)] = Some(i);
                    }
                }
            }

            // attack: pick adjacent enemy with lowest HP, tie by reading order
            let mut best: Option<(usize, u32, Point)> = None; // (idx, hp, pos)
            for n in neighbours[idx(units[i].pos, cols)].iter() {
                if let Some(tid) = occupied[idx(n, cols)] {
                    let t = &units[tid];
                    if t.hp > 0 && t.kind != attacker_kind {
                        match best {
                            None => best = Some((tid, t.hp, t.pos)),
                            Some((bid, bhp, bpos)) => {
                                if t.hp < bhp
                                    || (t.hp == bhp
                                        && (t.pos.row < bpos.row
                                            || (t.pos.row == bpos.row && t.pos.col < bpos.col)))
                                {
                                    best = Some((tid, t.hp, t.pos));
                                }
                            }
                        }
                    }
                }
            }
            if let Some((target_id, _, _)) = best {
                let power = if units[i].kind == Kind::Elf {
                    elf_attack_power
                } else {
                    3
                };
                if units[target_id].hp <= power {
                    units[target_id].hp = 0;
                    occupied[idx(units[target_id].pos, cols)] = None;
                    if stop_on_elf_death && units[target_id].kind == Kind::Elf {
                        return (false, 0);
                    }
                } else {
                    units[target_id].hp -= power;
                }
            }
        }
        round += 1;
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
