use std::{collections::HashSet, fmt::Display, str::FromStr};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(HashSet<Star>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("<")
            .unwrap()
            .strip_suffix(">")
            .unwrap();
        let (x, y) = s.split_once(", ").unwrap();

        let coord = Coord {
            x: x.trim().parse().unwrap(),
            y: y.trim().parse().unwrap(),
        };

        Ok(coord)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Star {
    id: usize,
    position: Coord,
    velocity: Coord,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .lines()
                .enumerate()
                .map(|(idx, line)| {
                    let (position, velocity) = line.split_once(" velocity=").unwrap();
                    let position = position
                        .strip_prefix("position=")
                        .unwrap();
                    Star {
                        id: idx,
                        position: position.parse().unwrap(),
                        velocity: velocity.parse().unwrap(),
                    }
                })
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut stars = self.0.clone();
        let mut new_stars = stars.clone();
        let (_minmax_x, minmax_y) = stars
            .iter()
            .map(|star| star.position)
            .fold(
                ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
                |(minmax_x, minmax_y), Coord { x, y }| {
                    (
                        (minmax_x.0.min(x), minmax_x.1.max(x)),
                        (minmax_y.0.min(y), minmax_y.1.max(y)),
                    )
                },
            );
        let mut size = minmax_y.1.abs_diff(minmax_y.0);
        let mut new_size;

        for _ in 0.. {
            new_stars.clear();
            // println!("After {} seconds", i);
            // show(&stars.iter().map(|star| star.position).collect());
            // println!();
            for star in stars.iter() {
                new_stars.insert(Star {
                    id: star.id,
                    position: Coord {
                        x: star.position.x + star.velocity.x,
                        y: star.position.y + star.velocity.y,
                    },
                    velocity: star.velocity,
                });
            }
            let (_minmax_x, minmax_y) = new_stars
                .iter()
                .map(|star| star.position)
                .fold(
                    ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
                    |(minmax_x, minmax_y), Coord { x, y }| {
                        (
                            (minmax_x.0.min(x), minmax_x.1.max(x)),
                            (minmax_y.0.min(y), minmax_y.1.max(y)),
                        )
                    },
                );
            new_size = minmax_y.1.abs_diff(minmax_y.0);
            if new_size > size {
                break;
            }
            // double buffer
            std::mem::swap(&mut stars, &mut new_stars);
            std::mem::swap(&mut size, &mut new_size);
        }

        // TODO: the OCR thingy I did for other years
        let stars: HashSet<Coord> = stars
            .iter()
            .map(|star| star.position)
            .collect();
        let mut result = String::new();

        let (minmax_x, minmax_y) = stars.iter().fold(
            ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
            |(minmax_x, minmax_y), Coord { x, y }| {
                (
                    (minmax_x.0.min(*x), minmax_x.1.max(*x)),
                    (minmax_y.0.min(*y), minmax_y.1.max(*y)),
                )
            },
        );
        for y in minmax_y.0..=minmax_y.1 {
            for x in minmax_x.0..=minmax_x.1 {
                let coord = Coord { x, y };
                let dot = if stars.contains(&coord) { '#' } else { '.' };
                result.push(dot);
            }
            result.push('\n');
        }

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut stars = self.0.clone();
        let mut new_stars = stars.clone();
        let (_minmax_x, minmax_y) = stars
            .iter()
            .map(|star| star.position)
            .fold(
                ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
                |(minmax_x, minmax_y), Coord { x, y }| {
                    (
                        (minmax_x.0.min(x), minmax_x.1.max(x)),
                        (minmax_y.0.min(y), minmax_y.1.max(y)),
                    )
                },
            );
        let mut size = minmax_y.1 - minmax_y.0;
        let mut new_size;

        for i in 0.. {
            new_stars.clear();
            // println!("After {} seconds", i);
            // show(&stars.iter().map(|star| star.position).collect());
            // println!();
            for star in stars.iter() {
                new_stars.insert(Star {
                    id: star.id,
                    position: Coord {
                        x: star.position.x + star.velocity.x,
                        y: star.position.y + star.velocity.y,
                    },
                    velocity: star.velocity,
                });
            }
            let (_minmax_x, minmax_y) = new_stars
                .iter()
                .map(|star| star.position)
                .fold(
                    ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
                    |(minmax_x, minmax_y), Coord { x, y }| {
                        (
                            (minmax_x.0.min(x), minmax_x.1.max(x)),
                            (minmax_y.0.min(y), minmax_y.1.max(y)),
                        )
                    },
                );
            new_size = minmax_y.1 - minmax_y.0;
            if new_size > size {
                return Ok(i);
            }
            // double buffer
            std::mem::swap(&mut stars, &mut new_stars);
            std::mem::swap(&mut size, &mut new_size);
        }

        Ok(stars.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(
            result,
            "#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
"
        );
    }

    #[test]
    fn part_2() {
        let input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "3");
    }
}
