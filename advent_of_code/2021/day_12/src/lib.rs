use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
// the faster code is lightly edited code from https://github.com/Killavus/aoc2021/blob/master/day12/src/main.rs

#[derive(Debug, Clone)]
pub struct Data {
    // a map with a single "from" cave as key and all possible "to" caves as value
    connections: HashMap<Cave, HashSet<Cave>>,
}

impl Data {
    pub fn part_one(&self) -> usize {
        let mut used = HashSet::new();
        used.insert(Cave::Start);
        self.depth_first(&Cave::Start, &mut used)
    }

    pub fn part_two(&self) -> usize {
        let mut used = HashSet::new();
        used.insert(Cave::Start);
        self.count_paths_from(&Cave::Start, None, &mut used)
    }

    fn depth_first(&self, current: &Cave, used: &mut HashSet<Cave>) -> usize {
        let mut result = 0;

        if current == &Cave::End {
            return 1;
        }

        // mark small caves as used
        if let Cave::Small(_) = current {
            used.insert(current.clone());
        }

        for cave in self.connections.get(current).unwrap() {
            match cave {
                _ if !used.contains(cave) => {
                    // big, eligible (unused) small, or end
                    result += self.depth_first(cave, used);
                }
                _ => {
                    // invalid
                    // start or ineligible (used) small
                }
            }
        }

        // remove the current cave from the used set if it was there
        // to ensure a recursive call doesn't have a cave marked as used
        // while in that recursive iteration, it hasn't been used
        used.remove(current);

        result
    }

    fn count_paths_from(
        &self,
        current: &Cave,
        twice_cave: Option<&Cave>,
        used: &mut HashSet<Cave>,
    ) -> usize {
        let mut result = 0;

        if current == &Cave::End {
            return 1;
        }

        if current.is_small() {
            used.insert(current.clone());
        }

        for cave in self.connections.get(current).unwrap() {
            match cave {
                Cave::Small(_) if used.contains(cave) && twice_cave.is_none() => {
                    // small cave was already used once, use it again
                    result += self.count_paths_from(cave, Some(cave), used);
                }
                _ if !used.contains(cave) => {
                    result += self.count_paths_from(cave, twice_cave, used);
                }
                _ => {
                    // invalid
                    // either Start or invalid Small
                }
            }
        }

        if used.contains(current) {
            // In case cave is entered twice we do not want to remove it until we backtrack to 'unused' state of this particular cave.
            if let Some(twice_cave) = twice_cave {
                if twice_cave != current {
                    used.remove(current);
                }
            } else {
                used.remove(current);
            }
        }

        result
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Cave {
    Start,
    End,
    Big(u64),
    Small(u64),
}

impl Cave {
    fn is_small(&self) -> bool {
        match self {
            Cave::Small(_) => true,
            _ => false,
        }
    }
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
                id if from.chars().all(|c| c.is_uppercase()) => {
                    Cave::Big(calculate_hash(&id))},
                id if from.chars().all(|c| c.is_lowercase()) => Cave::Small(calculate_hash(&id)),
                _ => {
                    panic!("Invalid input")
                }
            };
            let to = match to {
                "start" => Cave::Start,
                "end" => Cave::End,
                id if to.chars().all(|c| c.is_uppercase()) => Cave::Big(calculate_hash(&id)),
                id if to.chars().all(|c| c.is_lowercase()) => Cave::Small(calculate_hash(&id)),
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

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
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

    #[test]
    fn part_one_medium() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 19);
    }
    #[test]
    fn part_one_large() {
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
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 226);
    }

    #[test]
    fn part_two_tiny() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 36);
    }

    #[test]
    fn part_two_medium() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 103);
    }
    #[test]
    fn part_two_large() {
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
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 3509);
    }
}
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::str::FromStr;

// // the faster code is lightly edited code from https://github.com/Killavus/aoc2021/blob/master/day12/src/main.rs

// #[derive(Debug, Clone)]
// pub struct Data {
//     // a map with a single "from" cave as key and all possible "to" caves as value
//     connections: HashMap<Cave, HashSet<Cave>>,
// }

