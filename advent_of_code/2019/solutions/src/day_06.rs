use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(HashMap<&'a str, Vec<&'a str>>);

fn checksum(map: &HashMap<&str, Vec<&str>>, name: &str, depth: u32) -> u32 {
    let mut sum = depth;
    if let Some(orbiters) = map.get(name) {
        for orbiter in orbiters {
            sum += checksum(&map, &orbiter, depth + 1);
        }
    }
    sum
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        let mut map = HashMap::new();
        for line in input.lines() {
            let (center, orbiter) = line.split_once(")").unwrap();
            map.entry(orbiter).or_default();
            let orbiters: &mut Vec<_> = map.entry(center).or_default();
            orbiters.push(orbiter);
        }
        Ok(Self(map))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(checksum(&self.0, "COM", 0))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut map = self.0.clone();
        for (center, orbiters) in &self.0 {
            for orbiter in orbiters {
                map.entry(orbiter)
                    .or_default()
                    .push(center);
            }
        }

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        seen.insert("YOU");
        queue.push_back((0, "YOU"));

        while let Some((cost, name)) = queue.pop_front() {
            if name == "SAN" {
                // Between the objects they are orbiting - not between YOU and SAN.
                return Ok(cost - 2);
            }
            if let Some(neighbours) = map.get(name) {
                for neighbour in neighbours {
                    if seen.insert(neighbour) {
                        queue.push_back((cost + 1, neighbour));
                    }
                }
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
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "42");
    }

    #[test]
    fn part_2() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "4");
    }
}
