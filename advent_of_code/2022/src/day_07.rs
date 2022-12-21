use std::{collections::HashMap, path::PathBuf};

use crate::AoCData;

pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> String {
        let mut sizes = HashMap::new();
        let mut affected = Vec::new();

        for line in self.0.lines() {
            if line.starts_with("$ ls") || line.starts_with("dir") {
                continue;
            }

            let parts: Vec<_> = line.split_whitespace().collect();
            match parts[..] {
                ["$", "cd", ".."] => {
                    affected.pop();
                }
                ["$", "cd", name] => {
                    affected.push(name);
                }
                [size, _name] => {
                    let size: u32 = size.parse().unwrap();
                    for idx in 0..affected.len() {
                        let path = PathBuf::from_iter(&affected[..=idx]);
                        *sizes.entry(path).or_insert(0) += size;
                    }
                }
                _ => {}
            };
        }

        sizes
            .into_values()
            .filter(|size| *size <= 100_000)
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        let mut sizes = HashMap::new();
        let mut affected = Vec::new();

        for line in self.0.lines() {
            if line.starts_with("$ ls") || line.starts_with("dir") {
                continue;
            }

            let parts: Vec<_> = line.split_whitespace().collect();
            match parts[..] {
                ["$", "cd", ".."] => {
                    affected.pop();
                }
                ["$", "cd", name] => {
                    affected.push(name);
                }
                [size, _name] => {
                    let size: u32 = size.parse().unwrap();
                    for idx in 0..affected.len() {
                        let path = PathBuf::from_iter(&affected[..=idx]);
                        *sizes.entry(path).or_insert(0) += size;
                    }
                }
                _ => {}
            };
        }

        let disk = 70_000_000;
        let needed = 30_000_000;
        let root = sizes.get(&PathBuf::from("/")).unwrap();
        let available = disk - root;

        sizes
            .into_values()
            .filter(|size| available + size >= needed)
            .min()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(7);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "95437");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(7);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_2(), "24933642");
    }
}
