use std::{collections::HashMap, fmt::Display, path::PathBuf};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
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

        let result: u32 = sizes
            .into_values()
            .filter(|size| *size <= 100_000)
            .sum();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
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
                    let size: u32 = size.parse()?;
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

        let result = sizes
            .into_values()
            .filter(|size| available + size >= needed)
            .min()
            .ok_or(AoCError::Solving)?;

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "95437");
    }

    #[test]
    fn part_2() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "24933642");
    }
}
