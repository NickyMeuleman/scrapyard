use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt,
};

use itertools::Itertools;

use crate::AoCData;

pub struct Data {
    floors: Vec<HashSet<Item>>,
}

#[derive(PartialEq, Eq, Clone)]
struct State {
    elevator: i8,
    floors: [Floor; 4],
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Floor {
    items: HashSet<Item>,
}

impl Floor {
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
            .all(|chip_name| self.items.contains(&Item::Generator(chip_name.to_string())));

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

// test input needs a , before the last and on the first line
// only for the test input because I can't see where it's going wrong
impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        result.push('\n');
        for floor in (0..=3).rev() {
            let mut line = String::new();
            line.push_str(&format!("F{}  ", floor + 1));
            let e = if self.elevator == floor as i8 {
                " E "
            } else {
                " . "
            };
            line.push_str(e);
            let floor = &self.floors[floor];
            let hg = if floor
                .items
                .contains(&Item::Generator("hydrogen".to_string()))
            {
                " HG "
            } else {
                " .  "
            };
            let hm = if floor
                .items
                .contains(&Item::Microchip("hydrogen".to_string()))
            {
                " HM "
            } else {
                " .  "
            };
            let lg = if floor
                .items
                .contains(&Item::Generator("lithium".to_string()))
            {
                " LG "
            } else {
                " .  "
            };
            let lm = if floor
                .items
                .contains(&Item::Microchip("lithium".to_string()))
            {
                " LM "
            } else {
                " .  "
            };
            line.push_str(hg);
            line.push_str(hm);
            line.push_str(lg);
            line.push_str(lm);
            line.push('\n');
            result.push_str(&line);
        }
        write!(f, "{}", result)
    }
}

impl State {
    /// creates state where elevator starts at the bottom
    fn try_new(floors: Vec<HashSet<Item>>) -> Option<Self> {
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
    fn next_states(&self) -> Vec<State> {
        let highest_uncleared_floor = self
            .floors
            .iter()
            .position(|floor| !floor.items.is_empty())
            .unwrap();
        let mut possibilities: Vec<State> = Vec::new();
        let floor = &self.floors[self.elevator as usize];

        let moves = floor
            .items
            .iter()
            // all combinations where you take 2 items
            .combinations(2)
            // all combinations where you take 1 item
            .chain(floor.items.iter().combinations(1));

        for items in moves {
            // move them up or down
            for direction in [-1, 1] {
                let next_elevator = self.elevator + direction;
                // make sure elevator stays within bounds
                if !(0..=3).contains(&next_elevator) {
                    continue;
                }
                // don't ever bring things down to a cleared level
                if (next_elevator as usize) < highest_uncleared_floor {
                    continue;
                }
                let mut next_floors = self.floors.clone();
                for &item in items.iter() {
                    // remove item from old floor
                    next_floors[self.elevator as usize].items.remove(item);
                    // add item to new floor
                    next_floors[next_elevator as usize]
                        .items
                        .insert(item.clone());
                }
                // only add new state if both the old floor and the new floor are valid
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

    fn counts(&self) -> [(u8, u8); 4] {
        self.floors
            .iter()
            .map(|floor| floor.counts())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    /// minimum cost to move every item to the top
    fn min_cost_to_top(self) -> u32 {
        let mut min_cost = u32::MAX;
        // Each VALID floor can be represented by (num_gens, num_chips) as each pair is interchangeable
        let mut seen = HashSet::new();
        let mut pq = BinaryHeap::new();

        // add starting node to pq
        pq.push(Node {
            cost: 0,
            state: self,
        });

        'outer: while let Some(Node { cost, state }) = pq.pop() {
            dbg!(&cost);
            dbg!(&state);
            dbg!(&pq.len());
            dbg!("");

            if state.full_last_level() {
                // popped an endstate, return
                return cost;
            }

            seen.insert(state.counts());

            if cost >= min_cost {
                // a less costly solution has already been found, skip
                // this happens when there's a solution with a lower cost in the queue
                continue 'outer;
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
                if !seen.contains(&new_state.counts()) {
                    pq.push(Node {
                        cost: new_cost,
                        state: new_state,
                    });
                }
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
        State::try_new(self.floors.clone())
            .unwrap()
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
