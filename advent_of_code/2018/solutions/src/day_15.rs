use aoc_core::AoCError;

use crate::{AoCData, AoCResult};
use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Unit {
    pos: usize,
    elf: bool,
    hp: u8,
}

#[derive(Debug, Clone)]
pub struct Data {
    walls: Vec<bool>,
    neighbours: Vec<[usize; 4]>, // precomputed reading-ordered neighbours for each cell
    rows: usize,
    cols: usize,
    units: Vec<Unit>,
}

const EMPTY: usize = usize::MAX;

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let lines: Vec<&str> = input.lines().collect();
        let rows = lines.len();
        let cols = lines
            .first()
            .map(|l| l.chars().count())
            .unwrap_or(0);

        let mut walls = Vec::with_capacity(rows * cols);
        let mut units = Vec::new();

        for (r, line) in lines.iter().enumerate() {
            // pad short lines with walls to keep rectangular shape
            let mut chars: Vec<char> = line.chars().collect();
            if chars.len() < cols {
                chars.resize(cols, '#');
            }
            for (c, ch) in chars.into_iter().enumerate() {
                match ch {
                    '#' => walls.push(true),
                    '.' => walls.push(false),
                    'G' => {
                        units.push(Unit {
                            pos: r * cols + c,
                            elf: false,
                            hp: 200,
                        });
                        walls.push(false);
                    }
                    'E' => {
                        units.push(Unit {
                            pos: r * cols + c,
                            elf: true,
                            hp: 200,
                        });
                        walls.push(false);
                    }
                    _ => panic!("Invalid char"),
                }
            }
        }

        // Precompute neighbours in reading order
        let mut neighbours = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in 0..cols {
                let mut n = [EMPTY; 4];
                if r > 0 {
                    n[0] = (r - 1) * cols + c;
                }
                if c > 0 {
                    n[1] = r * cols + (c - 1);
                }
                if c + 1 < cols {
                    n[2] = r * cols + (c + 1);
                }
                if r + 1 < rows {
                    n[3] = (r + 1) * cols + c;
                }
                neighbours.push(n);
            }
        }

        Ok(Self {
            walls,
            neighbours,
            rows,
            cols,
            units,
        })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut units = self.units.clone();
        let (_, outcome) = sim_combat(
            &self.walls,
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
        let mut min_ap = u8::MAX;
        let mut low = 4;
        let mut high = 200;
        let mut final_outcome = 0;
        while low <= high {
            let middle = low + (high - low) / 2;
            buf.clear();
            buf.extend(self.units.iter().cloned());
            let (flawless, outcome) = sim_combat(
                &self.walls,
                &self.neighbours,
                self.rows,
                self.cols,
                &mut buf,
                middle,
                true,
            );
            if flawless {
                // this AP might be the minimum.
                min_ap = middle;
                final_outcome = outcome;
                if low == high {
                    break;
                }
                high = middle - 1;
            } else {
                if middle == high {
                    break;
                }
                low = middle + 1;
            }
        }
        if min_ap != u8::MAX {
            Ok(final_outcome)
        } else {
            Err(AoCError::Solving)
        }
    }
}

