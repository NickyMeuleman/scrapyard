use hashbrown::HashSet;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone)]
pub struct Data {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
enum Instruction {
    On(Range),
    Off(Range),
}

#[derive(Debug, Clone)]
struct Range {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Data {
    pub fn part_one(&self) -> usize {
        let mut cubes: HashSet<Point> = HashSet::new();
        // lol, I should do this a more elegant way, shouldn't I?
        let instructions: Vec<&Instruction> = self
            .instructions
            .iter()
            .filter(|instruction| {
                let range = match instruction {
                    Instruction::On(range) => range,
                    Instruction::Off(range) => range,
                };
                range.x_min >= -50
                    && range.y_min >= -50
                    && range.z_min >= -50
                    && range.x_max <= 50
                    && range.y_max <= 50
                    && range.z_max <= 50
            })
            .collect();

        // brute force!
        for instruction in instructions {
            match instruction {
                Instruction::On(range) => {
                    for x in range.x_min..=range.x_max {
                        for y in range.y_min..=range.y_max {
                            for z in range.z_min..=range.z_max {
                                let point = Point { x, y, z };
                                cubes.insert(point);
                            }
                        }
                    }
                }
                Instruction::Off(range) => {
                    for x in range.x_min..=range.x_max {
                        for y in range.y_min..=range.y_max {
                            for z in range.z_min..=range.z_max {
                                let point = Point { x, y, z };
                                cubes.remove(&point);
                            }
                        }
                    }
                }
            }
        }

        cubes.len()
    }

    // pub fn part_two(&self) -> i64 {
    //     // my solution ran out of memory for p2, so I used https://github.com/chaosteil/aoc2021/blob/main/aoc22/src/main.rs
    //     type Range = (isize, isize);
    //     type Cube = (Range, Range, Range);

    //     struct ToggleCube {
    //         toggle: bool,
    //         cube: Cube,
    //     }
    //     let input = std::fs::read_to_string("./input.txt").unwrap();
    //     let lines: Vec<(bool, (Range, Range, Range))> = input
    //         .trim()
    //         .lines()
    //         .map(|s| {
    //             let s = s.split_once(' ').unwrap();
    //             let toggle = match s.0 {
    //                 "on" => true,
    //                 "off" => false,
    //                 _ => unreachable!(),
    //             };
    //             let mut pos = s.1.split(',');
    //             let (x1, x2) = pos.next().unwrap().split_once("..").unwrap();
    //             let (y1, y2) = pos.next().unwrap().split_once("..").unwrap();
    //             let (z1, z2) = pos.next().unwrap().split_once("..").unwrap();
    //             (
    //                 toggle,
    //                 (
    //                     (
    //                         x1[2..].parse::<isize>().unwrap(),
    //                         x2.parse::<isize>().unwrap(),
    //                     ),
    //                     (
    //                         y1[2..].parse::<isize>().unwrap(),
    //                         y2.parse::<isize>().unwrap(),
    //                     ),
    //                     (
    //                         z1[2..].parse::<isize>().unwrap(),
    //                         z2.parse::<isize>().unwrap(),
    //                     ),
    //                 ),
    //             )
    //         })
    //         .collect();

    //     fn intersection(left: &Cube, right: &Cube) -> Option<Cube> {
    //         let c = (
    //             (left.0 .0.max(right.0 .0), left.0 .1.min(right.0 .1)),
    //             (left.1 .0.max(right.1 .0), left.1 .1.min(right.1 .1)),
    //             (left.2 .0.max(right.2 .0), left.2 .1.min(right.2 .1)),
    //         );
    //         if c.0 .0 <= c.0 .1 && c.1 .0 <= c.1 .1 && c.2 .0 <= c.2 .1 {
    //             Some(c)
    //         } else {
    //             None
    //         }
    //     }

