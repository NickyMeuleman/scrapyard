use std::{
    collections::{BTreeSet, HashMap},
    fmt::Display,
};

use aoc_core::{AoCError, Solution};
use md5::{digest::core_api::CoreWrapper, Digest, Md5, Md5Core};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

fn first_triplet(hex: &[u8]) -> Option<u8> {
    hex.windows(3)
        .find(|window| window.iter().all(|n| window[0] == *n))
        .map(|window| window[0])
}

fn has_quintet_of(hex: &[u8], target: u8) -> bool {
    hex.windows(5)
        .any(|window| window.iter().all(|n| *n == target))
}

fn get_hex_hash(
    hasher: &mut CoreWrapper<Md5Core>,
    salt: &str,
    index: usize,
    stretch_len: usize,
) -> Vec<u8> {
    let mut hash = Vec::new();
    hash.extend(salt.as_bytes().iter());
    hash.append(&mut format!("{}", index).into_bytes());

    for _ in 0..=stretch_len {
        hasher.update(&hash);
        hash.clear();
        let digest = hasher.finalize_reset();
        for &j in digest.iter() {
            for &k in &[j >> 4 & 0xF, j & 0xF] {
                hash.push(if k < 10 { b'0' + k } else { b'a' + k - 10 });
            }
        }
    }

    hash
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut hasher = Md5::new();
        let mut index = 0;
        let mut key_indexes = BTreeSet::new();
        let mut triplets: HashMap<usize, u8> = HashMap::new();
        let salt = self.0;

        // reason for the || condition:
        // the inner loop might add a higher index to the set of valid keys in an earlier loop
        // so force the outer loop to keep going until the index is at least 1000 bigger than the highest valid key index
        // to eliminate that possibility
        while key_indexes.len() < 64 || index < key_indexes.last().unwrap_or(&0) + 1000 {
            let hex_hash = get_hex_hash(&mut hasher, salt, index, 0);
            if let Some(triplet_char) = first_triplet(&hex_hash) {
                if has_quintet_of(&hex_hash, triplet_char) {
                    for (k, v) in triplets.iter() {
                        // all hashes with matching triplet within the 1000 previous hashes are valid keys
                        if triplet_char == *v && *k < index && index <= 1000 + k {
                            key_indexes.insert(*k);
                        }
                    }
                }
                triplets.insert(index, triplet_char);
            }
            index += 1;
        }
        let result = key_indexes.into_iter().nth(63);

        result.ok_or(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut hasher = Md5::new();
        let mut index = 0;
        let mut key_indexes = BTreeSet::new();
        let mut triplets: HashMap<usize, u8> = HashMap::new();
        let salt = self.0;

        while key_indexes.len() < 64 || index < key_indexes.last().unwrap_or(&0) + 1000 {
            let hex_hash = get_hex_hash(&mut hasher, salt, index, 2016);
            if let Some(triplet_char) = first_triplet(&hex_hash) {
                if has_quintet_of(&hex_hash, triplet_char) {
                    for (k, v) in triplets.iter() {
                        if triplet_char == *v && *k < index && index <= 1000 + k {
                            key_indexes.insert(*k);
                        }
                    }
                }
                triplets.insert(index, triplet_char);
            }
            index += 1;
        }
        let result = key_indexes.into_iter().nth(63);
        result.ok_or(AoCError::Solving)
    }

    fn solve(self) -> AoCResult<Solution>
    where
        Self: Sized,
    {
        let mut hasher = Md5::new();
        let mut index = 0;
        let mut key_indexes = BTreeSet::new();
        let mut key_indexes_stretched = BTreeSet::new();
        let mut triplets: HashMap<usize, u8> = HashMap::new();
        let mut triplets_stretched: HashMap<usize, u8> = HashMap::new();
        let salt = self.0;

        while key_indexes.len() < 64
            || key_indexes_stretched.len() < 64
            || index < key_indexes.last().unwrap_or(&0) + 1000
            || index
                < key_indexes_stretched
                    .last()
                    .unwrap_or(&0)
                    + 1000
        {
            let hex_hash = get_hex_hash(&mut hasher, salt, index, 0);
            let hex_hash_stretched = get_hex_hash(&mut hasher, salt, index, 2016);
            if let Some(triplet_char) = first_triplet(&hex_hash) {
                if has_quintet_of(&hex_hash, triplet_char) {
                    for (k, v) in triplets.iter() {
                        if triplet_char == *v && *k < index && index <= 1000 + k {
                            key_indexes.insert(*k);
                        }
                    }
                }
                triplets.insert(index, triplet_char);
            }
            if let Some(triplet_char) = first_triplet(&hex_hash_stretched) {
                if has_quintet_of(&hex_hash_stretched, triplet_char) {
                    for (k, v) in triplets_stretched.iter() {
                        if triplet_char == *v && *k < index && index <= 1000 + k {
                            key_indexes_stretched.insert(*k);
                        }
                    }
                }
                triplets_stretched.insert(index, triplet_char);
            }
            index += 1;
        }
        let part1 = key_indexes
            .into_iter()
            .nth(63)
            .ok_or(AoCError::Solving)?;
        let part2 = key_indexes_stretched
            .into_iter()
            .nth(63)
            .ok_or(AoCError::Solving)?;

        Ok(Solution {
            part1: Box::new(part1),
            part2: Box::new(part2),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "abc";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "22728");
    }

    #[test]
    fn part_2() {
        let input = "abc";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "22551");
    }

    #[test]
    fn solve() {
        let input = "abc";
        let data = Data::try_new(input).unwrap();
        let Solution { part1, part2 } = data.solve().unwrap();
        assert_eq!(part1.to_string(), "22728");
        assert_eq!(part2.to_string(), "22551");
    }
}
