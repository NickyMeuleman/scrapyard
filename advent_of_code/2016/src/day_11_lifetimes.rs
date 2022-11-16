use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use itertools::Itertools;

use crate::AoCData;

pub struct Data {
    floors: Vec<HashSet<OwnedItem>>,
}

#[derive(PartialEq, Eq, Clone)]
struct State<'a> {
    elevator: i8,
    floors: [Floor<'a>; 4],
}

#[derive(PartialEq, Eq, Clone)]
struct Floor<'a> {
    items: HashSet<Item<'a>>,
}

#[derive(PartialEq, Eq, Clone)]
struct Node<'a> {
    cost: u32,
    state: State<'a>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Item<'a> {
    Generator(&'a str),
    Microchip(&'a str),
}
#[derive(Clone, PartialEq, Eq, Hash)]
enum OwnedItem {
    Generator(String),
    Microchip(String),
}

impl Floor<'_> {
    fn is_valid(&self) -> bool {
        let (gens, chips): (Vec<_>, Vec<_>) = self.items.iter().partition(|item| match item {
            Item::Generator(_) => true,
            Item::Microchip(_) => false,
        });
        let has_single_item_type = gens.is_empty() || chips.is_empty();
        let all_chips_paired = self
            .items
            .iter()
            .filter_map(|item| match item {
                Item::Microchip(name) => Some(name),
                Item::Generator(_) => None,
            })
            .all(|chip_name| self.items.contains(&Item::Generator(*chip_name)));

        self.items.is_empty() || has_single_item_type || all_chips_paired
    }

    fn counts(&self) -> (u8, u8) {
        let (gens, chips): (Vec<_>, Vec<_>) = self.items.iter().partition(|item| match item {
            Item::Generator(_) => true,
            Item::Microchip(_) => false,
        });
        (gens.len() as u8, chips.len() as u8)
    }
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> State<'a> {
    /// creates state where elevator starts at the bottom
    fn try_new(floors: Vec<HashSet<Item<'a>>>) -> Option<Self> {
        Some(Self {
            elevator: 0,
            floors: floors
                .into_iter()
                .map(|items| Floor { items })
                .collect::<Vec<Floor>>()
                .try_into()
                .ok()?,
        })
    }
    /// checks if top floor is full by checking the first 3 floors are empty
    fn full_last_level(&self) -> bool {
        self.floors
            .iter()
            .take(self.floors.len() - 1)
            .all(|floor| floor.items.is_empty())
    }

    /// returns all legal next states from a given state
    fn next_states(&self) -> Vec<State<'a>> {
        let highest_uncleared_floor = self
            .floors
            .iter()
            .position(|floor| !floor.items.is_empty())
            .unwrap();
        let mut possibilities = Vec::new();
        let floor = &self.floors[self.elevator as usize];

        let moves = floor
            .items
            .iter()
            .combinations(2)
            .chain(floor.items.iter().combinations(1));

        for items in moves {
            for direction in [-1, 1] {
                let next_elevator = self.elevator + direction;
                if !(0..=3).contains(&next_elevator) {
                    continue;
                }
                if (next_elevator as usize) < highest_uncleared_floor {
                    continue;
                }
                let mut next_floors = self.floors.clone();
                for &item in items.iter() {
                    next_floors[self.elevator as usize].items.remove(item);
                    next_floors[next_elevator as usize]
                        .items
                        .insert(item.clone());
                }
                if next_floors[self.elevator as usize].is_valid()
                    && next_floors[next_elevator as usize].is_valid()
                {
                    possibilities.push(State {
                        elevator: next_elevator,
                        floors: next_floors,
                    });
                }
            }
        }

        possibilities
    }

    fn counts(&'a self) -> (u8, [(u8, u8); 4]) {
        let floors = self
            .floors
            .iter()
            .map(|floor| floor.counts())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        (self.elevator as u8, floors)
    }

    /// minimum cost to move every item to the top
    fn min_cost_to_top(self) -> u32 {
        let mut seen = HashSet::new();
        let mut pq = BinaryHeap::new();

        seen.insert(self.counts());
        pq.push(Node {
            cost: 0,
            state: self,
        });

        while let Some(node) = pq.pop() {
            if node.state.full_last_level() {
                return node.cost;
            }

            for new_state in node.state.next_states() {
                if seen.insert(new_state.counts()) {
                    pq.push(Node {
                        cost: node.cost + 1,
                        state: new_state,
                    });
                }
            }
        }

        u32::MAX
    }
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut floors = Vec::new();
        for line in input.lines() {
            let mut items = HashSet::new();
            let (_, rest) = line.strip_suffix('.')?.split_once("contains ")?;
            for part in rest.split(", ") {
                match part {
                    part if part.starts_with("and") => {
                        let part = part.strip_prefix("and a ")?;
                        match part {
                            part if part.ends_with("generator") => {
                                let name = part.strip_suffix(" generator")?;
                                items.insert(OwnedItem::Generator(name.to_string()));
                            }
                            part if part.ends_with("microchip") => {
                                let name = part.strip_suffix("-compatible microchip")?;
                                items.insert(OwnedItem::Microchip(name.to_string()));
                            }
                            _ => {}
                        };
                    }
                    part if part.starts_with('a') => {
                        let part = part.strip_prefix("a ")?;
                        match part {
                            part if part.ends_with("generator") => {
                                let name = part.strip_suffix(" generator")?;
                                items.insert(OwnedItem::Generator(name.to_string()));
                            }
                            part if part.ends_with("microchip") => {
                                let name = part.strip_suffix("-compatible microchip")?;
                                items.insert(OwnedItem::Microchip(name.to_string()));
                            }
                            _ => {}
                        };
                    }
                    "nothing relevant" => {}
                    _ => {}
                }
            }

            floors.push(items);
        }
        Some(Self { floors })
    }

    fn part_1(&self) -> String {
        let state = State::try_new(
            self.floors
                .iter()
                .map(|set| {
                    set.iter()
                        .map(|item| match item {
                            OwnedItem::Generator(name) => Item::Generator(name),
                            OwnedItem::Microchip(name) => Item::Microchip(name),
                        })
                        .collect()
                })
                .collect(),
        )
        .unwrap();
        state.min_cost_to_top().to_string()
    }

    fn part_2(&self) -> String {
        let mut state = State::try_new(
            self.floors
                .iter()
                .map(|set| {
                    set.iter()
                        .map(|item| match item {
                            OwnedItem::Generator(name) => Item::Generator(name),
                            OwnedItem::Microchip(name) => Item::Microchip(name),
                        })
                        .collect()
                })
                .collect(),
        )
        .unwrap();
        state.floors[0].items.insert(Item::Generator("elerium"));
        state.floors[0].items.insert(Item::Microchip("elerium"));
        state.floors[0].items.insert(Item::Generator("dilithium"));
        state.floors[0].items.insert(Item::Microchip("dilithium"));
        state.min_cost_to_top().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(11);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "11");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(11);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "61");
    }
}
// use std::{
//     cmp::Ordering,
//     collections::{BinaryHeap, HashSet},
// };

