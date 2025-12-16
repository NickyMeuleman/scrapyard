// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2025-day10/

use crate::{AoCData, AoCError, AoCResult};
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

#[derive(Debug, Clone)]
pub struct Data(Vec<Machine>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    jolts: Vec<u32>,
}

impl Machine {
    fn min_presses_jolts_2(&self) -> u32 {
        use good_lp::*;

        // create variable per button (number of times it got pressed)
        let mut vars = variables!();
        let presses: Vec<Variable> = (0..self.buttons.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect();

        // minimize total presses
        let total_presses: Expression = presses.iter().sum();
        let mut problem = vars
            .minimise(total_presses)
            .using(default_solver);

        // for each jolt counter, sum of relevant presses must equal the target joltage
        for (jolt_idx, &target) in self.jolts.iter().enumerate() {
            let mut expr = Expression::from(0.0);

            for (btn_idx, relevant_idxs) in self.buttons.iter().enumerate() {
                // if button is relevant, add its press variable to the constraint
                if relevant_idxs.contains(&jolt_idx) {
                    expr += presses[btn_idx];
                }
            }

            // sum of relevant presses == target joltage
            problem.add_constraint(expr.eq(target as f64));
        }

        let solution = problem.solve().unwrap();

        presses
            .iter()
            .map(|v| solution.value(*v).round() as u32)
            .sum()
    }

    fn min_presses_lights(&self) -> usize {
        let goal: u32 = self
            .lights
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &on)| if on { acc | (1 << i) } else { acc });
        let buttons: Vec<u32> = self
            .buttons
            .iter()
            .map(|idxs| {
                idxs.iter()
                    .fold(0, |acc, &idx| acc | (1 << idx))
            })
            .collect();

        // queue of (curr_lights, num_presses)
        let mut q = VecDeque::from([(0, 0)]);
        let mut seen = HashSet::from([0]);

        while let Some((lights, presses)) = q.pop_front() {
            if lights == goal {
                return presses;
            }
            for button in &buttons {
                let next = lights ^ button;
                if seen.insert(next) {
                    q.push_back((next, presses + 1));
                }
            }
        }

        0
    }

    // fn min_presses_jolts(&self) -> u32 {
    //     use good_lp::*;
    //
    //     let mut vars = variables!();
    //
    //     // create integer variable per button (number of times it got pressed)
    //     let presses: Vec<_> = (0..self.buttons.len())
    //         .map(|_| vars.add(variable().min(0).integer()))
    //         .collect();
    //
    //     // goal: minimize total presses
    //     let total_presses: Expression = presses.iter().sum();
    //     let mut problem = highs(vars.minimise(total_presses));
    //
    //     // constraints: for each jolt counter, sum of relevant presses must equal the target joltage
    //     for (jolt_idx, &target_joltage) in self.jolts.iter().enumerate() {
    //         let mut constraint = Expression::from(0.0);
    //
    //         for (btn_idx, relevant_idxs) in self.buttons.iter().enumerate() {
    //             // if button is relevant, add its press variable to the constraint
    //             if relevant_idxs.contains(&jolt_idx) {
    //                 constraint += presses[btn_idx];
    //             }
    //         }
    //
    //         // sum of relevant presses == target joltage
    //         problem.add_constraint(constraint.eq(target_joltage as f64));
    //     }
    //
    //     let solution = problem.solve().unwrap();
    //
    //     presses
    //         .iter()
    //         .map(|&v| solution.value(v).round() as u32)
    //         .sum()
    // }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .lines()
                .map(|line| {
                    let (lights_str, rest) = line
                        .split_once(' ')
                        .ok_or(AoCError::Parsing)?;
                    let (buttons_str, jolts_str) = rest
                        .rsplit_once(' ')
                        .ok_or(AoCError::Parsing)?;

                    let lights = lights_str
                        .trim_matches(['[', ']'])
                        .chars()
                        .map(|c| c == '#')
                        .collect();

                    let buttons = buttons_str
                        .split(' ')
                        .map(|s| {
                            s.trim_matches(['(', ')'])
                                .split(',')
                                .map(|s| s.parse().map_err(|_| AoCError::Parsing))
                                .collect::<AoCResult<Vec<_>>>()
                        })
                        .collect::<AoCResult<Vec<_>>>()?;

                    let jolts = jolts_str
                        .trim_matches(['{', '}'])
                        .split(',')
                        .map(|s| s.parse().map_err(|_| AoCError::Parsing))
                        .collect::<AoCResult<Vec<_>>>()?;

                    Ok(Machine {
                        lights,
                        buttons,
                        jolts,
                    })
                })
                .collect::<AoCResult<_>>()?,
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(self
            .0
            .iter()
            .map(|m| m.min_presses_lights())
            .sum::<usize>())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(self
            .0
            .iter()
            .map(|m| m.min_presses_jolts_2())
            .sum::<u32>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "7");
    }

    #[test]
    fn part_2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "33");
    }
}
