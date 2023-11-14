use std::{
    f64::consts::{FRAC_PI_2, PI},
    fmt::Display,
};

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<(i64, i64)>);

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn angle(row: i64, col: i64) -> f64 {
    // y.atan2(x)
    let result = (row as f64).atan2(col as f64);
    if result < 0.0 {
        result + 2.0 * PI
    } else {
        result
    }
}

fn mag(row: i64, col: i64) -> i64 {
    row * row + col * col
}

// vaporizes n asteroids around base position
// returns last asteroid vaporized
fn vaporize(asteroids: &[(i64, i64)], base_col: i64, base_row: i64, n: usize) -> (i64, i64) {
    let mut list = Vec::new();
    for (curr_col, curr_row) in asteroids.iter() {
        if *curr_col == base_col && *curr_row == base_row {
            continue;
        }

        let diff_col = *curr_col - base_col;
        // because 0,0 is top left instead of bottom left,
        // reverse the y values. seems hacky: any better way to do this?
        let diff_row = base_row - *curr_row;
        list.push((
            angle(diff_row, diff_col),
            mag(diff_row, diff_col),
            (curr_col, curr_row),
        ));
    }

    list.sort_by(|a, b| {
        // sort by reverse angle (as we want to process angles in a clockwise order)
        // then magnitude if same angle
        // partial_cmp is safe here as atan2 can't return nan/infinity
        a.0.partial_cmp(&b.0)
            .unwrap()
            .reverse()
            .then(a.1.cmp(&b.1))
    });

    let mut removed = Vec::with_capacity(n);
    // find first occurence of lower than PI/2, otherwise use first element
    for (idx, v) in list.iter().enumerate() {
        if v.0 <= FRAC_PI_2 {
            removed.push(list.remove(idx));
            break;
        }
    }
    if let None = removed.last() {
        removed.push(list.remove(0));
    }

    'outer: for _ in 1..n {
        for (idx, v) in list.iter().enumerate() {
            if v.0 < removed.last().unwrap().0 {
                removed.push(list.remove(idx));
                continue 'outer;
            }
        }
        // reached end of list without removing, remove first element
        removed.push(list.remove(0));
    }

    removed
        .last()
        .map(|(_, _, (&col, &row))| (col, row))
        .unwrap()
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut points = Vec::new();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    points.push((col as i64, row as i64));
                }
            }
        }
        Ok(Self(points))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        self.0
            .iter()
            .map(|curr| {
                self.0
                    .iter()
                    .filter(|other| *other != curr)
                    .map(|other| {
                        // reduce point to closest one that curr can see
                        // sign matters or you filter out opposite angles
                        let delta_col = other.0 as i64 - curr.0 as i64;
                        let delta_row = other.1 as i64 - curr.1 as i64;
                        let gcd = gcd(delta_col.abs(), delta_row.abs());
                        (delta_col / gcd, delta_row / gcd)
                    })
                    .unique()
                    .count()
            })
            .max()
            .ok_or(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // merged my solution with https://github.com/prscoelho/aoc2019/blob/master/src/aoc10/mod.rs
        let base = self
            .0
            .iter()
            .max_by_key(|curr| {
                self.0
                    .iter()
                    .filter(|other| other != curr)
                    .map(|other| {
                        // reduce point to closest one that curr can see
                        // sign matters or you filter out opposite angles
                        let delta_col = other.0 as i64 - curr.0 as i64;
                        let delta_row = other.1 as i64 - curr.1 as i64;
                        let gcd = gcd(delta_col.abs(), delta_row.abs());
                        (delta_col / gcd, delta_row / gcd)
                    })
                    .unique()
                    .count()
            })
            .ok_or(AoCError::Solving)?;

        let last = vaporize(&self.0, base.0, base.1, 200);

        Ok(last.0 * 100 + last.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "210");
    }

    #[test]
    fn part_2() {
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "st");
    }
}
