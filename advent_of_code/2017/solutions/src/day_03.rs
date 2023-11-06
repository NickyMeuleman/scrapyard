use std::{collections::HashMap, fmt::Display};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(i64);

struct CoordinateMap {
    x: i64,
    y: i64,
    values: HashMap<(i64, i64), u64>,
}

impl CoordinateMap {
    fn new() -> CoordinateMap {
        CoordinateMap {
            x: 0,
            y: 0,
            values: [((0, 0), 1)].iter().cloned().collect(),
        }
    }

    fn move_by(&mut self, x: i64, y: i64) {
        self.x += x;
        self.y += y;
    }

    fn get(&self) -> u64 {
        self.get_at(self.x, self.y)
    }

    fn get_at(&self, x: i64, y: i64) -> u64 {
        match self.values.get(&(x, y)) {
            Some(val) => *val,
            None => 0,
        }
    }

    fn set(&mut self, val: u64) {
        self.values
            .insert((self.x, self.y), val);
    }

    fn set_to_surrounding_sum(&mut self) {
        let sum: u64 = (-1..2)
            .map::<u64, _>(|x| {
                (-1..2)
                    .filter(|y| (x, *y) != (0, 0))
                    .map(|y| self.get_at(self.x + x, self.y + y))
                    .sum()
            })
            .sum();
        self.set(sum)
    }

    fn check_and_print_if_greq(&self, num: u64) -> Option<u64> {
        let val = self.get();
        let greater = val >= num;
        if greater {
            Some(val)
        } else {
            None
        }
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(input.trim().parse()?))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let ring = ((((self.0 - 1) as f64).sqrt() - 1.0) / 2.0 + 1.0) as i64;
        let first = (1 + 2 * (ring - 1)).pow(2) + 1;
        let ring_size = ring * 8;
        let pos_on_side = (self.0 - first) % (ring_size / 4);

        Ok((pos_on_side - ring + 1).abs() + ring)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut map = CoordinateMap::new();
        let mut move_distance = 1;
        let mut dir = (1, 0);
        loop {
            for _ in 0..2 {
                let (x, y) = dir;
                for _ in 0..move_distance {
                    if let Some(n) = map.check_and_print_if_greq(
                        self.0
                            .try_into()
                            .map_err(|_| AoCError::Solving)?,
                    ) {
                        return Ok(n);
                    }
                    map.move_by(x, y);
                    map.set_to_surrounding_sum();
                }
                dir = (-y, x);
            }
            move_distance += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1024";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "31");
    }

    #[test]
    fn part_2() {
        let input = "5";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "5");
    }
}