// use itertools::Itertools;

// use crate::AoCData;

// pub struct Data {
//     floors: Vec<HashSet<OwnedItem>>,
// }

// #[derive(PartialEq, Eq, Clone)]
// struct State<'a> {
//     elevator: i8,
//     floors: [Floor<'a>; 4],
// }

// #[derive(PartialEq, Eq, Clone)]
// struct Floor<'a> {
//     items: HashSet<Item<'a>>,
// }

// #[derive(PartialEq, Eq, Clone)]
// struct Node<'a> {
//     cost: u32,
//     state: State<'a>,
// }

// #[derive(Clone, PartialEq, Eq, Hash)]
// enum Item<'a> {
//     Generator(&'a str),
//     Microchip(&'a str),
// }
// #[derive(Clone, PartialEq, Eq, Hash)]
// enum OwnedItem {
//     Generator(String),
//     Microchip(String),
// }

// impl Floor<'_> {
//     fn is_valid(&self) -> bool {
//         let (gens, chips): (Vec<_>, Vec<_>) = self.items.iter().partition(|item| match item {
//             Item::Generator(_) => true,
//             Item::Microchip(_) => false,
//         });
//         let has_single_item_type = gens.is_empty() || chips.is_empty();
//         let all_chips_paired = self
//             .items
//             .iter()
//             .filter_map(|item| match item {
//                 Item::Microchip(name) => Some(name),
//                 Item::Generator(_) => None,
//             })
//             .all(|chip_name| self.items.contains(&Item::Generator(*chip_name)));

