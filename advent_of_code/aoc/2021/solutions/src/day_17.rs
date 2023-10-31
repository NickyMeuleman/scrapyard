use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let (x_range, y_range) = input
            .trim()
            .strip_prefix("target area: ")
            .ok_or(AoCError::Parsing)?
            .split_once(", ")
            .ok_or(AoCError::Parsing)?;
        let (x_min, x_max) = x_range
            .strip_prefix("x=")
            .ok_or(AoCError::Parsing)?
            .split_once("..")
            .ok_or(AoCError::Parsing)?;
        let (y_min, y_max) = y_range
            .strip_prefix("y=")
            .ok_or(AoCError::Parsing)?
            .split_once("..")
            .ok_or(AoCError::Parsing)?;
        let x_min = x_min.parse()?;
        let x_max = x_max.parse()?;
        let y_min = y_min.parse()?;
        let y_max = y_max.parse()?;

        Ok(Self {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        // So this is the sum of an arithmetic series again
        // https://gist.github.com/mdarrik/72835482b47e9b3e2827faa5789f8e6a#file-aoc-2021-day-17-math-explanation-part-1-md
        // .............#....#............
        // .......#..............#........
        // ...............................
        // S------------------------#----- velocity to reach here is -intial_y_velocity
        // ...............................
        // ...............................
        // ...........................#...

        // We know then that the first y position where y < 0 is y = -initial_y_velocity - 1
        // The biggest that step can be then is min(target_y)
        // -intial_y_velocity - 1 = min(target_y)
        // Written differently initial_y_velocity = |min(target_y)| - 1
        // the maximum height is reached right before the projectile starts going down:
        // intial_y_velocity + (intial_y_velocity - 1) + (intial_y_velocity - 2) + ... + 2 + 1 + 0
        // reordered that's the sum of the arithmetic series starting at 1 and ending at intial_y_velocity
        let first = 1;
        let last = self.y_min;
        let steps = self.y_min;
        let result = steps * (first + last) / 2;

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // no fancy math solution here, brute force it with clever choosing of bounds (not calculating impossible options)

        // the initial y velocity has to be higher or equal to the target y_min, or we'd shoot under the target immediately
        // the initial y velocity can't be higher than |min_target_y| - 1. (same logic as part1)
        let result = (self.y_min..=(self.y_min.abs() - 1))
            // create an iterator of every possible starting x velocity and y velocity
            // the initial x velocity has to be higher than 0 because we can't shoot backwards
            // the initial x velocity can't be higher than x_max or we'd overshoot the target immediately horizontally
            .flat_map(|vy| (1..=self.x_max).map(move |vx| (vx, vy)))
            // calculate points with those starting values, if a point is inside the target, keep it
            .filter(|(vx, vy)| {
                let (mut vx, mut vy) = (*vx, *vy);
                let (mut x, mut y) = (0, 0);
                while x <= self.x_max && y >= self.y_min {
                    if x >= self.x_min && y <= self.y_max {
                        // point inside target zone
                        return true;
                    }
                    // update positions and velocities
                    x += vx;
                    y += vy;
                    vx = (vx - 1).max(0);
                    vy -= 1;
                }
                false
            })
            .count();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "target area: x=20..30, y=-10..-5";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "45");
    }

    #[test]
    fn part_2() {
        let input = "target area: x=20..30, y=-10..-5";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "112");
    }
}
