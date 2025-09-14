use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: i32,
}

impl Point {
    fn manhattan(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x)
            + self.y.abs_diff(other.y)
            + self.z.abs_diff(other.z)
            + self.t.abs_diff(other.t)
    }
}

fn remove_constellation(points: &mut Vec<Point>) {
    let mut stack = vec![points.pop().unwrap()];
    while let Some(curr) = stack.pop() {
        let neighbours = points.extract_if(.., |p| curr.manhattan(p) <= 3);
        stack.extend(neighbours);
    }
}

// fn remove_constellation(points: &mut Vec<Point>) {
//     let removed = points.pop().unwrap();
//     let mut stack = vec![removed];
//
//     while let Some(curr) = stack.pop() {
//         let mut neighbour_idxs = Vec::new();
//         for (idx, other) in points.iter().enumerate() {
//             if curr.same_constellation(other) {
//                 neighbour_idxs.push(idx);
//             }
//         }
//
//         for &idx in neighbour_idxs.iter().rev() {
//             let removed = points.remove(idx);
//             stack.push(removed);
//         }
//     }
// }

// fn remove_constellation(points: &mut Vec<Point>, start_idx: usize) {
//     let mut stack = vec![points.swap_remove(start_idx)];
//
//     while let Some(curr) = stack.pop() {
//         let mut i = 0;
//         while i < points.len() {
//             if curr.same_constellation(&points[i]) {
//                 let neighbor = points.swap_remove(i);
//                 stack.push(neighbor);
//                 // don't increment i — swapped element needs to be checked
//             } else {
//                 i += 1;
//             }
//         }
//     }
// }

// fn remove_constellation(points: &mut Vec<Point>, point: &Point) {
//     if let Some(pos) = points.iter().position(|p| p == point) {
//         points.remove(pos);
//     } else {
//         return;
//     }
//
//     // Collect neighbors that are still in the list
//     let neighbors: Vec<Point> = points
//         .iter()
//         .filter(|p| point.same_constellation(p))
//         .cloned()
//         .collect();
//
//     for neighbor in neighbors {
//         remove_constellation(points, &neighbor);
//     }
// }

#[derive(Debug, Clone)]
pub struct Data(Vec<Point>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .lines()
                .map(|line| {
                    let nums: Vec<_> = line
                        .split(",")
                        .map(|s| s.parse().unwrap())
                        .collect();
                    Point {
                        x: nums[0],
                        y: nums[1],
                        z: nums[2],
                        t: nums[3],
                    }
                })
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut points = self.0.clone();
        let mut constellations = 0;
        while !points.is_empty() {
            remove_constellation(&mut points);
            constellations += 1;
        }
        Ok(constellations)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok("ho ho ho")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "4");
    }
}