fn best_move_final(
    start: usize,
    walls: &[bool],
    neighbours: &[[usize; 4]],
    occupied: &[usize],
    in_range_counts: &[u8],
    dist_map: &mut [usize],
    visited_version: &mut [u32],
    current_version: u32,
    q: &mut VecDeque<usize>,
) -> usize {
    q.clear();

    let mut min_dist = usize::MAX;
    let mut best = usize::MAX;

    q.push_back(start);
    dist_map[start] = 0;
    visited_version[start] = current_version; // Mark start as visited for this "generation"

    while let Some(pos) = q.pop_front() {
        let dist = dist_map[pos];
        if dist > min_dist {
            break;
        }

        if in_range_counts[pos] > 0 {
            if dist < min_dist || (dist == min_dist && pos < best) {
                min_dist = dist;
                best = pos;
            }
            continue;
        }

        for &n in &neighbours[pos] {
            // The check for "unvisited" now uses the version number.
            if n != EMPTY
                && visited_version[n] != current_version
                && !walls[n]
                && occupied[n] == EMPTY
            {
                visited_version[n] = current_version; // "Visit" the node for this generation.
                dist_map[n] = dist + 1;
                q.push_back(n);
            }
        }
    }

    if best == EMPTY {
        EMPTY
    } else {
        // Backtracking logic is identical to your original, but we add a version check for correctness.
        let mut cur = best;
        while dist_map[cur] > 1 {
            let mut next = EMPTY;
            for &n in &neighbours[cur] {
                // The distance check implicitly works, but checking the version is safer.
                if n != EMPTY
                    && visited_version[n] == current_version
                    && dist_map[n] == dist_map[cur] - 1
                {
                    next = n;
                    break;
                }
            }
            cur = next;
        }
        cur
    }
}
// Day 15/Part 1           time:   [1.8645 ms 1.8764 ms 1.8931 ms]
//                         change: [-0.0853% +0.4247% +1.0356%] (p = 0.17 > 0.05)
//                         No change in performance detected.
// Found 17 outliers among 100 measurements (17.00%)
//   3 (3.00%) low mild
//   7 (7.00%) high mild
//   7 (7.00%) high severe
// Day 15/Part 2           time:   [3.1916 ms 3.1963 ms 3.2017 ms]
//                         change: [-0.5166% -0.3218% -0.1279%] (p = 0.00 < 0.05)
//                         Change within noise threshold.
// Found 3 outliers among 100 measurements (3.00%)
//   2 (2.00%) high mild
//   1 (1.00%) high severe
// Day 15/Both parts       time:   [5.1528 ms 5.1636 ms 5.1820 ms]
//                         change: [-0.5022% -0.2659% +0.0937%] (p = 0.08 > 0.05)
//                         No change in performance detected.
fn best_move(
    start: usize,
    walls: &[bool],
    neighbours: &[[usize; 4]],
    occupied: &[usize],
    in_range: &[bool],
    dist_map: &mut [usize],
    q: &mut VecDeque<usize>,
) -> usize {
    q.clear();
    dist_map.fill(EMPTY);
    let mut min_dist = EMPTY;
    q.push_back(start);
    dist_map[start] = 0;
    let mut best = EMPTY;

    while let Some(pos) = q.pop_front() {
        let dist = dist_map[pos];
        // stop the loop, do not explore paths that are longer than min_dist
        if dist > min_dist {
            break;
        }

        if in_range[pos] {
            // update best target if a better one (better dist or better reading order) is found
            if dist < min_dist || (dist == min_dist) && pos < best {
                min_dist = dist;
                best = pos;
            }
            continue; // Do not explore from a target square.
        }

        // Explore neighbours in reading order to handle tie-breaking for paths.
        for &n in &neighbours[pos] {
            if n != EMPTY && dist_map[n] == EMPTY && !walls[n] && occupied[n] == EMPTY {
                dist_map[n] = dist + 1;
                q.push_back(n);
            }
        }
    }

    if best == EMPTY {
        EMPTY
    } else {
        // return the first step in reading order that belongs to the best target (in reading order during the search)
        // backtrack from the best target, always picking the reading-order best neighbour
        let mut cur = best;
        while dist_map[cur] > 1 {
            let mut next = EMPTY;
            for &n in &neighbours[cur] {
                if n != EMPTY && dist_map[n] == dist_map[cur] - 1 {
                    next = n;
                    break;
                }
            }
            cur = next;
        }
        cur
    }
}

