use crate::{AoCData, AoCResult};
use aoc_core::AoCError;
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
    neighbours: Vec<[usize; 4]>,
    rows: usize,
    cols: usize,
    units: Vec<Unit>,
}

const EMPTY: usize = usize::MAX;

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let lines: Vec<&str> = input.lines().collect();
        let rows = lines.len();
        let cols = lines.first().unwrap_or(&"").len();

        let mut walls = Vec::with_capacity(rows * cols);
        let mut units = Vec::new();

        for (r, line) in lines.iter().enumerate() {
            // This padding step isn't needed because the input is rectangular for everyone:
            // let mut chars: Vec<char> = line.chars().collect();
            // if chars.len() < cols {
            //     chars.resize(cols, '#');
            // }
            for (c, ch) in line.chars().enumerate() {
                let pos = r * cols + c;
                match ch {
                    '#' => walls.push(true),
                    '.' => walls.push(false),
                    'G' => {
                        units.push(Unit {
                            pos,
                            elf: false,
                            hp: 200,
                        });
                        walls.push(false);
                    }
                    'E' => {
                        units.push(Unit {
                            pos,
                            elf: true,
                            hp: 200,
                        });
                        walls.push(false);
                    }
                    _ => return Err(AoCError::Parsing),
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
        let mut combat = Combat::new(
            self.rows,
            self.cols,
            &self.units,
            &self.walls,
            &self.neighbours,
            3,
            false,
        );
        let (_, outcome) = combat.sim_combat();
        Ok(outcome)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut low = 4;
        let mut high = 200;
        let mut final_outcome = 0;
        let mut combat = Combat::new(
            self.rows,
            self.cols,
            &self.units,
            &self.walls,
            &self.neighbours,
            4,
            true,
        );

        while low <= high {
            let middle = low + (high - low) / 2;
            combat.reset(middle, true);
            let (flawless, outcome) = combat.sim_combat();
            if flawless {
                final_outcome = outcome;
                high = middle - 1;
            } else {
                low = middle + 1;
            }
        }

        if final_outcome != 0 {
            // combat.debug_state();
            Ok(final_outcome)
        } else {
            Err(AoCError::Solving)
        }
    }
}

struct Combat<'a> {
    // static
    initial_units: &'a Vec<Unit>,
    walls: &'a [bool],
    neighbours: &'a [[usize; 4]],
    initial_elf_count: i32,
    elf_attack_power: u8,
    stop_on_elf_death: bool,

    // simulation state
    round: usize,
    units: Vec<Unit>,
    turn_order: Vec<usize>,

    dist_map: Vec<usize>,
    q: VecDeque<(usize, usize, usize)>,
    occupied: Vec<usize>,
    visited_version: Vec<u32>,
    current_version: u32,
    adj_to_elves: Vec<u8>,
    adj_to_goblins: Vec<u8>,

    // kept up-to-date instead of recalculated every time
    total_hp: usize,
    elf_count: i32,
    goblin_count: i32,

    // for debugging
    rows: usize,
    cols: usize,
}

impl<'a> Combat<'a> {
    fn new(
        rows: usize,
        cols: usize,
        units: &'a Vec<Unit>,
        walls: &'a [bool],
        neighbours: &'a [[usize; 4]],
        elf_attack_power: u8,
        stop_on_elf_death: bool,
    ) -> Self {
        let unit_len = units.len();
        let grid_size = rows * cols;
        let mut res = Self {
            initial_units: units,
            walls,
            neighbours,
            initial_elf_count: 0,
            elf_attack_power,
            stop_on_elf_death,

            round: 0,
            units: Vec::with_capacity(unit_len),
            turn_order: Vec::with_capacity(unit_len),

            dist_map: vec![EMPTY; grid_size],
            q: VecDeque::new(),
            occupied: vec![EMPTY; grid_size],
            visited_version: vec![0; grid_size],
            current_version: 1,
            adj_to_elves: vec![0; grid_size],
            adj_to_goblins: vec![0; grid_size],

            total_hp: 0,
            elf_count: 0,
            goblin_count: 0,

            // for debugging
            rows,
            cols,
        };
        res.reset(elf_attack_power, stop_on_elf_death);
        res
    }