    //     let mut v = Vec::<ToggleCube>::new();
    //     for (toggle, cube) in lines.iter() {
    //         let mut add = Vec::new();
    //         if *toggle {
    //             add.push(ToggleCube {
    //                 toggle: true,
    //                 cube: *cube,
    //             });
    //         }
    //         for tc in v.iter() {
    //             if let Some(ic) = intersection(cube, &tc.cube) {
    //                 add.push(ToggleCube {
    //                     toggle: !tc.toggle,
    //                     cube: ic,
    //                 });
    //             }
    //         }
    //         v.extend(add);
    //     }
    //     v.iter()
    //         .map(|tc| {
    //             let sign = if tc.toggle { 1 } else { -1 };
    //             let (x, y, z) = tc.cube;
    //             sign * ((x.1 - x.0) as isize + 1)
    //                 * (((y.1 - y.0) as isize) + 1)
    //                 * ((z.1 - z.0) as isize + 1)
    //         })
    //         .sum::<isize>() as i64
    // }

    pub fn part_two(&self) -> i64 {
        // my solution ran out of memory for p2, so I used https://github.com/AxlLind/AdventOfCode2021/blob/main/src/bin/22.rs
        use itertools::Itertools;
        use std::cmp::{max, min};
        type Cube = (bool, (i64, i64), (i64, i64), (i64, i64));

        let input = std::fs::read_to_string("./input.txt").unwrap();
        let cubes = input
            .lines()
            .map(|l| {
                let (on, rest) = l.split_once(' ').unwrap();
                let (x, y, z) = rest
                    .split(',')
                    .map(|s| {
                        s[2..]
                            .split("..")
                            .map(|x| x.parse().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect_tuple()
                    .unwrap();
                (on == "on", x, y, z)
            })
            .collect::<Vec<_>>();

        fn volume((_, (x0, x1), (y0, y1), (z0, z1)): Cube) -> i64 {
            (x1 - x0 + 1) * (y1 - y0 + 1) * (z1 - z0 + 1)
        }

        fn subaxis((a, b): (i64, i64), (low, high): (i64, i64)) -> Option<(i64, i64)> {
            if b < low {
                return None;
            }
            if a > high {
                return None;
            }
            let a = min(max(a, low), high);
            let b = min(max(b, low), high);
            Some((a, b))
        }

        fn subcube(c1: Cube, c2: Cube) -> Option<Cube> {
            let xr = subaxis(c1.1, c2.1)?;
            let yr = subaxis(c1.2, c2.2)?;
            let zr = subaxis(c1.3, c2.3)?;
            Some((c1.0, xr, yr, zr))
        }

        fn corrected_volume(c: Cube, rest: &[Cube]) -> i64 {
            let subcubes = rest
                .iter()
                .filter_map(|&c2| subcube(c2, c))
                .collect::<Vec<_>>();
            let vsubcubes = (0..subcubes.len())
                .map(|i| corrected_volume(subcubes[i], &subcubes[i + 1..]))
                .sum::<i64>();
            volume(c) - vsubcubes
        }

        fn total_volume(cubes: &[Cube]) -> i64 {
            (0..cubes.len())
                .filter(|&i| cubes[i].0)
                .map(|i| corrected_volume(cubes[i], &cubes[i + 1..]))
                .sum()
        }

        total_volume(&cubes)
    }
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let instructions = input
            .trim()
            .lines()
            .map(|line| {
                let (kind, rest) = line.split_once(" ").unwrap();
                let axes: Vec<Vec<i32>> = rest
                    .split(",")
                    .map(|axis| {
                        axis[2..]
                            .split("..")
                            .map(|num| num.parse().unwrap())
                            .collect()
                    })
                    .collect();

                let range = Range {
                    x_min: axes[0][0],
                    x_max: axes[0][1],
                    y_min: axes[1][0],
                    y_max: axes[1][1],
                    z_min: axes[2][0],
                    z_max: axes[2][1],
                };

                match kind {
                    "on" => Instruction::On(range),
                    "off" => Instruction::Off(range),
                    _ => unreachable!("invalid input"),
                }
            })
            .collect();

        Ok(Self { instructions })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 590784);
    }

    #[ignore]
    #[test]
    fn part_two() {
        let input = "Player 1 starting position: 4
Player 2 starting position: 8";

        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 444356092776315);
    }
}
