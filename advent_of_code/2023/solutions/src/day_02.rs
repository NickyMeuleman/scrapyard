use std::fmt::Display;

use aoc_core::Solution;
// use aoc_core::AoCError;
// use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

// struct Game {
//     id: usize,
//     draws: Vec<Draw>,
// }

// struct Draw {
//     red: u32,
//     green: u32,
//     blue: u32,
// }

// impl Draw {
//     fn try_new(s: &str) -> AoCResult<Draw> {
//         s.split(", ").try_fold(
//             Draw {
//                 red: 0,
//                 green: 0,
//                 blue: 0,
//             },
//             |mut acc, item| {
//                 let (num, color) = item
//                     .split_once(" ")
//                     .ok_or(AoCError::Parsing)?;
//                 let num = num
//                     .parse()
//                     .map_err(|_| AoCError::Parsing)?;
//                 match color {
//                     "red" => acc.red = num,
//                     "green" => acc.green = num,
//                     "blue" => acc.blue = num,
//                     _ => panic!("at the disco"),
//                 };
//                 Ok(acc)
//             },
//         )
//     }
// }

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    // faster
    fn part_1(&self) -> AoCResult<impl Display> {
        let mut sum = 0;
        'game: for (idx, line) in self.0.lines().enumerate() {
            let id = idx + 1;
            let (_, draws) = line.split_once(": ").unwrap();
            for draw in draws.split("; ") {
                for pair in draw.split(", ") {
                    let (num, color) = pair.split_once(" ").unwrap();
                    let num: u32 = num.parse().unwrap();
                    let possible = match color {
                        "red" => num <= 12,
                        "green" => num <= 13,
                        "blue" => num <= 14,
                        _ => panic!("at the disco"),
                    };
                    if !possible {
                        // a check failed, move on to next game
                        continue 'game;
                    }
                }
            }
            // all checks passed in this game, add to sum
            sum += id;
        }

        Ok(sum)
    }

    // slower, propagates errors in an iterator chain
    // uncomment structs and imports too to use this
    // fn part_1(&self) -> AoCResult<impl Display> {
    //     self.0
    //         .lines()
    //         .enumerate()
    //         .map(|(idx, line)| {
    //             let (_, draws) = line
    //                 .split_once(": ")
    //                 .ok_or(AoCError::Parsing)?;
    //             let draws = draws
    //                 .split("; ")
    //                 .map(Draw::try_new)
    //                 .collect::<AoCResult<Vec<Draw>>>()?;
    //             Ok(Game { id: idx + 1, draws })
    //         })
    //         .filter_ok(|game| {
    //             game.draws
    //                 .iter()
    //                 .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
    //         })
    //         .map_ok(|game| game.id)
    //         .sum::<AoCResult<usize>>()
    // }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut sum = 0;

        for line in self.0.lines() {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            let (_, draws) = line.split_once(": ").unwrap();
            for draw in draws.split("; ") {
                for pair in draw.split(", ") {
                    let (num, color) = pair.split_once(" ").unwrap();
                    let num: u32 = num.parse().unwrap();
                    match color {
                        "red" => min_red = min_red.max(num),
                        "green" => min_green = min_green.max(num),
                        "blue" => min_blue = min_blue.max(num),
                        _ => panic!("at the disco"),
                    }
                }
            }
            sum += min_red * min_green * min_blue;
        }

        Ok(sum)
    }

    fn solve(self) -> AoCResult<aoc_core::Solution>
    where
        Self: Sized,
    {
        let mut part_1 = 0;
        let mut part_2 = 0;

        for (idx, line) in self.0.lines().enumerate() {
            let id = idx + 1;
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            let mut possible_game = true;

            let (_, draws) = line.split_once(": ").unwrap();
            for draw in draws.split("; ") {
                for pair in draw.split(", ") {
                    let (num, color) = pair.split_once(" ").unwrap();
                    let num: u32 = num.parse().unwrap();
                    let possible_draw = match color {
                        "red" => {
                            min_red = min_red.max(num);
                            num <= 12
                        }
                        "green" => {
                            min_green = min_green.max(num);
                            num <= 13
                        }
                        "blue" => {
                            min_blue = min_blue.max(num);
                            num <= 14
                        }
                        _ => panic!("at the disco"),
                    };
                    if !possible_draw {
                        possible_game = false;
                    }
                }
            }
            // if game is still possible at the end of a line, add it to the part_1 sum
            if possible_game {
                part_1 += id;
            }
            let power = min_red * min_green * min_blue;
            part_2 += power;
        }

        Ok(Solution {
            part1: Box::new(part_1),
            part2: Box::new(part_2),
        })
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
        assert_eq!(result, "");
    }

    #[test]
    fn part_2() {
        let input = "";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "");
    }
}
