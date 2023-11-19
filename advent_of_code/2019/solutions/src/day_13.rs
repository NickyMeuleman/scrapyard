use std::{collections::HashSet, fmt::Display};

use crate::{
    intcode::{Computer, Status},
    AoCData, AoCResult,
};
use aoc_core::AoCError;

#[derive(Debug, Clone)]
pub struct Data(Vec<i64>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .split(",")
                .filter_map(|s| s.parse().ok())
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut blocks: HashSet<(i64, i64)> = HashSet::new();
        let mut computer: Computer = Default::default();
        computer.set_memory(self.0.clone());
        while let Ok(status) = computer.operate() {
            match status {
                Status::Halted => break,
                _ => {
                    if computer.outputs.len() == 3 {
                        let x = computer
                            .consume_output()
                            .ok_or(AoCError::Solving)?;
                        let y = computer
                            .consume_output()
                            .ok_or(AoCError::Solving)?;
                        let id = computer
                            .consume_output()
                            .ok_or(AoCError::Solving)?;
                        if id == 2 {
                            blocks.insert((x, y));
                        }
                    }
                }
            }
        }

        Ok(blocks.len())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut computer: Computer = Default::default();
        computer.set_memory(self.0.clone());
        computer.memory[0] = 2;

        let mut score = 0;
        let mut ball_x = -1;
        let mut paddle_x = -1;

        loop {
            let end_status = computer.run()?;

            while let (Some(x), Some(y), Some(third)) = (
                computer.consume_output(),
                computer.consume_output(),
                computer.consume_output(),
            ) {
                if x == -1 && y == 0 {
                    // update score
                    score = third;
                } else {
                    match third {
                        // update paddle
                        3 => paddle_x = x,
                        // update ball
                        4 => ball_x = x,
                        // do nothing
                        _ => {}
                    }
                }
            }
            // // why is `for &[x, y, third] in computer` not allowed because of a refutable pattern? Isn't the use of chunks_exact(3) a guarantee?
            // for chunk in computer
            //     .outputs
            //     .make_contiguous()
            //     .chunks_exact(3)
            // {
            //     let (x, y, third) = (chunk[0], chunk[1], chunk[2]);
            //     if x == -1 && y == 0 {
            //         score = third;
            //     } else {
            //         match third {
            //             3 => paddle_x = x,
            //             4 => ball_x = x,
            //             _ => {}
            //         }
            //     }
            // }

            // break AFTER potentially updating the score
            if end_status == Status::Halted {
                break;
            }

            // if using the method that uses the entire outputs vecdeque at once, remember to clear it
            // computer.outputs.clear();
            // keep paddle under the ball at all times
            let input = match ball_x.cmp(&paddle_x) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            computer.input(input);
        }

        Ok(score)
    }
}