    fn reset(&mut self, elf_attack_power: u8, stop_on_elf_death: bool) {
        self.elf_attack_power = elf_attack_power;
        self.stop_on_elf_death = stop_on_elf_death;

        self.round = 0;
        self.units = self.initial_units.clone();
        self.turn_order.clear();

        self.dist_map.fill(EMPTY);
        self.q.clear();
        self.occupied.fill(EMPTY);
        self.visited_version.fill(0);
        self.current_version = 1;
        self.adj_to_elves.fill(0);
        self.adj_to_goblins.fill(0);

        self.total_hp = 0;
        self.elf_count = 0;
        self.goblin_count = 0;
        for (i, u) in self.initial_units.iter().enumerate() {
            self.total_hp += u.hp as usize;
            if u.hp > 0 {
                self.occupied[u.pos] = i;
                if u.elf {
                    self.elf_count += 1;
                } else {
                    self.goblin_count += 1;
                }
            }
        }

        // static but unset during initialisation
        self.initial_elf_count = self.elf_count;

        self.rebuild_in_range();
    }

    fn remove_unit(&mut self, idx: usize) {
        let old_pos = self.units[idx].pos;
        self.occupied[old_pos] = EMPTY;
        self.update_in_range(idx, old_pos, EMPTY);
        if self.units[idx].elf {
            self.elf_count -= 1;
        } else {
            self.goblin_count -= 1;
        }
        self.units[idx].pos = EMPTY;
        self.units[idx].hp = 0;
    }

    fn in_range(&self, i: usize) -> bool {
        let unit = &self.units[i];
        let adj_enemies = if unit.elf {
            self.adj_to_goblins[unit.pos]
        } else {
            self.adj_to_elves[unit.pos]
        };
        adj_enemies > 0
    }

    fn try_move(&mut self, i: usize) {
        let step = self.best_move(i);
        if step != EMPTY {
            let old_pos = self.units[i].pos;
            self.update_in_range(i, old_pos, step);
            self.occupied[old_pos] = EMPTY;
            self.units[i].pos = step;
            self.occupied[step] = i;
        }
    }

    // return false if combat ended
    fn attack(&mut self, i: usize) -> bool {
        let atkr_pos = self.units[i].pos;
        let atkr_elf = self.units[i].elf;
        let atkr_power = if atkr_elf { self.elf_attack_power } else { 3 };

        let mut best = EMPTY;
        let mut best_key = (u8::MAX, usize::MAX);

        for &n_idx in &self.neighbours[atkr_pos] {
            if n_idx == EMPTY {
                continue;
            }
            let t_idx = self.occupied[n_idx];
            if t_idx == EMPTY {
                continue;
            }
            let target = &self.units[t_idx];
            if target.elf == atkr_elf {
                continue;
            }

            let key = (target.hp, target.pos);
            if key < best_key {
                best_key = key;
                best = t_idx;
            }
        }

        if best != EMPTY {
            let target = &mut self.units[best];
            let target_is_elf = target.elf;
            let prev_hp = target.hp;
            let damage = atkr_power.min(prev_hp);
            target.hp -= damage;
            self.total_hp -= damage as usize;

            if target.hp == 0 {
                self.remove_unit(best);
                if target_is_elf && self.stop_on_elf_death {
                    return false;
                }
            }
        }
        true
    }

    // returns false if combat ended
    fn take_turn(&mut self, i: usize) -> bool {
        if !self.in_range(i) {
            self.try_move(i);
        }
        self.attack(i)
    }

    fn has_target(&self, i: usize) -> bool {
        if self.units[i].elf {
            self.goblin_count > 0
        } else {
            self.elf_count > 0
        }
    }

    fn calc_turn_order(&mut self) {
        self.turn_order.clear();
        for i in 0..self.units.len() {
            if self.units[i].hp > 0 {
                self.turn_order.push(i);
            }
        }
        self.turn_order
            .sort_unstable_by_key(|&i| self.units[i].pos);
    }

