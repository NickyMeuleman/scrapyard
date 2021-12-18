use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone)]
pub struct Data {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Data {
    pub fn part_one(&self) -> i32 {
        // So this is the sum of an arithmetic series again
        // sum_of_series = num_steps * (beginning_in_series + last_in_series) / 2
        // num_steps is y_min, beginning is 1, last is y_min
        (self.y_min * (1 + self.y_min)) / 2
    }

    pub fn part_two(&self) -> usize {
        // no fancy math solution here, brute force it with clever choosing of bounds (not calculating impossible options)

        // the initial y velocity has to be higher or equal to the target y_min, or we'd shoot down when the target is up
        // the initial y velocity can't be higher than -y_min or we'd overshoot the target immediately vertically
        (self.y_min..=(-self.y_min))
            // create an iterator of every possible starting x velocity and y velocity
            // the initial x velocity has to be higher than 0 because we can't shoot backwards
            // the initial x velocity can't be higher than x_max or we'd overshoot the target immediately horizontally
            .flat_map(|vy| (1..=self.x_max).map(move |vx| (vx, vy)))
            // calculate points with those starting values, if a point is inside the target, count it
            .filter_map(|(vx, vy)| {
                let (mut vx, mut vy) = (vx, vy);
                let (mut x, mut y) = (0, 0);
                while x <= self.x_max && y >= self.y_min {
                    if x >= self.x_min && y <= self.y_max {
                        // point inside target zone
                        return Some(());
                    }
                    // update positions and velocities
                    x += vx;
                    y += vy;
                    vx = (vx - 1).max(0);
                    vy -= 1;
                }
                None
            })
            .count()
    }
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let vec: Vec<i32> = input
            .trim()
            // get every substring that is seperated by one of these characters 
            .split_terminator(&['=', '.', ','][..])
            // only keep the numbers
            .filter_map(|x| x.parse().ok())
            .collect();

        Ok(Self {
            x_min: vec[0],
            x_max: vec[1],
            y_min: vec[2],
            y_max: vec[3],
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "target area: x=20..30, y=-10..-5";
        let data: Data = input.parse().unwrap();
        dbg!(&data);
        assert_eq!(data.part_one(), 45);
    }

    #[test]
    fn part_two_example() {
        let input = "target area: x=20..30, y=-10..-5";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 112);
    }
}