//         self.items.is_empty() || has_single_item_type || all_chips_paired
//     }

//     fn counts(&self) -> (u8, u8) {
//         let (gens, chips): (Vec<_>, Vec<_>) = self.items.iter().partition(|item| match item {
//             Item::Generator(_) => true,
//             Item::Microchip(_) => false,
//         });
//         (gens.len() as u8, chips.len() as u8)
//     }
// }

// impl<'a> Ord for Node<'a> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.cost.cmp(&self.cost)
//     }
// }

// impl<'a> PartialOrd for Node<'a> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl<'a> State<'a> {
//     /// creates state where elevator starts at the bottom
//     fn try_new(floors: Vec<HashSet<Item<'a>>>) -> Option<Self> {
//         Some(Self {
//             elevator: 0,
//             floors: floors
//                 .into_iter()
//                 .map(|items| Floor { items })
//                 .collect::<Vec<Floor>>()
//                 .try_into()
//                 .ok()?,
//         })
//     }
//     /// checks if top floor is full by checking the first 3 floors are empty
//     fn full_last_level(&self) -> bool {
//         self.floors
//             .iter()
//             .take(self.floors.len() - 1)
//             .all(|floor| floor.items.is_empty())
//     }

//     /// returns all legal next states from a given state
//     fn next_states(&self) -> Vec<State<'a>> {
//         let highest_uncleared_floor = self
//             .floors
//             .iter()
//             .position(|floor| !floor.items.is_empty())
//             .unwrap();
//         let mut possibilities = Vec::new();
//         let floor = &self.floors[self.elevator as usize];

//         let moves = floor
//             .items
//             .iter()
//             .combinations(2)
//             .chain(floor.items.iter().combinations(1));

//         for items in moves {
//             for direction in [-1, 1] {
//                 let next_elevator = self.elevator + direction;
//                 if !(0..=3).contains(&next_elevator) {
//                     continue;
//                 }
//                 if (next_elevator as usize) < highest_uncleared_floor {
//                     continue;
//                 }
//                 let mut next_floors = self.floors.clone();
//                 for &item in items.iter() {
//                     next_floors[self.elevator as usize].items.remove(item);
//                     next_floors[next_elevator as usize]
//                         .items
//                         .insert(item.clone());
//                 }
//                 if next_floors[self.elevator as usize].is_valid()
//                     && next_floors[next_elevator as usize].is_valid()
//                 {
//                     possibilities.push(State {
//                         elevator: next_elevator,
//                         floors: next_floors,
//                     });
//                 }
//             }
//         }

//         possibilities
//     }

//     fn counts(&'a self) -> (u8, [(u8, u8); 4]) {
//         let floors = self
//             .floors
//             .iter()
//             .map(|floor| floor.counts())
//             .collect::<Vec<_>>()
//             .try_into()
//             .unwrap();

//         (self.elevator as u8, floors)
//     }