    fn update_in_range(&mut self, i: usize, old_pos: usize, new_pos: usize) {
        let contrib = if self.units[i].elf {
            &mut self.adj_to_elves
        } else {
            &mut self.adj_to_goblins
        };
        let mut adjust = |pos: usize, add: bool| {
            for &n in &self.neighbours[pos] {
                if n == EMPTY {
                    continue;
                }
                if add {
                    contrib[n] = contrib[n].saturating_add(1);
                } else if contrib[n] > 0 {
                    contrib[n] -= 1;
                }
            }
        };
        if old_pos != EMPTY {
            adjust(old_pos, false);
        }
        if new_pos != EMPTY {
            adjust(new_pos, true);
        }
    }

    fn rebuild_in_range(&mut self) {
        self.adj_to_elves.fill(0);
        self.adj_to_goblins.fill(0);

        for i in 0..self.units.len() {
            if self.units[i].hp == 0 {
                continue;
            }
            let contrib = if self.units[i].elf {
                &mut self.adj_to_elves
            } else {
                &mut self.adj_to_goblins
            };

            for &n in &self.neighbours[self.units[i].pos] {
                if n == EMPTY {
                    continue;
                }
                contrib[n] = contrib[n].saturating_add(1);
            }
        }
    }

    fn sim_combat(&mut self) -> (bool, usize) {
        loop {
            self.calc_turn_order();

            for order_i in 0..self.turn_order.len() {
                let i = self.turn_order[order_i];
                if self.units[i].hp == 0 {
                    continue;
                }

                if !self.has_target(i) {
                    return (
                        self.elf_count == self.initial_elf_count,
                        self.round * self.total_hp,
                    );
                }
                if !self.take_turn(i) {
                    return (false, 0);
                }
            }
            self.round += 1;
        }
    }

    fn best_move(&mut self, u_idx: usize) -> usize {
        self.q.clear();
        self.current_version += 1;
        let u = self.units[u_idx].clone();
        let adj_enemies = if u.elf {
            &self.adj_to_goblins
        } else {
            &self.adj_to_elves
        };

        let mut min_dist = EMPTY;
        self.q.push_back((0, u.pos, EMPTY));
        self.visited_version[u.pos] = self.current_version;
        let mut bests = Vec::new();

        while let Some((dist, pos, first)) = self.q.pop_front() {
            if dist > min_dist {
                break;
            }

            if adj_enemies[pos] > 0 {
                if dist < min_dist {
                    bests.clear();
                    min_dist = dist;
                }
                bests.push((pos, first));
                continue; // Do not explore from a target square.
            }

            // Explore neighbours in reading order to handle tie-breaking for paths.
            for n in self.neighbours[pos] {
                if n != EMPTY
                    && self.visited_version[n] != self.current_version
                    && !self.walls[n]
                    && self.occupied[n] == EMPTY
                {
                    self.visited_version[n] = self.current_version;
                    let new_first = if first != EMPTY { first } else { n };
                    self.q
                        .push_back((dist + 1, n, new_first));
                }
            }
        }
        if bests.is_empty() {
            return EMPTY;
        }

        // Sort by target position (reading order), then by first step (reading order)
        bests.sort_by_key(|&(target, first)| (target, first));
        bests[0].1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_1() {
        // #######       #######
        // #G..#E#       #...#E#   E(200)
        // #E#E.E#       #E#...#   E(197)
        // #G.##.#  -->  #.E##.#   E(185)
        // #...#E#       #E..#E#   E(200), E(200)
        // #...E.#       #.....#
        // #######       #######
        // Combat ends after 37 full rounds
        // Elves win with 982 total hit points left
        // Outcome: 37 * 982 = 36334
        let input = "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "36334");
    }

    #[test]
    fn part_1_2() {
        // #######       #######
        // #E..EG#       #.E.E.#   E(164), E(197)
        // #.#G.E#       #.#E..#   E(200)
        // #E.##E#  -->  #E.##.#   E(98)
        // #G..#.#       #.E.#.#   E(200)
        // #..E#.#       #...#.#
        // #######       #######
        // Combat ends after 46 full rounds
        // Elves win with 859 total hit points left
        // Outcome: 46 * 859 = 39514
        let input = "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "39514");
    }
    #[test]
    fn part_1_3() {
        // #######       #######
        // #E.G#.#       #G.G#.#   G(200), G(98)
        // #.#G..#       #.#G..#   G(200)
        // #G.#.G#  -->  #..#..#
        // #G..#.#       #...#G#   G(95)
        // #...E.#       #...G.#   G(200)
        // #######       #######
        // Combat ends after 35 full rounds
        // Goblins win with 793 total hit points left
        // Outcome: 35 * 793 = 27755
        let input = "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "27755");
    }

    #[test]
    fn part_1_4() {
        // #######       #######
        // #.E...#       #.....#
        // #.#..G#       #.#G..#   G(200)
        // #.###.#  -->  #.###.#
        // #E#G#G#       #.#.#.#
        // #...#G#       #G.G#G#   G(98), G(38), G(200)
        // #######       #######
        // Combat ends after 54 full rounds
        // Goblins win with 536 total hit points left
        // Outcome: 54 * 536 = 28944
        let input = "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "28944");
    }

    #[test]
    fn part_1_5() {
        // #########       #########
        // #G......#       #.G.....#   G(137)
        // #.E.#...#       #G.G#...#   G(200), G(200)
        // #..##..G#       #.G##...#   G(200)
        // #...##..#  -->  #...##..#
        // #...#...#       #.G.#...#   G(200)
        // #.G...G.#       #.......#
        // #.....G.#       #.......#
        // #########       #########
        // Combat ends after 20 full rounds
        // Goblins win with 937 total hit points left
        // Outcome: 20 * 937 = 18740
        let input = "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "18740");
    }

    #[test]
    fn part_2_1() {
        // #######       #######
        // #.G...#       #..E..#   E(158)
        // #...EG#       #...E.#   E(14)
        // #.#.#G#  -->  #.#.#.#
        // #..G#E#       #...#.#
        // #.....#       #.....#
        // #######       #######
        //
        // Combat ends after 29 full rounds
        // Elves win with 172 total hit points left
        // Outcome: 29 * 172 = 4988
        let input = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "4988");
    }