// impl Data {
//     pub fn part_one(&self) -> usize {
//         let mut used = HashSet::new();
//         used.insert(Cave::Start);
//         self.depth_first(&Cave::Start, &mut used)
//     }
//     // pub fn part_one(&self) -> usize {
//     //     // create all paths that start at Start, end at End, and only visit Small caves once
//     //     let mut do_not_visit: HashSet<Cave> = HashSet::new();
//     //     do_not_visit.insert(Cave::Start);
//     //     self.count_from1(&Cave::Start, do_not_visit)
//     // }

//     pub fn part_two(&self) -> usize {
//         let mut used = HashSet::new();
//         used.insert(Cave::Start);
//         self.count_paths_from(&Cave::Start, None, &mut used)
//     }
//     // pub fn part_two(&self) -> usize {
//     //     // create all paths that start at Start, end at End, and only visit Small caves once
//     //     let mut do_not_visit: HashSet<Cave> = HashSet::new();
//     //     do_not_visit.insert(Cave::Start);
//     //     self.count_from2(&Cave::Start, do_not_visit, false)
//     // }

//     fn depth_first(&self, current: &Cave, used: &mut HashSet<Cave>) -> usize {
//         let mut result = 0;

//         if current == &Cave::End {
//             return 1;
//         }

//         // mark small caves as used
//         if let Cave::Small(_) = current {
//             used.insert(current.clone());
//         }

//         for cave in self.connections.get(current).unwrap() {
//             match cave {
//                 _ if !used.contains(cave) => {
//                     // big, eligible (unused) small, or end
//                     result += self.depth_first(cave, used);
//                 }
//                 _ => {
//                     // invalid
//                     // start or ineligible (used) small
//                 }
//             }
//         }

//         // remove the current cave from the used set if it was there
//         // to ensure a recursive call doesn't have a cave marked as used
//         // while in that recursive iteration, it hasn't been used
//         used.remove(current);

//         result
//     }
//     // also correct, but much slower
//     fn count_from1(&self, from: &Cave, do_not_visit: HashSet<Cave>) -> usize {
//         // goal: get to the end, if we do, total + 1
//         self.connections
//             .get(from)
//             .unwrap()
//             .iter()
//             .map(|option| {
//                 let mut do_not_visit = do_not_visit.clone();
//                 match option {
//                     Cave::Small(_) if !do_not_visit.contains(option) => {
//                         // add option to do_not_visit
//                         do_not_visit.insert(option.clone());
//                         // not yet, recurse
//                         self.count_from1(option, do_not_visit)
//                     }
//                     Cave::Big(_) => {
//                         // not yet, recurse
//                         self.count_from1(option, do_not_visit)
//                     }
//                     Cave::End => {
//                         // found the end
//                         1
//                     }
//                     _ => {
//                         // invalid
//                         0
//                     }
//                 }
//             })
//             .sum()
//     }

//     fn count_paths_from(
//         &self,
//         current: &Cave,
//         twice_cave: Option<&Cave>,
//         used: &mut HashSet<Cave>,
//     ) -> usize {
//         let mut result = 0;

//         if current == &Cave::End {
//             return 1;
//         }

//         if current.is_small() {
//             used.insert(current.clone());
//         }

//         for cave in self.connections.get(current).unwrap() {
//             match cave {
//                 Cave::Small(_) if used.contains(cave) && twice_cave.is_none() => {
//                     // small cave was already used once, use it again
//                     result += self.count_paths_from(cave, Some(cave), used);
//                 }
//                 _ if !used.contains(cave) => {
//                     result += self.count_paths_from(cave, twice_cave, used);
//                 }
//                 _ => {
//                     // invalid
//                     // either Start or invalid Small
//                 }
//             }
//         }

//         if used.contains(current) {
//             // In case cave is entered twice we do not want to remove it until we backtrack to 'unused' state of this particular cave.
//             if let Some(twice_cave) = twice_cave {
//                 if twice_cave != current {
//                     used.remove(current);
//                 }
//             } else {
//                 used.remove(current);
//             }
//         }

//         result
//     }