//     /// minimum cost to move every item to the top
//     fn min_cost_to_top(self) -> u32 {
//         let mut seen = HashSet::new();
//         let mut pq = BinaryHeap::new();

//         seen.insert(self.counts());
//         pq.push(Node {
//             cost: 0,
//             state: self,
//         });

//         while let Some(node) = pq.pop() {
//             if node.state.full_last_level() {
//                 return node.cost;
//             }

//             for new_state in node.state.next_states() {
//                 if seen.insert(new_state.counts()) {
//                     pq.push(Node {
//                         cost: node.cost + 1,
//                         state: new_state,
//                     });
//                 }
//             }
//         }

//         u32::MAX
//     }
// }

// impl AoCData for Data {
//     fn try_new(input: String) -> Option<Self> {
//         let mut floors = Vec::new();
//         for line in input.lines() {
//             let mut items = HashSet::new();
//             let (_, rest) = line.strip_suffix('.')?.split_once("contains ")?;
//             for part in rest.split(", ") {
//                 match part {
//                     part if part.starts_with("and") => {
//                         let part = part.strip_prefix("and a ")?;
//                         match part {
//                             part if part.ends_with("generator") => {
//                                 let name = part.strip_suffix(" generator")?;
//                                 items.insert(OwnedItem::Generator(name.to_string()));
//                             }
//                             part if part.ends_with("microchip") => {
//                                 let name = part.strip_suffix("-compatible microchip")?;
//                                 items.insert(OwnedItem::Microchip(name.to_string()));
//                             }
//                             _ => {}
//                         };
//                     }
//                     part if part.starts_with('a') => {
//                         let part = part.strip_prefix("a ")?;
//                         match part {
//                             part if part.ends_with("generator") => {
//                                 let name = part.strip_suffix(" generator")?;
//                                 items.insert(OwnedItem::Generator(name.to_string()));
//                             }
//                             part if part.ends_with("microchip") => {
//                                 let name = part.strip_suffix("-compatible microchip")?;
//                                 items.insert(OwnedItem::Microchip(name.to_string()));
//                             }
//                             _ => {}
//                         };
//                     }
//                     "nothing relevant" => {}
//                     _ => {}
//                 }
//             }

//             floors.push(items);
//         }
//         Some(Self { floors })
//     }

//     fn part_1(&self) -> String {
//         let state = State::try_new(
//             self.floors
//                 .iter()
//                 .map(|set| {
//                     set.iter()
//                         .map(|item| match item {
//                             OwnedItem::Generator(name) => Item::Generator(name),
//                             OwnedItem::Microchip(name) => Item::Microchip(name),
//                         })
//                         .collect()
//                 })
//                 .collect(),
//         )
//         .unwrap();
//         state.min_cost_to_top().to_string()
//     }

//     fn part_2(&self) -> String {
//         let mut state = State::try_new(
//             self.floors
//                 .iter()
//                 .map(|set| {
//                     set.iter()
//                         .map(|item| match item {
//                             OwnedItem::Generator(name) => Item::Generator(name),
//                             OwnedItem::Microchip(name) => Item::Microchip(name),
//                         })
//                         .collect()
//                 })
//                 .collect(),
//         )
//         .unwrap();
//         state.floors[0].items.insert(Item::Generator("elerium"));
//         state.floors[0].items.insert(Item::Microchip("elerium"));
//         state.floors[0].items.insert(Item::Generator("dilithium"));
//         state.floors[0].items.insert(Item::Microchip("dilithium"));
//         state.min_cost_to_top().to_string()
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::utils;

//     #[test]
//     fn part_1() {
//         let input = utils::get_sample_input(11);
//         let data = Data::try_new(input).unwrap();
//         assert_eq!(data.part_1(), "11");
//     }

//     #[test]
//     fn part_2() {
//         let input = utils::get_input(11);
//         let data = Data::try_new(input).unwrap();
//         assert_eq!(data.part_2(), "61");
//     }
// }