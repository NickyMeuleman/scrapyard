use crate::AoCData;
use std::collections::HashMap;

const EMPTY: i8 = -1;
const A: i8 = 0;
const B: i8 = 1;
const C: i8 = 2;
const D: i8 = 3;

#[derive(Debug, Clone)]
pub struct Data {
    rooms: [[i8; 2]; 4],
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct State<const DEPTH: usize> {
    hallway: [i8; 11],
    rooms: [[i8; DEPTH]; 4],
}

impl<const DEPTH: usize> State<DEPTH> {
    fn new(rooms: [[i8; DEPTH]; 4]) -> Self {
        Self {
            hallway: [EMPTY; 11],
            rooms,
        }
    }

    // fn show(&self) {
    //     // needs std HashMap, but HashBrown is significantly faster, only use std for debugging
    //     let charmap = HashMap::from([(-1, '.'), (0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);
    //     println!("#############");
    //     print!("#");
    //     for i in self.hallway {
    //         print!("{}", charmap[&i])
    //     }
    //     println!("#");

    //     print!("###");
    //     for i in self.rooms {
    //         print!("{}#", charmap[&i[0]])
    //     }
    //     println!("##");

    //     for i in 1..4 {
    //         print!("  #");
    //         for r in self.rooms {
    //             print!("{}#", charmap[&r[i]]);
    //         }
    //         println!();
    //     }

    //     println!("  #########");

    //     println!()
    // }

    fn room_ok(&self, room: usize) -> bool {
        self.rooms[room]
            .iter()
            .all(|r| *r == room as i8 || *r == EMPTY)
    }

    fn room_depth(&self, room: usize) -> usize {
        self.rooms[room]
            .iter()
            .take_while(|&&pod| pod == EMPTY)
            .count()
    }

    /// given a starting state, return a vector of tuples: (next possible state, cost to reach that state)
    fn possible_next_states(self) -> Vec<(State<DEPTH>, i32)> {
        let mut possible = vec![];

        // room to hallway
        for room in 0..4 {
            // if the room exclusively populated by the correct pods/empty spaces, there is no point in moving out of it
            if self.room_ok(room) {
                continue;
            }

            // get col idx from room idx
            let room_pos = 2 + room * 2;
            let depth = self.room_depth(room);
            // guaranteed amphipod that needs to move from a room to the hallway (because we checked if the room was ok first) at:
            let pod = self.rooms[room as usize][depth];

            // target col is left of current col
            for target in (0..room_pos).rev() {
                // spots right in front of rooms are not allowed
                if let 2 | 4 | 6 | 8 = target {
                    continue;
                }

                // check if target position is empty
                if self.hallway[target] != EMPTY {
                    break;
                }

                // pod moves to target pos and leaves original pos
                let mut hallway = self.hallway;
                hallway[target] = pod;
                let mut rooms = self.rooms;
                rooms[room][depth] = EMPTY;
                let distance = (room_pos - target) + depth + 1;
                possible.push((
                    State { hallway, rooms },
                    distance as i32 * 10i32.pow(pod as u32),
                ))
            }

            // target col is right of current col
            for target in room_pos + 1..11 {
                if let 2 | 4 | 6 | 8 = target {
                    continue;
                }

                if self.hallway[target] != EMPTY {
                    break;
                }

                let mut hallway = self.hallway;
                hallway[target] = pod;
                let mut rooms = self.rooms;
                rooms[room][depth] = EMPTY;
                let distance = (target - room_pos) + depth + 1;
                possible.push((
                    State { hallway, rooms },
                    distance as i32 * 10i32.pow(pod as u32),
                ))
            }
        }

        // hallway to room
        for hall in 0..11 {
            let pod = self.hallway[hall];
            // ensure pod is an amphipod
            if pod == EMPTY {
                continue;
            }
            // get col idx from room idx
            let room_pos = (2 + pod * 2) as usize;

            // check if all spots on the path to the target are empty
            if !self.hallway[(hall + 1).min(room_pos)..hall.max(room_pos)]
                .iter()
                .all(|&f| f == EMPTY)
            {
                continue;
            }

            // if the home room of this pod is not exclusively populated by the correct pods/empty spaces, we can't enter it
            if !self.room_ok(pod as usize) {
                continue;
            }

            // how deep should we go into the home room if it's accessible (not full)
            let depth = self.room_depth(pod as usize);
            if depth == 0 {
                continue;
            }

            // pod moves to target pos and leaves original pos
            let mut hallway = self.hallway;
            let mut rooms = self.rooms;
            hallway[hall] = EMPTY;
            rooms[pod as usize][depth - 1] = pod;
            let distance = (hall as isize - room_pos as isize).abs() + depth as isize;
            possible.push((
                State { hallway, rooms },
                distance as i32 * 10i32.pow(pod as u32),
            ))
        }

        possible
    }

    /// Return the minimum cost from a given state to the finished state if the end is reachable
    fn best_cost(self, mem: &mut HashMap<Self, Option<i64>>) -> Option<i64> {
        // check if finished
        if self.rooms == [[A; DEPTH], [B; DEPTH], [C; DEPTH], [D; DEPTH]] {
            return Some(0);
        }

        // does this state have a known minimum cost?
        if mem.contains_key(&self) {
            return mem[&self];
        }

        // get all next possible states, this vector can be empty if there are no valid next moves
        let next_states = self.possible_next_states();

        // get the minimum cost to the end
        let result = next_states
            .into_iter()
            .filter_map(|(next_state, move_to_next_cost)| {
                next_state
                    .best_cost(mem)
                    .map(|next_min_cost| move_to_next_cost as i64 + next_min_cost)
            })
            .min();

        // cache results
        // key: current state
        // value: Some() of the minimum total cost if the end is reachable, None if is not
        mem.insert(self, result);

        result
    }
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let rooms = input
            .trim()
            .lines()
            .skip(2)
            .take(2)
            .map(parse_line)
            .collect::<Option<Vec<Vec<i8>>>>()?;

        fn parse_line(input: &str) -> Option<Vec<i8>> {
            input
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| match c {
                    'A' => Some(A),
                    'B' => Some(B),
                    'C' => Some(C),
                    'D' => Some(D),
                    _ => None,
                })
                .collect()
        }

        Some(Self {
            rooms: [
                [*rooms.get(0)?.get(0)?, *rooms.get(1)?.get(0)?],
                [*rooms.get(0)?.get(1)?, *rooms.get(1)?.get(1)?],
                [*rooms.get(0)?.get(2)?, *rooms.get(1)?.get(2)?],
                [*rooms.get(0)?.get(3)?, *rooms.get(1)?.get(3)?],
            ],
        })
    }

    fn part_1(&self) -> String {
        State::new(self.rooms)
            .best_cost(&mut HashMap::new())
            .unwrap()
            .to_string()
    }

    fn part_2(&self) -> String {
        State::new([
            [self.rooms[0][0], D, D, self.rooms[0][1]],
            [self.rooms[1][0], C, B, self.rooms[1][1]],
            [self.rooms[2][0], B, A, self.rooms[2][1]],
            [self.rooms[3][0], A, C, self.rooms[3][1]],
        ])
        .best_cost(&mut HashMap::new())
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
        let input = utils::get_sample_input(23);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "12521");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(23);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "44169");
    }
}
