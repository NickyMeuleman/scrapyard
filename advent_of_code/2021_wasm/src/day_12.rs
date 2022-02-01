use crate::AoCData;
use hashbrown::{HashMap, HashSet};

pub struct Data {
    connections: HashMap<String, Vec<String>>,
}

struct Path<'a> {
    current: &'a str,
    visited: HashSet<&'a str>,
    has_double: bool,
}

impl AoCData for Data {
    fn new(input: String) -> Self {
        // TODO: figure out how to pass a &str so the connections map can store &str instead of String
        let mut connections: HashMap<String, Vec<String>> = HashMap::new();
        for (from, to) in input.lines().map(|line| line.split_once("-").unwrap()) {
            connections
                .entry(from.to_string())
                .or_default()
                .push(to.to_string());
            connections
                .entry(to.to_string())
                .or_default()
                .push(from.to_string());
        }

        Self { connections }
    }

    fn part_1(&self) -> String {
        let mut stack = vec![vec!["start"]];
        let mut paths = 0;

        while let Some(path) = stack.pop() {
            let current = path.last().unwrap();
            for neighbour in self.connections.get(*current).unwrap() {
                if neighbour == "end" {
                    paths += 1;
                    continue;
                }

                if neighbour.chars().next().unwrap().is_lowercase() && path.contains(&neighbour.as_str()) {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(neighbour);
                stack.push(new_path);
            }
        }

        paths.to_string()
    }

    fn part_2(&self) -> String {
        let mut stack = vec![Path {
            current: "start",
            visited: HashSet::new(),
            has_double: false,
        }];
        let mut paths = 0;

        while let Some(path) = stack.pop() {
            for neighbour in self.connections.get(path.current).unwrap() {
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

                if neighbour.chars().next().unwrap().is_lowercase()
                    && path.visited.contains(neighbour.as_str())
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

        paths.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(12);
        let data = Data::new(input);
        assert_eq!(data.part_1(), "226");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(12);
        let data = Data::new(input);
        assert_eq!(data.part_2(), "3509");
    }
}
