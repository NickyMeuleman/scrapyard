use crate::{AoCData, AoCResult};
use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    row: usize,
    col: usize,
}
impl Point {
    fn idx(&self, cols: usize) -> usize {
        self.row * cols + self.col
    }
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
    hp: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
}

#[derive(Debug, Clone)]
pub struct Data {
    map: Vec<Tile>,
    neighbours: Vec<Vec<Point>>, // precomputed neighbours for each cell
    rows: usize,
    cols: usize,
    units: Vec<Mob>,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let lines: Vec<&str> = input.lines().collect();
        let rows = lines.len();
        let cols = lines
            .first()
            .map(|l| l.chars().count())
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
                let mut n = Vec::with_capacity(4);
                if r > 0 {
                    n.push(Point { row: r - 1, col: c });
                }
                if c > 0 {
                    n.push(Point { row: r, col: c - 1 });
                }
                if c + 1 < cols {
                    n.push(Point { row: r, col: c + 1 });
                }
                if r + 1 < rows {
                    n.push(Point { row: r + 1, col: c });
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

// BFS to find the best target for a unit
// returns the first step of the path to that target.
fn find_best_move(
    start: Point,
    map: &[Tile],
    neighbours: &[Vec<Point>],
    cols: usize,
    occupied: &[Option<usize>],
    in_range: &[bool],
    visited: &mut [bool],
    found: &mut Vec<(Point, Point)>,
) -> Option<Point> {
    let mut q = VecDeque::new();
    visited.fill(false);
    found.clear();
    let mut min_dist = usize::MAX;

    // (pos, first, cost)
    q.push_back((start, None, 0));
    visited[start.idx(cols)] = true;

    while let Some((pos, first, dist)) = q.pop_front() {
        // stop the loop, do not explore paths that are longer than min_dist
        if dist > min_dist {
            break;
        }

        // Check if the current position is a valid target square.
        let pi = pos.idx(cols);
        if in_range[pi] {
            min_dist = dist;
            found.push((pos, first.unwrap_or(pos)));
            continue; // Do not explore from a target square.
        }

        // Explore neighbours in reading order to handle tie-breaking for paths.
        for &n in &neighbours[pi] {
            let ni = n.idx(cols);
            // Only explore open, unvisited, and unoccupied squares.
            if !visited[ni] && map[ni] == Tile::Open && occupied[ni].is_none() {
                visited[ni] = true;
                q.push_back((n, Some(first.unwrap_or(n)), dist + 1));
            }
        }
    }

    // return the first step (in reading order during the search) that belongs to the first target in reading order
    found
        .iter()
        .min_by_key(|(p, _)| (p.row, p.col))
        .map(|(_, first)| *first)
}

fn sim_combat(
    map: &[Tile],
    neighbours: &[Vec<Point>],
    rows: usize,
    cols: usize,
    units: &mut [Mob],
    elf_attack_power: usize,
    stop_on_elf_death: bool,
) -> (bool, usize) {
    let mut occupied = vec![None; rows * cols];
    // the in_range is a bool-mask with the same shape as the map, true means it's a valid destination for a
    // unit that has an adjacent target
    let mut in_range = vec![false; rows * cols];
    let mut round = 0;
    // reusable BFS data structures
    let mut visited = vec![false; rows * cols];
    let mut found = Vec::new();

    loop {
        // fixed turn order for this round based on positions at round start.
        let mut turn_order: Vec<usize> = (0..units.len())
            .filter(|&i| units[i].hp > 0)
            .collect();
        turn_order.sort_unstable_by_key(|&i| (units[i].pos.row, units[i].pos.col));

        // Rebuild occupancy map for the current round.
        occupied.fill(None);
        for &i in &turn_order {
            occupied[units[i].pos.idx(cols)] = Some(i);
        }

        for i in turn_order {
            // unit may have died earlier this round
            if units[i].hp == 0 {
                continue;
            }

            let attacker_kind = units[i].kind;
            // Check for combat end.
            let has_target = units
                .iter()
                .any(|u| u.hp > 0 && u.kind != attacker_kind);
            if !has_target {
                let hp_sum: usize = units
                    .iter()
                    .filter(|u| u.hp > 0)
                    .map(|u| u.hp)
                    .sum();
                let no_elf_died = units
                    .iter()
                    .all(|u| u.kind != Kind::Elf || u.hp > 0);
                return (no_elf_died, round * hp_sum);
            }

            // Determine if already in attack range.
            let already_in_range = neighbours[units[i].pos.idx(cols)]
                .iter()
                .any(|n| {
                    occupied[n.idx(cols)]
                        .map(|tid| units[tid].kind != attacker_kind)
                        .unwrap_or(false)
                });

            // If not in range, move.
            if !already_in_range {
                // Find all in-range squares
                in_range.fill(false);
                for target in units
                    .iter()
                    .filter(|t| t.hp > 0 && t.kind != attacker_kind)
                {
                    for n in neighbours[target.pos.idx(cols)].iter() {
                        let ni = n.idx(cols);
                        if map[ni] == Tile::Open && occupied[ni].is_none() {
                            in_range[ni] = true;
                        }
                    }
                }

                // Call the helper function to find the best move.
                if let Some(step) = find_best_move(
                    units[i].pos,
                    map,
                    neighbours,
                    cols,
                    &occupied,
                    &in_range,
                    &mut visited,
                    &mut found,
                ) {
                    // Update the unit's position.
                    occupied[units[i].pos.idx(cols)] = None;
                    units[i].pos = step;
                    occupied[step.idx(cols)] = Some(i);
                }
            }

            // Attack: pick adjacent enemy with lowest HP, tie-break by reading order.
            let attacker_pos = units[i].pos;
            let best = neighbours[attacker_pos.idx(cols)]
                .iter()
                .filter_map(|n| occupied[n.idx(cols)])
                .filter(|tid| {
                    let t = &units[*tid];
                    t.hp > 0 && t.kind != attacker_kind
                })
                .min_by_key(|tid| {
                    let t = &units[*tid];
                    (t.hp, t.pos.row, t.pos.col)
                });

            if let Some(tid) = best {
                let power = if attacker_kind == Kind::Elf {
                    elf_attack_power
                } else {
                    3
                };
                let target = &mut units[tid];
                target.hp = target.hp.saturating_sub(power);
                if target.hp == 0 {
                    occupied[target.pos.idx(cols)] = None;
                    if stop_on_elf_death && target.kind == Kind::Elf {
                        return (false, 0);
                    }
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
