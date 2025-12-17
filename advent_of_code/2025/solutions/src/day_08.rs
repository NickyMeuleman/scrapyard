// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2025-day08/

use crate::{AoCData, AoCError, AoCResult};
use itertools::Itertools;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data(Vec<Point>);

#[derive(Debug, Clone)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl Point {
    fn dist_squared(&self, other: &Point) -> u32 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

fn get_sorted_pairs(points: &[Point]) -> Vec<(usize, usize)> {
    (0..points.len())
        .tuple_combinations()
        .sorted_by_key(|&(a, b)| points[a].dist_squared(&points[b]))
        .collect()
}

fn find(parents: &mut [usize], id: usize) -> usize {
    if id != parents[id] {
        parents[id] = find(parents, parents[id]);
    }
    parents[id]
}

fn merge(parents: &mut [usize], a: usize, b: usize) {
    let a_root = find(parents, a);
    let b_root = find(parents, b);
    if a_root != b_root {
        parents[b_root] = a_root;
    }
}

fn part_1_helper(points: &[Point], wire_count: usize) -> AoCResult<u64> {
    let pairs = get_sorted_pairs(points);

    // vec where parent of an index is its value
    let mut parents: Vec<usize> = (0..points.len()).collect();

    for &(a, b) in pairs.iter().take(wire_count) {
        merge(&mut parents, a, b);
    }

    let mut sizes = vec![0; points.len()];
    for id in 0..points.len() {
        let root_id = find(&mut parents, id);
        sizes[root_id] += 1;
    }

    Ok(sizes
        .iter()
        .sorted()
        .rev()
        .take(3)
        .product::<u64>())
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let res = input
            .lines()
            .map(|line| {
                let mut parts = line.split(',');
                let x = parts
                    .next()
                    .ok_or(AoCError::Parsing)?
                    .parse()?;
                let y = parts
                    .next()
                    .ok_or(AoCError::Parsing)?
                    .parse()?;
                let z = parts
                    .next()
                    .ok_or(AoCError::Parsing)?
                    .parse()?;

                if parts.next().is_some() {
                    return Err(AoCError::Parsing);
                }

                Ok(Point { x, y, z })
            })
            .collect::<AoCResult<Vec<Point>>>()?;
        Ok(Self(res))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        part_1_helper(&self.0, 1_000)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let pairs = get_sorted_pairs(&self.0);

        // vec where parent of an index is its value
        let mut parents: Vec<usize> = (0..self.0.len()).collect();
        let mut count = 0;

        for (a, b) in pairs {
            if find(&mut parents, a) != find(&mut parents, b) {
                count += 1;
                if count == self.0.len() - 1 {
                    return Ok(self.0[a].x * self.0[b].x);
                }
                merge(&mut parents, a, b);
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
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let data = Data::try_new(input).unwrap();
        let result = part_1_helper(&data.0, 10)
            .unwrap()
            .to_string();
        assert_eq!(result, "40");
    }

    #[test]
    fn part_2() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "25272");
    }
}
