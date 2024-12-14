// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2024-day14/

use crate::{AoCData, AoCResult};
use aoc_core::AoCError;
use itertools::Itertools;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: i64,
    col: i64,
}

impl Point {
    fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Robot {
    pos: Point,
    vel: Point,
}

impl Robot {
    fn new(p_row: i64, p_col: i64, v_row: i64, v_col: i64) -> Self {
        Self {
            pos: Point::new(p_row, p_col),
            vel: Point::new(v_row, v_col),
        }
    }

    fn run(&mut self, steps: i64, rows: i64, cols: i64) {
        self.pos.row = (self.pos.row + self.vel.row * steps).rem_euclid(rows);
        self.pos.col = (self.pos.col + self.vel.col * steps).rem_euclid(cols);
    }
}

const ROWS: i64 = 103;
const COLS: i64 = 101;

fn safety(robots: &[Robot], rows: i64, cols: i64) -> usize {
    let mut sectors = [0; 4];
    let col_mid = cols / 2;
    let row_mid = rows / 2;
    for &Robot { pos, .. } in robots {
        if pos.row == row_mid || pos.col == col_mid {
            continue;
        }
        let top = pos.row < row_mid;
        let left = pos.col < col_mid;
        match (top, left) {
            (true, true) => sectors[0] += 1,
            (true, false) => sectors[1] += 1,
            (false, true) => sectors[2] += 1,
            (false, false) => sectors[3] += 1,
        }
    }
    sectors.iter().product()
}

// fn show(robots: &[Robot], rows: i64, cols: i64) {
//     for r in 0..rows {
//         for c in 0..cols {
//             let pos = Point::new(r, c);
//             if robots
//                 .iter()
//                 .map(|robot| robot.pos)
//                 .contains(&pos)
//             {
//                 print!("■")
//             } else {
//                 print!(" ")
//             }
//         }
//         println!();
//     }
// }

#[derive(Debug, Clone)]
pub struct Data(Vec<Robot>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .tuples()
                .map(|(px, py, vx, vy)| Robot::new(py, px, vy, vx))
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut robots = self.0.clone();
        // for Robot { pos, vel } in &mut robots {
        //     // pos.row = (((pos.row + vel.row * 100) % ROWS) + ROWS) % ROWS;
        //     // pos.col = (((pos.col + vel.col * 100) % COLS) + COLS) % COLS;
        //     // or
        //     pos.row = (pos.row + vel.row * 100).rem_euclid(ROWS);
        //     pos.col = (pos.col + vel.col * 100).rem_euclid(COLS);
        // }
        for robot in &mut robots {
            robot.run(100, ROWS, COLS);
        }
        Ok(safety(&robots, ROWS, COLS))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut robots = self.0.clone();
        for i in 0.. {
            // for Robot { pos, vel } in &mut robots {
            //     pos.row = (pos.row + vel.row).rem_euclid(ROWS);
            //     pos.col = (pos.col + vel.col).rem_euclid(COLS);
            // }
            for robot in &mut robots {
                robot.run(1, ROWS, COLS);
            }
            if robots
                .iter()
                .map(|robot| robot.pos)
                .all_unique()
            {
                // show(&robots, ROWS, COLS);
                return Ok(i + 1);
            }
        }
        Err(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let data = Data::try_new(input).unwrap();
        let mut robots = data.0.clone();

        let rows = 7;
        let cols = 11;

        for _ in 0..100 {
            for Robot { pos, vel } in &mut robots {
                pos.row = (pos.row + vel.row).rem_euclid(rows);
                pos.col = (pos.col + vel.col).rem_euclid(cols);
            }
        }

        let result = safety(&robots, rows, cols);
        assert_eq!(result, 12);
    }
}
