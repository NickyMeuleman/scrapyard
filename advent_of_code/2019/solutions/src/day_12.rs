use std::fmt::Display;

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data([Pos; 4]);

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone)]
struct Moon {
    pos: Pos,
    velocity: Pos,
}

impl Moon {
    fn new(starting_pos: Pos) -> Self {
        Self {
            pos: starting_pos,
            velocity: Pos { x: 0, y: 0, z: 0 },
        }
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    fn potential_energy(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

impl AoCData<'_> for Data {
    // I could do this cleanly, but I don't wanna, I gotta go soon and this works
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut iter = input.lines();
        let mut pos_1 = iter.next().unwrap();
        pos_1 = pos_1.strip_prefix("<").unwrap();
        pos_1 = pos_1.strip_suffix(">").unwrap();
        let mut pos_1_iter = pos_1
            .split_terminator(", ")
            .map(|s| s.split_once("=").unwrap());
        let pos_1 = Pos {
            x: pos_1_iter
                .next()
                .unwrap()
                .1
                .parse()
                .unwrap(),
            y: pos_1_iter
                .next()
                .unwrap()
                .1
                .parse()
                .unwrap(),
            z: pos_1_iter
                .next()
                .unwrap()
                .1
                .parse()
                .unwrap(),
        };
        let mut pos_2 = iter.next().unwrap();
        pos_2 = pos_2.strip_prefix("<").unwrap();
        pos_2 = pos_2.strip_suffix(">").unwrap();
        let mut pos_2_iter = pos_2
            .split_terminator(", ")
            .map(|s| s.split_once("=").unwrap());
        let pos_2 = Pos {
            x: pos_2_iter
                .next()
                .unwrap()
                .1
                .parse()
                .unwrap(),
            y: pos_2_iter
                .next()
                .unwrap()
                .1
                .parse()
                .unwrap(),
            z: pos_2_iter
                .next()
                .unwrap()
                .1
                .parse()
                .unwrap(),
        };
        let mut pos_3 = iter.next().unwrap();
        pos_3 = pos_3.strip_prefix("<").unwrap();
        pos_3 = pos_3.strip_suffix(">").unwrap();
        let mut pos_3_iter = pos_3
            .split_terminator(", ")
            .map(|s| s.split_once("=").unwrap());
        let pos_3 = Pos {
            x: pos_3_iter
                .next()
                .unwrap()
                .1
                .parse()
                .unwrap(),
            y: pos_3_iter
                .next()
                .unwrap()
                .1
                .parse()
                .unwrap(),
            z: pos_3_iter
                .next()
                .unwrap()
                .1
                .parse()
                .unwrap(),
        };
        let mut pos_4 = iter.next().unwrap();
        pos_4 = pos_4.strip_prefix("<").unwrap();
        pos_4 = pos_4.strip_suffix(">").unwrap();
        let mut pos_4_iter = pos_4
            .split_terminator(", ")
            .map(|s| s.split_once("=").unwrap());
        let pos_4 = Pos {
            x: pos_4_iter
                .next()
                .unwrap()
                .1
                .parse()
                .unwrap(),
            y: pos_4_iter
                .next()
                .unwrap()
                .1
                .parse()
                .unwrap(),
            z: pos_4_iter
                .next()
                .unwrap()
                .1
                .parse()
                .unwrap(),
        };

        Ok(Self([pos_1, pos_2, pos_3, pos_4]))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut moons = [
            Moon::new(self.0[0]),
            Moon::new(self.0[1]),
            Moon::new(self.0[2]),
            Moon::new(self.0[3]),
        ];
        for _ in 0..1000 {
            // update velocities by applying gravity
            for (idx_1, idx_2) in (0..moons.len()).tuple_combinations() {
                let diff_x = (moons[idx_1].pos.x - moons[idx_2].pos.x).signum();
                let diff_y = (moons[idx_1].pos.y - moons[idx_2].pos.y).signum();
                let diff_z = (moons[idx_1].pos.z - moons[idx_2].pos.z).signum();

                moons[idx_1].velocity.x -= diff_x;
                moons[idx_2].velocity.x += diff_x;

                moons[idx_1].velocity.y -= diff_y;
                moons[idx_2].velocity.y += diff_y;

                moons[idx_1].velocity.z -= diff_z;
                moons[idx_2].velocity.z += diff_z;
            }
            // apply velocities
            for moon in moons.iter_mut() {
                moon.pos.x += moon.velocity.x;
                moon.pos.y += moon.velocity.y;
                moon.pos.z += moon.velocity.z;
            }
        }
        Ok(moons
            .iter()
            .map(|moon| moon.energy())
            .sum::<i32>())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut moons = [
            Moon::new(self.0[0]),
            Moon::new(self.0[1]),
            Moon::new(self.0[2]),
            Moon::new(self.0[3]),
        ];

        let (mut period_x, mut period_y, mut period_z) = (None, None, None);
        let start_x = moons
            .iter()
            .map(|m| (m.pos.x, m.velocity.x))
            .collect::<Vec<_>>();
        let start_y = moons
            .iter()
            .map(|m| (m.pos.y, m.velocity.y))
            .collect::<Vec<_>>();
        let start_z = moons
            .iter()
            .map(|m| (m.pos.z, m.velocity.z))
            .collect::<Vec<_>>();

        let mut steps: u64 = 0;

        while period_x.is_none() || period_y.is_none() || period_z.is_none() {
            for (idx_1, idx_2) in (0..moons.len()).tuple_combinations() {
                let diff_x = (moons[idx_1].pos.x - moons[idx_2].pos.x).signum();
                let diff_y = (moons[idx_1].pos.y - moons[idx_2].pos.y).signum();
                let diff_z = (moons[idx_1].pos.z - moons[idx_2].pos.z).signum();

                moons[idx_1].velocity.x -= diff_x;
                moons[idx_2].velocity.x += diff_x;

                moons[idx_1].velocity.y -= diff_y;
                moons[idx_2].velocity.y += diff_y;

                moons[idx_1].velocity.z -= diff_z;
                moons[idx_2].velocity.z += diff_z;
            }
            // apply velocities
            for moon in moons.iter_mut() {
                moon.pos.x += moon.velocity.x;
                moon.pos.y += moon.velocity.y;
                moon.pos.z += moon.velocity.z;
            }
            steps += 1;
            if let None = period_x {
                let curr_x = moons
                    .iter()
                    .map(|m| (m.pos.x, m.velocity.x))
                    .collect::<Vec<_>>();
                if curr_x == start_x {
                    period_x = Some(steps)
                }
            }
            if let None = period_y {
                let curr_y = moons
                    .iter()
                    .map(|m| (m.pos.y, m.velocity.y))
                    .collect::<Vec<_>>();
                if curr_y == start_y {
                    period_y = Some(steps)
                }
            }
            if let None = period_z {
                let curr_z = moons
                    .iter()
                    .map(|m| (m.pos.z, m.velocity.z))
                    .collect::<Vec<_>>();
                if curr_z == start_z {
                    period_z = Some(steps)
                }
            }
        }

        Ok(lcm(
            period_x.ok_or(AoCError::Solving)?,
            lcm(
                period_y.ok_or(AoCError::Solving)?,
                period_z.ok_or(AoCError::Solving)?,
            ),
        ))
    }
    // fn part_2(&self) -> AoCResult<impl Display> {
    //     let mut moons = [
    //         Moon::new(self.0[0]),
    //         Moon::new(self.0[1]),
    //         Moon::new(self.0[2]),
    //         Moon::new(self.0[3]),
    //     ];

    //     fn determine_periods(moons: &mut [Moon]) -> (u64, u64, u64) {
    //         let (mut period_x, mut period_y, mut period_z) = (None, None, None);
    //         let start_x = moons
    //             .iter()
    //             .map(|m| (m.pos.x, m.velocity.x))
    //             .collect::<Vec<_>>();
    //         let start_y = moons
    //             .iter()
    //             .map(|m| (m.pos.y, m.velocity.y))
    //             .collect::<Vec<_>>();
    //         let start_z = moons
    //             .iter()
    //             .map(|m| (m.pos.z, m.velocity.z))
    //             .collect::<Vec<_>>();

    //         let mut steps: u64 = 0;

    //         while period_x.is_none() || period_y.is_none() || period_z.is_none() {
    //             for (idx_1, idx_2) in (0..moons.len()).tuple_combinations() {
    //                 let diff_x = (moons[idx_1].pos.x - moons[idx_2].pos.x).signum();
    //                 let diff_y = (moons[idx_1].pos.y - moons[idx_2].pos.y).signum();
    //                 let diff_z = (moons[idx_1].pos.z - moons[idx_2].pos.z).signum();

    //                 moons[idx_1].velocity.x -= diff_x;
    //                 moons[idx_2].velocity.x += diff_x;

    //                 moons[idx_1].velocity.y -= diff_y;
    //                 moons[idx_2].velocity.y += diff_y;

    //                 moons[idx_1].velocity.z -= diff_z;
    //                 moons[idx_2].velocity.z += diff_z;
    //             }
    //             // apply velocities
    //             for moon in moons.iter_mut() {
    //                 moon.pos.x += moon.velocity.x;
    //                 moon.pos.y += moon.velocity.y;
    //                 moon.pos.z += moon.velocity.z;
    //             }
    //             steps += 1;
    //             if let None = period_x {
    //                 let curr_x = moons
    //                     .iter()
    //                     .map(|m| (m.pos.x, m.velocity.x))
    //                     .collect::<Vec<_>>();
    //                 if curr_x == start_x {
    //                     period_x = Some(steps)
    //                 }
    //             }
    //             if let None = period_y {
    //                 let curr_y = moons
    //                     .iter()
    //                     .map(|m| (m.pos.y, m.velocity.y))
    //                     .collect::<Vec<_>>();
    //                 if curr_y == start_y {
    //                     period_y = Some(steps)
    //                 }
    //             }
    //             if let None = period_z {
    //                 let curr_z = moons
    //                     .iter()
    //                     .map(|m| (m.pos.z, m.velocity.z))
    //                     .collect::<Vec<_>>();
    //                 if curr_z == start_z {
    //                     period_z = Some(steps)
    //                 }
    //             }
    //         }

    //         (period_x.unwrap(), period_y.unwrap(), period_z.unwrap())
    //     }
    //     let (px, py, pz) = determine_periods(&mut moons);
    //     Ok(lcm(px, lcm(py, pz)))
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "strt");
    }

    #[test]
    fn part_2() {
        let input = "";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "");
    }
}
