use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    connections: HashMap<String, Vec<String>>,
}

struct Path<'a> {
    current: &'a str,
    visited: HashSet<&'a str>,
    has_double: bool,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut connections: HashMap<String, Vec<String>> = HashMap::new();
        let line = input
            .trim()
            .lines()
            .map(|line| line.split_once("-"))
            .collect::<Option<Vec<_>>>()
            .ok_or(AoCError::Parsing)?;
        for (from, to) in line {
            connections
                .entry(from.to_string())
                .or_default()
                .push(to.to_string());
            connections
                .entry(to.to_string())
                .or_default()
                .push(from.to_string());
        }

        Ok(Self { connections })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut stack = vec![vec!["start"]];
        let mut paths = 0;

        while let Some(path) = stack.pop() {
            let current = path.last().unwrap();
            for neighbour in self.connections.get(*current).unwrap() {
                if neighbour == "end" {
                    paths += 1;
                    continue;
                }

                if neighbour
                    .chars()
                    .next()
                    .unwrap()
                    .is_lowercase()
                    && path.contains(&neighbour.as_str())
                {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(neighbour);
                stack.push(new_path);
            }
        }

        Ok(paths)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut stack = vec![Path {
            current: "start",
            visited: HashSet::new(),
            has_double: false,
        }];
        let mut paths = 0;

        while let Some(path) = stack.pop() {
            for neighbour in self
                .connections
                .get(path.current)
                .unwrap()
            {
                // can't go back to start, don't visit start but move to the next potential path
                if neighbour == "start" {
                    continue;
                }

                // if we visit end, we're done, increment the counter and move to the next potential path
                if neighbour == "end" {
                    paths += 1;
                    continue;
                }

                let mut has_double = path.has_double;

                if neighbour
                    .chars()
                    .next()
                    .unwrap()
                    .is_lowercase()
                    && path
                        .visited
                        .contains(neighbour.as_str())
                {
                    // we already visited this small neighbour
                    // move to the next potential path if we already visited a small cave twice
                    if has_double {
                        continue;
                    }
                    // visit this small cave a second time and record we visited a small cave twice
                    has_double = true;
                }

                // push the path with the visited neighbour onto the stack
                let mut new_path = Path {
                    current: neighbour,
                    visited: path.visited.clone(),
                    has_double,
                };
                new_path.visited.insert(neighbour);

                stack.push(new_path);
            }
        }

        Ok(paths)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "226");
    }

    #[test]
    fn part_2() {
        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "3509");
    }
}
