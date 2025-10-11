use crate::{intcode::Computer, AoCData, AoCError, AoCResult};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data(Vec<i64>);

const SHIP_SIZE: i64 = 100;

fn is_pulled(memory: &[i64], x: i64, y: i64) -> AoCResult<bool> {
    let mut computer: Computer = Default::default();
    computer.set_memory(memory.to_vec());
    computer.input(x);
    computer.input(y);
    computer.run()?;
    match computer.consume_output() {
        Some(1) => Ok(true),
        Some(0) => Ok(false),
        _ => Err(AoCError::Solving),
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut sum = 0;
        for x in 0..50 {
            for y in 0..50 {
                if is_pulled(&self.0, x, y)? {
                    sum += 1;
                }
            }
        }
        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // Don't rotate the square; it should be aligned to the same axes as the drone grid
        // find deph at which beam is 100 wide, then go 100 higher
        // reason: beam goes to the right, becomes wider as is goes deeper
        // incrementing top works too, but is less efficient because it requires more checks
        let mut left = 0;
        let mut bottom = SHIP_SIZE - 1;
        loop {
            bottom += 1;
            while !is_pulled(&self.0, left, bottom)? {
                left += 1;
            }
            let right = left + (SHIP_SIZE - 1);
            let top = bottom - (SHIP_SIZE - 1);
            if is_pulled(&self.0, right, top)? {
                return Ok(left * 10_000 + top);
            }
        }
    }
}
