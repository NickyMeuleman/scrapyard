use std::collections::HashMap;

use crate::AoCData;

pub struct Data(Vec<Reindeer>);

#[derive(Debug)]
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
    let mut points: HashMap<String, u32> =
            deer.iter().map(|deer| (deer.name.clone(), 0)).collect();
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

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut reindeer = Vec::new();
        for line in input.lines() {
            let (left, right) = line.split_once(" seconds, but then must rest for ")?;
            let rest_time = right.strip_suffix(" seconds.")?;
            let (name, right) = left.split_once(" can fly ")?;
            let (speed, move_time) = right.split_once(" km/s for ")?;

            reindeer.push(Reindeer {
                name: name.to_string(),
                speed: speed.parse().ok()?,
                move_time: move_time.parse().ok()?,
                rest_time: rest_time.parse().ok()?,
            })
        }

        Some(Self(reindeer))
    }

    fn part_1(&self) -> String {
        part_1_helper(&self.0, 2503).to_string()
    }

    fn part_2(&self) -> String {
        part_2_helper(&self.0, 2503).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(14);
        let data = Data::try_new(input).unwrap();
        assert_eq!(part_1_helper(&data.0, 1000), 1120);
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(14);
        let data = Data::try_new(input).unwrap();
        assert_eq!(part_2_helper(&data.0, 1000), 689);
    }
}