    #[test]
    fn part_2_2() {
        // #######       #######
        // #E..EG#       #.E.E.#   E(200), E(23)
        // #.#G.E#       #.#E..#   E(200)
        // #E.##E#  -->  #E.##E#   E(125), E(200)
        // #G..#.#       #.E.#.#   E(200)
        // #..E#.#       #...#.#
        // #######       #######
        // Combat ends after 33 full rounds
        // Elves win with 948 total hit points left
        // Outcome: 33 * 948 = 31284
        let input = "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "31284");
    }

    #[test]
    fn part_2_3() {
        // #######       #######
        // #E.G#.#       #.E.#.#   E(8)
        // #.#G..#       #.#E..#   E(86)
        // #G.#.G#  -->  #..#..#
        // #G..#.#       #...#.#
        // #...E.#       #.....#
        // #######       #######
        // Combat ends after 37 full rounds
        // Elves win with 94 total hit points left
        // Outcome: 37 * 94 = 3478
        let input = "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "3478");
    }
    #[test]
    fn part_2_4() {
        // #######       #######
        // #.E...#       #...E.#   E(14)
        // #.#..G#       #.#..E#   E(152)
        // #.###.#  -->  #.###.#
        // #E#G#G#       #.#.#.#
        // #...#G#       #...#.#
        // #######       #######
        //
        // Combat ends after 39 full rounds
        // Elves win with 166 total hit points left
        // Outcome: 39 * 166 = 6474
        let input = "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "6474");
    }

    #[test]
    fn part_2_5() {
        // #########       #########
        // #G......#       #.......#
        // #.E.#...#       #.E.#...#   E(38)
        // #..##..G#       #..##...#
        // #...##..#  -->  #...##..#
        // #...#...#       #...#...#
        // #.G...G.#       #.......#
        // #.....G.#       #.......#
        // #########       #########
        // Combat ends after 30 full rounds
        // Elves win with 38 total hit points left
        // Outcome: 30 * 38 = 1140
        let input = "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "1140");
    }
}
