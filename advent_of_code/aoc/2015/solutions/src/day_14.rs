use std::{collections::HashMap, fmt::Display};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Reindeer>);

#[derive(Debug, Clone)]
struct Reindeer {
    name: String,
    speed: u32,     // in km/s
    move_time: u32, // in s
    rest_time: u32, // in s
}

impl Reindeer {
    fn dist_at(&self, time: u32) -> u32 {
        let cycle_time = self.move_time + self.rest_time;
        let full_cycles = time / cycle_time;
        let remain_sec = time - (cycle_time * full_cycles);
        if remain_sec > self.move_time {
            (full_cycles + 1) * self.move_time * self.speed
        } else {
            (self.speed * self.move_time * full_cycles) + (remain_sec * self.speed)
        }
    }
}

fn part_1_helper(deer: &[Reindeer], time: u32) -> u32 {
    deer.iter()
        .map(|deer| deer.dist_at(time))
        .max()
        .unwrap_or(0)
}

fn part_2_helper(deer: &[Reindeer], time: u32) -> u32 {
    let mut points: HashMap<String, u32> = deer
        .iter()
        .map(|deer| (deer.name.clone(), 0))
        .collect();
    for elapsed in 1..=time {
        let max = deer
            .iter()
            .map(|deer| deer.dist_at(elapsed))
            .max()
            .unwrap_or(0);
        let winners = deer
            .iter()
            .filter(|deer| deer.dist_at(elapsed) == max)
            .map(|deer| deer.name.clone());
        for winner in winners {
            points
                .entry(winner)
                .and_modify(|val| *val += 1)
                .or_insert(1);
        }
    }

    *points.values().max().unwrap()
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut reindeer = Vec::new();
        for line in input.lines() {
            let (left, right) = line
                .split_once(" seconds, but then must rest for ")
                .ok_or(AoCError::Parsing)?;
            let rest_time = right
                .strip_suffix(" seconds.")
                .ok_or(AoCError::Parsing)?;
            let (name, right) = left
                .split_once(" can fly ")
                .ok_or(AoCError::Parsing)?;
            let (speed, move_time) = right
                .split_once(" km/s for ")
                .ok_or(AoCError::Parsing)?;

            reindeer.push(Reindeer {
                name: name.to_string(),
                speed: speed.parse()?,
                move_time: move_time.parse()?,
                rest_time: rest_time.parse()?,
            })
        }

        Ok(Self(reindeer))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(part_1_helper(&self.0, 2503))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(part_2_helper(&self.0, 2503))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let data = Data::try_new(input).unwrap();
        assert_eq!(part_1_helper(&data.0, 1000), 1120);
    }

    #[test]
    fn part_2() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let data = Data::try_new(input).unwrap();
        assert_eq!(part_2_helper(&data.0, 1000), 689);
    }
}
