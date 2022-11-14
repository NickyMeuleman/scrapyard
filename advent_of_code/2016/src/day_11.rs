use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use crate::AoCData;

pub struct Data {
    floors: Vec<HashSet<Item>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    elevator: u8,
    floors: Vec<HashSet<Item>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    cost: u32,
    state: State,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    /// creates state where elevator starts at the bottom
    fn new(floors: Vec<HashSet<Item>>) -> Self {
        Self {
            elevator: 0,
            floors,
        }
    }
    /// checks if top floor is full by checking the first 3 floors are empty
    fn full_last_level(&self) -> bool {
        self.floors.iter().take(3).all(|set| set.is_empty())
    }

    /// check if floor at idx is valid
    fn is_valid_floor(&self, idx: usize) -> bool {
        let floor = &self.floors[idx];
        let all_chips_safe = floor
            .iter()
            .filter_map(|item| match item {
                Item::Microchip(name) => Some(name),
                Item::Generator(_) => None,
            })
            .all(|chip_name| floor.contains(&Item::Generator(chip_name.to_string())));

        floor.is_empty() || floor.len() == 1 || all_chips_safe
    }

    /// returns all legal next states from a given state
    fn next_states(&self) -> Vec<State> {
        unimplemented!()
    }

    /// minimum cost to move every item to the top
    fn min_cost_to_top(self) -> u32 {
        // init min_cost to end and priority queue
        let mut min_cost = u32::MAX;
        let mut pq = BinaryHeap::new();
        // add starting node to pq
        pq.push(Node {
            cost: 0,
            state: self,
        });

        'outer: while let Some(Node { cost, state }) = pq.pop() {
            if state.full_last_level() {
                // popped an endstate, return
                return cost;
            }
            if cost > min_cost {
                // a less costly solution has already been found, skip
                continue;
            }

            for new_state in state.next_states() {
                let new_cost = cost + 1;
                if new_state.full_last_level() {
                    // do not return here, the route to the end might not be the cheapest one
                    min_cost = min_cost.min(new_cost);
                    pq.push(Node {
                        cost: new_cost,
                        state: new_state,
                    });
                    continue 'outer;
                }

                // if we reached this, the items are not at the top yet, keep going with this branch
                pq.push(Node {
                    cost: new_cost,
                    state: new_state,
                });
            }
        }

        // only get here if there is no way found to completely move all items to the top (so: min_cost is still MAX)
        min_cost
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Item {
    Generator(String),
    Microchip(String),
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
                                items.insert(Item::Generator(name.to_string()));
                            }
                            part if part.ends_with("microchip") => {
                                let name = part.strip_suffix("-compatible microchip")?;
                                items.insert(Item::Microchip(name.to_string()));
                            }
                            _ => {}
                        };
                    }
                    part if part.starts_with('a') => {
                        let part = part.strip_prefix("a ")?;
                        match part {
                            part if part.ends_with("generator") => {
                                let name = part.strip_suffix(" generator")?;
                                items.insert(Item::Generator(name.to_string()));
                            }
                            part if part.ends_with("microchip") => {
                                let name = part.strip_suffix("-compatible microchip")?;
                                items.insert(Item::Microchip(name.to_string()));
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
        State::new(self.floors.clone())
            .min_cost_to_top()
            .to_string()
    }

    fn part_2(&self) -> String {
        String::new()
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
        assert_eq!(data.part_1(), "");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(11);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "");
    }
}
