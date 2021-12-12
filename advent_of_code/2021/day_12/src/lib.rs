use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Data {
    // a map with a single "from" cave as key and all possible "to" caves as value
    connections: HashMap<Cave, HashSet<Cave>>,
}

impl Data {
    pub fn part_one(&self) -> usize {
        // create all paths that start at Start, end at End, and only visit Small caves once
        let mut do_not_visit: HashSet<Cave> = HashSet::new();
        do_not_visit.insert(Cave::Start);
        self.count_from(&Cave::Start, do_not_visit)
    }

    pub fn part_two(&self) -> usize {
        1
    }

    fn count_from(&self, from: &Cave, do_not_visit: HashSet<Cave>) -> usize {
        // goal: get to the end, if we do, total + 1
        self.connections
            .get(from)
            .unwrap()
            .iter()
            .map(|option| {
                let mut do_not_visit = do_not_visit.clone();
                match option {
                    Cave::Small(_) if !do_not_visit.contains(option) => {
                        // add option to do_not_visit
                        do_not_visit.insert(option.clone());
                        // not yet, recurse
                        self.count_from(option, do_not_visit)
                    }
                    Cave::Big(_) => {
                        // not yet, recurse
                        self.count_from(option, do_not_visit)
                    }
                    Cave::End => {
                        // found the end
                        1
                    }
                    _ => {
                        // invalid
                        0
                    }
                }
            })
            .sum()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Cave {
    // the string is a unique ID
    Start,
    End,
    Big(String),
    Small(String),
}

impl FromStr for Data {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut connections: HashMap<Cave, HashSet<Cave>> = HashMap::new();

        for (from, to) in input.trim().lines().map(|connection| {
            let (from, to) = connection.split_once("-").unwrap();
            let from = match from {
                "start" => Cave::Start,
                "end" => Cave::End,
                id if from.chars().all(|c| c.is_uppercase()) => Cave::Big(id.to_string()),
                id if from.chars().all(|c| c.is_lowercase()) => Cave::Small(id.to_string()),
                _ => {
                    panic!("Invalid input")
                }
            };
            let to = match to {
                "start" => Cave::Start,
                "end" => Cave::End,
                id if to.chars().all(|c| c.is_uppercase()) => Cave::Big(id.to_string()),
                id if to.chars().all(|c| c.is_lowercase()) => Cave::Small(id.to_string()),
                _ => {
                    panic!("Invalid input")
                }
            };
            (from, to)
        }) {
            // add the "to" cave to the "from" options
            let options = connections.entry(from.clone()).or_default();
            options.insert(to.clone());

            // add the "from" cave to the "to" options
            let options = connections.entry(to).or_default();
            options.insert(from);
        }

        Ok(Self { connections })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_tiny() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 10);
    }

    //     #[test]
    //     fn part_one_medium() {
    //         let input = "dc-end
    // HN-start
    // start-kj
    // dc-start
    // dc-HN
    // LN-dc
    // HN-end
    // kj-sa
    // kj-HN
    // kj-dc";
    //         let data: Data = input.parse().unwrap();
    //         assert_eq!(data.part_one(), 19);
    //     }
    //     #[test]
    //     fn part_one_large() {
    //         let input = "fs-end
    // he-DX
    // fs-he
    // start-DX
    // pj-DX
    // end-zg
    // zg-sl
    // zg-pj
    // pj-he
    // RW-he
    // fs-DX
    // pj-RW
    // zg-RW
    // start-pj
    // he-WI
    // zg-he
    // pj-fs
    // start-RW";
    //         let data: Data = input.parse().unwrap();
    //         assert_eq!(data.part_one(), 226);
    //     }

    //     #[test]

    //     fn part_two_example() {
    //         let input = "5483143223
    // 2745854711
    // 5264556173
    // 6141336146
    // 6357385478
    // 4167524645
    // 2176841721
    // 6882881134
    // 4846848554
    // 5283751526";
    //         let data: Data = input.parse().unwrap();
    //         assert_eq!(data.part_two(), 195);
    //     }
}