fn sim_combat(
    walls: &[bool],
    neighbours: &[[usize; 4]],
    rows: usize,
    cols: usize,
    units: &mut [Unit],
    elf_attack_power: u8,
    stop_on_elf_death: bool,
) -> (bool, usize) {
    let mut round = 0;
    // the in_range is a bool-mask with the same shape as the map, true means it's a valid destination for a
    // unit that has an adjacent target
    // let mut in_range = vec![false; rows * cols];
    // let mut in_range_set = Vec::new();
    let mut occupied = vec![EMPTY; rows * cols];
    let mut dist_map = vec![EMPTY; rows * cols];
    let mut q = VecDeque::new();
    let mut visited_version = vec![0u32; rows * cols];
    let mut current_version = 1u32;
    let mut turn_order = Vec::with_capacity(units.len());
    let mut in_range_for_elves = vec![0u8; rows * cols]; // squares elves can move to (adjacent to goblins)
    let mut in_range_for_goblins = vec![0u8; rows * cols]; // squares goblins can move to (adjacent to elves)

    let mut elf_count = 0;
    let mut goblin_count = 0;
    for u in units.iter() {
        if u.hp == 0 {
            continue;
        }
        if u.elf {
            elf_count += 1;
        } else {
            goblin_count += 1;
        }
    }
    let initial_elf_count = elf_count;

    // units are in reading-order when combat starts
    for (i, u) in units.iter().enumerate() {
        if u.hp > 0 {
            occupied[u.pos] = i;
        }
    }

    let mut total_hp: usize = units
        .iter()
        .map(|u| u.hp as usize)
        .sum();

    loop {
        // fixed turn order for this round based on positions at round start.
        turn_order.clear();
        for (i, u) in units.iter().enumerate() {
            if u.hp > 0 {
                turn_order.push(i);
            }
        }
        turn_order.sort_unstable_by_key(|&i| units[i].pos);
        // Because pos is already row*cols+col, this preserves reading order.

        // // Rebuild occupancy map for the current round.
        // occupied.fill(EMPTY);
        // for &i in &turn_order {
        //     occupied[units[i].pos] = i;
        // }

        in_range_for_elves.fill(0);
        in_range_for_goblins.fill(0);

        for (i, u) in units.iter().enumerate() {
            if u.hp == 0 {
                continue;
            }
            // This unit contributes to the 'opponent' in-range refcounts.
            let contrib = if u.elf {
                &mut in_range_for_goblins
            } else {
                &mut in_range_for_elves
            };

            for &n in &neighbours[u.pos] {
                if n != EMPTY && !walls[n] && occupied[n] == EMPTY {
                    // Saturating add avoids overflow; u8 is plenty for typical adjacency.
                    contrib[n] = contrib[n].saturating_add(1);
                }
            }
        }

        for &i in &turn_order {
            // unit may have died earlier this round
            if units[i].hp == 0 {
                continue;
            }

            let atkr_is_elf = units[i].elf;
            let power = if atkr_is_elf { elf_attack_power } else { 3 };
            // Check for combat end.
            // let mut has_target = false;
            // for u in units.iter() {
            //     if u.hp > 0 && u.elf != atkr_is_elf {
            //         has_target = true;
            //         break;
            //     }
            // }
            let has_target = if atkr_is_elf {
                goblin_count > 0
            } else {
                elf_count > 0
            };
            if !has_target {
                // let mut hp_sum = 0;
                // for u in units.iter() {
                //     hp_sum += u.hp as usize;
                // }
                // let mut no_elf_died = true;
                // for u in units.iter() {
                //     hp_sum += u.hp as usize;
                //     if u.elf && u.hp == 0 {
                //         no_elf_died = false;
                //     }
                // }
                return (elf_count == initial_elf_count, round * total_hp);
            }

            // Determine if already in attack range.
            let mut already_in_range = false;
            for &n in &neighbours[units[i].pos] {
                if n == EMPTY {
                    continue;
                }
                let tid = occupied[n];
                if tid == EMPTY {
                    continue;
                }
                // inclusion in occupied implies hp > 0
                let t = &units[tid];
                if t.elf != atkr_is_elf {
                    already_in_range = true;
                    break;
                }
            }

            // If not in range, move.
            if !already_in_range {
                // Find all in-range squares
                // in_range_set.clear();
                // for target in units.iter() {
                //     if target.hp == 0 || target.elf == atkr_is_elf {
                //         continue;
                //     }
                //     for &n in &neighbours[target.pos] {
                //         if n != EMPTY && !walls[n] && occupied[n] == EMPTY {
                //             in_range[n] = true;
                //             in_range_set.push(n);
                //         }
                //     }
                // }

                // let step = best_move(
                //     units[i].pos,
                //     walls,
                //     neighbours,
                //     &occupied,
                //     &in_range,
                //     &mut dist_map,
                //     &mut q,
                // );

                // let step = best_move_final(
                //     units[i].pos,
                //     walls,
                //     neighbours,
                //     &occupied,
                //     &in_range,
                //     &mut dist_map,
                //     &mut visited_version,
                //     current_version,
                //     &mut q,
                // );
                // // Increment the version for the next BFS run.
                // current_version += 1;

                let step = best_move_final(
                    units[i].pos,
                    walls,
                    neighbours,
                    &occupied,
                    if atkr_is_elf {
                        &in_range_for_elves
                    } else {
                        &in_range_for_goblins
                    },
                    &mut dist_map,
                    &mut visited_version,
                    current_version,
                    &mut q,
                );
                current_version += 1;

                // Update the unit's position.
                // if step != EMPTY {
                //     occupied[units[i].pos] = EMPTY;
                //     units[i].pos = step;
                //     occupied[step] = i;
                // }

                if step != EMPTY {
                    let old_pos = units[i].pos;
                    let new_pos = step;

                    occupied[old_pos] = EMPTY;
                    units[i].pos = new_pos;
                    occupied[new_pos] = i;

                    // This unit contributes to the opponent’s refcount array.
                    let contrib = if atkr_is_elf {
                        &mut in_range_for_goblins
                    } else {
                        &mut in_range_for_elves
                    };

                    // Remove contributions from old_pos neighbors
                    for &n in &neighbours[old_pos] {
                        if n != EMPTY && !walls[n] && occupied[n] == EMPTY {
                            // Guard underflow
                            if contrib[n] > 0 {
                                contrib[n] -= 1;
                            }
                        }
                    }

                    // Add contributions for new_pos neighbors
                    for &n in &neighbours[new_pos] {
                        if n != EMPTY && !walls[n] && occupied[n] == EMPTY {
                            contrib[n] = contrib[n].saturating_add(1);
                        }
                    }
                }
                // reset in_range
                // for &n in &in_range_set {
                //     in_range[n] = false;
                // }
            }

            // Attack: pick adjacent enemy with lowest HP, tie-break by reading order.
            let atkr_pos = units[i].pos;
            let mut best = EMPTY;
            let mut best_key = (u8::MAX, usize::MAX);

            for &n in &neighbours[atkr_pos] {
                if n == EMPTY {
                    continue;
                }
                let tid = occupied[n];
                if tid == EMPTY {
                    continue;
                }
                // inclusion in occupied implies hp > 0
                let t = &units[tid];
                if t.elf == atkr_is_elf {
                    continue;
                }

                let key = (t.hp, t.pos);
                if key < best_key {
                    best_key = key;
                    best = tid;
                }
            }

            if best != EMPTY {
                let target = &mut units[best];
                let prev = target.hp;
                target.hp = target.hp.saturating_sub(power);
                let dealt = (prev - target.hp) as usize;
                total_hp -= dealt;
                if target.hp == 0 {
                    occupied[target.pos] = EMPTY;

                    if target.elf {
                        if stop_on_elf_death {
                            return (false, 0);
                        }
                        elf_count -= 1;
                    } else {
                        goblin_count -= 1;
                    }
                    // Remove this unit's contribution to the opponent refcounts
                    let contrib = if target.elf {
                        &mut in_range_for_goblins
                    } else {
                        &mut in_range_for_elves
                    };
                    for &n in &neighbours[target.pos] {
                        if n != EMPTY && !walls[n] && occupied[n] == EMPTY && contrib[n] > 0 {
                            contrib[n] -= 1;
                        }
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