//     // also correct, but much slower
//     fn count_from2(
//         &self,
//         from: &Cave,
//         do_not_visit: HashSet<Cave>,
//         visited_small_twice: bool,
//     ) -> usize {
//         // goal: get to the end, if we do, total + 1
//         self.connections
//             .get(from)
//             .unwrap()
//             .iter()
//             .map(|option| {
//                 let mut do_not_visit = do_not_visit.clone();
//                 match option {
//                     Cave::Small(_) => {
//                         let visited_already = do_not_visit.contains(&option);
//                         match (visited_small_twice, visited_already) {
//                             (false, false) => {
//                                 // first visit to this cave
//                                 // add option to do_not_visit
//                                 do_not_visit.insert(option.clone());
//                                 // not yet, recurse
//                                 self.count_from2(option, do_not_visit, false)
//                             }
//                             (false, true) => {
//                                 // second visit to this cave
//                                 // add option to do_not_visit
//                                 do_not_visit.insert(option.clone());
//                                 // not yet, recurse
//                                 self.count_from2(option, do_not_visit, true)
//                             }
//                             (true, false) => {
//                                 // first visit to this cave
//                                 // add option to do_not_visit
//                                 do_not_visit.insert(option.clone());
//                                 // not yet, recurse
//                                 self.count_from2(option, do_not_visit, true)
//                             }
//                             (true, true) => {
//                                 // invalid
//                                 0
//                             }
//                         }
//                     }
//                     Cave::Big(_) => {
//                         // not yet, recurse
//                         self.count_from2(option, do_not_visit, visited_small_twice)
//                     }
//                     Cave::End => {
//                         // found the end
//                         1
//                     }
//                     _ => {
//                         // invalid
//                         0
//                     }
//                 }
//             })
//             .sum()
//     }
// }

// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
// enum Cave {
//     // the string is a unique ID
//     Start,
//     End,
//     Big(String),
//     Small(String),
// }

// impl Cave {
//     fn is_small(&self) -> bool {
//         match self {
//             Cave::Small(_) => true,
//             _ => false,
//         }
//     }
// }

// impl FromStr for Data {
//     type Err = ();

//     fn from_str(input: &str) -> Result<Self, Self::Err> {
//         let mut connections: HashMap<Cave, HashSet<Cave>> = HashMap::new();

//         for (from, to) in input.trim().lines().map(|connection| {
//             let (from, to) = connection.split_once("-").unwrap();
//             let from = match from {
//                 "start" => Cave::Start,
//                 "end" => Cave::End,
//                 id if from.chars().all(|c| c.is_uppercase()) => Cave::Big(id.to_string()),
//                 id if from.chars().all(|c| c.is_lowercase()) => Cave::Small(id.to_string()),
//                 _ => {
//                     panic!("Invalid input")
//                 }
//             };
//             let to = match to {
//                 "start" => Cave::Start,
//                 "end" => Cave::End,
//                 id if to.chars().all(|c| c.is_uppercase()) => Cave::Big(id.to_string()),
//                 id if to.chars().all(|c| c.is_lowercase()) => Cave::Small(id.to_string()),
//                 _ => {
//                     panic!("Invalid input")
//                 }
//             };
//             (from, to)
//         }) {
//             // add the "to" cave to the "from" options
//             let options = connections.entry(from.clone()).or_default();
//             options.insert(to.clone());

//             // add the "from" cave to the "to" options
//             let options = connections.entry(to).or_default();
//             options.insert(from);
//         }

//         Ok(Self { connections })
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn part_one_tiny() {
//         let input = "start-A
// start-b
// A-c
// A-b
// b-d
// A-end
// b-end";
//         let data: Data = input.parse().unwrap();
//         assert_eq!(data.part_one(), 10);
//     }

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
//     fn part_two_tiny() {
//         let input = "start-A
// start-b
// A-c
// A-b
// b-d
// A-end
// b-end";
//         let data: Data = input.parse().unwrap();
//         assert_eq!(data.part_two(), 36);
//     }

//     #[test]
//     fn part_two_medium() {
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
//         assert_eq!(data.part_two(), 103);
//     }
//     #[test]
//     fn part_two_large() {
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
//         assert_eq!(data.part_two(), 3509);
//     }
// }
