use crate::AoCData;
use std::ops::RangeInclusive;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: i32,
    col: i32,
}

impl Coord {
    fn manhattan(&self, other: &Self) -> i32 {
        (self.row - other.row).abs() + (self.col - other.col).abs()
    }
}

fn beacon_row_range(sensor: &Coord, beacon: &Coord, row: i32) -> Option<RangeInclusive<i32>> {
    let radius = sensor.manhattan(beacon);
    let offset = radius - (sensor.row - row).abs();
    if offset < 0 {
        None
    } else {
        Some(sensor.col - offset..=sensor.col + offset)
    }
}

fn row_ranges(row: i32, pairs: &[[Coord; 2]]) -> Vec<RangeInclusive<i32>> {
    let mut ranges: Vec<_> = pairs
        .iter()
        .flat_map(|pair| beacon_row_range(&pair[0], &pair[1], row))
        .collect();
    ranges.sort_unstable_by_key(|range| *range.start());

    let mut merged_ranges = vec![ranges[0].clone()];
    for next in &ranges[1..] {
        let last_idx = merged_ranges.len() - 1;
        let last = &merged_ranges[last_idx];
        // check if the two sorted ranges overlap
        // create a single bigger range if possible
        if next.start() <= last.end() || last.end() + 1 == *next.start() {
            if next.end() > last.end() {
                let old = &merged_ranges[last_idx];
                let new = *old.start()..=*next.end();
                merged_ranges[last_idx] = new;
            }
        } else {
            merged_ranges.push(next.clone());
        }
    }

    merged_ranges
}

fn part_1_helper(pairs: Vec<[Coord; 2]>, row: i32) -> usize {
    let covered = row_ranges(row, &pairs)
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<i32>() as usize;

    let beacons = pairs
        .into_iter()
        .map(|pair| pair[1])
        .filter(|beacon| beacon.row == row)
        .map(|beacon| beacon.col)
        .dedup()
        .count();

    covered - beacons
}

fn part_2_helper(pairs: Vec<[Coord; 2]>, size: i32) -> i64 {
    let (row, col_ranges) = (0..=size)
        // not needed but faster
        .rev()
        .map(|row| (row, row_ranges(row, &pairs)))
        .find(|(_, ranges)| ranges.len() > 1)
        .unwrap();

    let col = col_ranges.first().unwrap().end() + 1;

    i64::from(col) * 4_000_000 + i64::from(row)
}

pub struct Data(Vec<[Coord; 2]>);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut pairs = Vec::new();
        for line in input.lines() {
            let (sensor, beacon) = line.split_once(": ")?;
            let sensor = sensor.strip_prefix("Sensor at ")?;
            let beacon = beacon.strip_prefix("closest beacon is at ")?;
            let (sx, sy) = sensor.split_once(", ")?;
            let (bx, by) = beacon.split_once(", ")?;
            let sx = sx.strip_prefix("x=")?;
            let sy = sy.strip_prefix("y=")?;
            let bx = bx.strip_prefix("x=")?;
            let by = by.strip_prefix("y=")?;

            let pair = [
                Coord {
                    col: sx.parse().ok()?,
                    row: sy.parse().ok()?,
                },
                Coord {
                    col: bx.parse().ok()?,
                    row: by.parse().ok()?,
                },
            ];

            pairs.push(pair);
        }

        Some(Self(pairs))
    }

    fn part_1(&self) -> String {
        part_1_helper(self.0.clone(), 2_000_000).to_string()
    }

    fn part_2(&self) -> String {
        part_2_helper(self.0.clone(), 4_000_000).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(15);
        let data = Data::try_new(input).unwrap();
        assert_eq!(part_1_helper(data.0, 10), 26);
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(15);
        let data = Data::try_new(input).unwrap();
        assert_eq!(part_2_helper(data.0, 20), 56000011);
    }
}
