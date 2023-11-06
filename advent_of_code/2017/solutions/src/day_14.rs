use std::{collections::HashSet, fmt::Display};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn neighbours(&self) -> Vec<Coord> {
        let mut result = Vec::new();
        // up
        if self.row > 0 {
            result.push(Coord {
                row: self.row - 1,
                col: self.col,
            });
        }
        // right
        result.push(Coord {
            row: self.row,
            col: self.col + 1,
        });
        // down
        result.push(Coord {
            row: self.row + 1,
            col: self.col,
        });
        //left
        if self.col > 0 {
            result.push(Coord {
                row: self.row,
                col: self.col - 1,
            });
        }
        result
    }
}

fn knot_hash(input: &[u8]) -> Vec<u8> {
    let mut skip_size = 0;
    let mut curr_pos = 0;
    let mut list: Vec<u8> = (0..=255).collect();
    let len_seq: Vec<u8> = input
        .iter()
        .copied()
        .chain([17, 31, 73, 47, 23])
        .collect();
    for _ in 0..64 {
        for length in len_seq.iter() {
            let length = *length as usize;
            for j in 0..length / 2 {
                let pos1 = (curr_pos + j) % list.len();
                let pos2 = (curr_pos + length - 1 - j) % list.len();
                list.swap(pos1, pos2);
            }
            curr_pos = (curr_pos + length + skip_size) % list.len();
            skip_size += 1;
        }
    }
    list.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, x| acc ^ x))
        .collect()
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input.trim()))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result: u32 = (0..128)
            .map(|row| {
                let key = format!("{}-{}", self.0, row);
                let hash = knot_hash(key.as_bytes());
                hash.iter()
                    .map(|byte| byte.count_ones())
                    .sum::<u32>()
            })
            .sum();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut used = HashSet::new();
        for row in 0..128 {
            let key = format!("{}-{}", self.0, row);
            let hash = knot_hash(key.as_bytes());
            for (byte_idx, byte) in hash.iter().rev().enumerate() {
                for bit_idx in (0..8).filter(|idx| (byte >> idx).trailing_ones() >= 1) {
                    let coord = Coord {
                        row,
                        col: (byte_idx * 8) + bit_idx,
                    };
                    used.insert(coord);
                }
            }
        }
        let mut count = 0;
        let mut stack = Vec::new();
        while !used.is_empty() {
            count += 1;
            let next = *used.iter().next().unwrap();
            used.remove(&next);
            stack.push(next);

            while let Some(coord) = stack.pop() {
                for neighbour in coord.neighbours() {
                    if let Some(&coord) = used.get(&neighbour) {
                        used.remove(&coord);
                        stack.push(coord);
                    }
                }
            }
        }

        Ok(count)
    }
}

// pub fn part_2(input: &str) -> u32 {
//     let mut used = HashSet::new();
//     for row in 0..128 {
//         let key = format!("{}-{}", input.trim(), row);
//         let hash = knot_hash(key.as_bytes());
//         for (n, byte) in hash.iter().rev().enumerate() {
//             for k in (0..8).filter(|idx| (byte >> idx) & 1 == 1) {
//                 let coord = Coord {
//                     row,
//                     // soooo, I looked up the bitmanip wizardry
//                     col: (n << 3) | k,
//                 };
//                 used.insert(coord);
//             }
//         }
//     }
//     let mut count = 0;
//     let mut stack = Vec::new();
//     while !used.is_empty() {
//         count += 1;
//         let next = *used.iter().next().unwrap();
//         used.remove(&next);
//         stack.push(next);

//         while let Some(coord) = stack.pop() {
//             for neighbour in coord.neighbours() {
//                 if let Some(&coord) = used.get(&neighbour) {
//                     used.remove(&coord);
//                     stack.push(coord);
//                 }
//             }
//         }
//     }
//     count
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "flqrgnkx";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "8108");
    }

    #[test]
    fn part_2() {
        let input = "flqrgnkx";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "1242");
    }
}
