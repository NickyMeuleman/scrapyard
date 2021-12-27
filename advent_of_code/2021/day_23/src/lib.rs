use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};
use std::{convert::Infallible, str::FromStr};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Data {
    rooms: [[i8; 2]; 4],
}

impl Data {
    pub fn part_one(&self) -> i64 {
        //    test
        // dbg!(run([[B, A], [C, D], [B, C], [D, A]]));
        run(self.rooms)
    }

    pub fn part_two(&self) -> i64 {
        run2(self.rooms)
    }
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rooms: Vec<Vec<i8>> = input
            .trim()
            .lines()
            .skip(2)
            .take(2)
            .map(|line| parse_line(line))
            .collect();

        fn parse_line(input: &str) -> Vec<i8> {
            input
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| match c {
                    'A' => 0,
                    'B' => 1,
                    'C' => 2,
                    'D' => 3,
                    _ => unreachable!("invalid input"),
                })
                .collect()
        }

        Ok(Self {
            rooms: [
                [rooms[0][0], rooms[1][0]],
                [rooms[0][1], rooms[1][1]],
                [rooms[0][2], rooms[1][2]],
                [rooms[0][3], rooms[1][3]],
            ],
        })
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct State1 {
    hallway: [i8; 11],
    rooms: [[i8; 2]; 4],
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct State2 {
    hallway: [i8; 11],
    rooms: [[i8; 4]; 4],
}

const A: i8 = 0;
const B: i8 = 1;
const C: i8 = 2;
const D: i8 = 3;

impl State1 {
    fn new(rooms: [[i8; 2]; 4]) -> Self {
        Self {
            hallway: [-1; 11],
            rooms,
        }
    }

    fn show(&self) {
        let charmap = HashMap::from([(-1, '.'), (0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);
        println!("#############");
        print!("#");
        for i in self.hallway {
            print!("{}", charmap[&i])
        }
        println!("#");

        print!("###");
        for i in self.rooms {
            print!("{}#", charmap[&i[0]])
        }
        println!("##");

        for i in 1..4 {
            print!("  #");
            for r in self.rooms {
                print!("{}#", charmap[&r[i]]);
            }
            println!();
        }

        println!("  #########");

        println!()
    }

    fn room_ok(&self, room: usize) -> bool {
        self.rooms[room]
            .iter()
            .all(|r| *r == room as i8 || *r == -1)
    }

    fn room_depth(&self, room: usize) -> usize {
        self.rooms[room]
            .iter()
            .take_while(|&&pod| pod == -1)
            .count()
    }

    fn possible_states(self) -> Vec<(State1, i32)> {
        let mut possible = vec![];
        // room to hallway
        for room in 0..4 {
            if self.room_ok(room) {
                continue;
            }

            let room_pos = 2 + room * 2;
            let depth = self.room_depth(room);
            let pod = self.rooms[room as usize][depth];
            for target in (0..room_pos).rev() {
                if let 2 | 4 | 6 | 8 = target {
                    continue;
                }

                if self.hallway[target] >= 0 {
                    break;
                }

                let mut hallway = self.hallway;
                hallway[target] = pod;
                let mut rooms = self.rooms;
                rooms[room][depth] = -1;
                let distance = (room_pos - target) + depth + 1;
                possible.push((
                    State1 { hallway, rooms },
                    distance as i32 * 10i32.pow(pod as u32),
                ))
            }

            for target in room_pos + 1..11 {
                if let 2 | 4 | 6 | 8 = target {
                    continue;
                }

                if self.hallway[target] >= 0 {
                    break;
                }

                let mut hallway = self.hallway;
                hallway[target] = pod;
                let mut rooms = self.rooms;
                rooms[room][depth] = -1;
                let distance = (target - room_pos) + depth + 1;
                possible.push((
                    State1 { hallway, rooms },
                    distance as i32 * 10i32.pow(pod as u32),
                ))
            }
        }

        // hallway to room
        for hall in 0..11 {
            let pod = self.hallway[hall];
            if pod < 0 {
                continue;
            }
            let room_pos = (2 + pod * 2) as usize;

            if !self.hallway[(hall + 1).min(room_pos)..hall.max(room_pos)]
                .iter()
                .all(|&f| f < 0)
            {
                continue;
            }

            if !self.room_ok(pod as usize) {
                continue;
            }
            let depth = self.room_depth(pod as usize);
            if depth == 0 {
                continue;
            }

            let mut hallway = self.hallway;
            let mut rooms = self.rooms;
            hallway[hall] = -1;
            rooms[pod as usize][depth - 1] = pod;
            let distance = (hall as isize - room_pos as isize).abs() + depth as isize;
            possible.push((
                State1 { hallway, rooms },
                distance as i32 * 10i32.pow(pod as u32),
            ))
        }
        possible
    }
}

fn min_cost_to(state: State1, mem: &mut HashMap<State1, Option<i64>>) -> Option<i64> {
    if state.rooms == [[A; 2], [B; 2], [C; 2], [D; 2]] {
        return Some(0);
    }

    if mem.contains_key(&state) {
        return mem[&state];
    }

    let states = state.possible_states();

    let result = states
        .into_iter()
        .filter_map(|(state, cost)| min_cost_to(state, mem).map(|f| f + cost as i64))
        .min();
    mem.insert(state, result);
    result
}

fn run(initial: [[i8; 2]; 4]) -> i64 {
    let rooms1: [[i8; 2]; 4] = [
        [initial[0][0], initial[0][1]],
        [initial[1][0], initial[1][1]],
        [initial[2][0], initial[2][1]],
        [initial[3][0], initial[3][1]],
    ];

    let initial = State1::new(rooms1);

    let mut result_map = HashMap::new();

    let mut next = initial;
    loop {
        let states = next.possible_states();
        if states.is_empty() {
            break;
        }
        let n = states
            .into_iter()
            .filter_map(|(state, cost)| Some((state, cost, (*result_map.get(&state)?)?)))
            .min_by_key(|(_state, _cost, tcost)| *tcost);
        if let Some(n) = n {
            next = n.0;
        } else {
            break;
        }
    }

    min_cost_to(initial, &mut result_map).unwrap()
}
impl State2 {
    fn new(rooms: [[i8; 4]; 4]) -> Self {
        Self {
            hallway: [-1; 11],
            rooms,
        }
    }

    fn show(&self) {
        let charmap = HashMap::from([(-1, '.'), (0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);
        println!("#############");
        print!("#");
        for i in self.hallway {
            print!("{}", charmap[&i])
        }
        println!("#");

        print!("###");
        for i in self.rooms {
            print!("{}#", charmap[&i[0]])
        }
        println!("##");

        for i in 1..4 {
            print!("  #");
            for r in self.rooms {
                print!("{}#", charmap[&r[i]]);
            }
            println!();
        }

        println!("  #########");

        println!()
    }

    fn room_ok(&self, room: usize) -> bool {
        self.rooms[room]
            .iter()
            .all(|r| *r == room as i8 || *r == -1)
    }

    fn room_depth(&self, room: usize) -> usize {
        self.rooms[room]
            .iter()
            .take_while(|&&pod| pod == -1)
            .count()
    }

    fn possible_states(self) -> Vec<(State2, i32)> {
        let mut possible = vec![];
        // room to hallway
        for room in 0..4 {
            if self.room_ok(room) {
                continue;
            }

            let room_pos = 2 + room * 2;
            let depth = self.room_depth(room);
            let pod = self.rooms[room as usize][depth];
            for target in (0..room_pos).rev() {
                if let 2 | 4 | 6 | 8 = target {
                    continue;
                }

                if self.hallway[target] >= 0 {
                    break;
                }

                let mut hallway = self.hallway;
                hallway[target] = pod;
                let mut rooms = self.rooms;
                rooms[room][depth] = -1;
                let distance = (room_pos - target) + depth + 1;
                possible.push((
                    State2 { hallway, rooms },
                    distance as i32 * 10i32.pow(pod as u32),
                ))
            }

            for target in room_pos + 1..11 {
                if let 2 | 4 | 6 | 8 = target {
                    continue;
                }

                if self.hallway[target] >= 0 {
                    break;
                }

                let mut hallway = self.hallway;
                hallway[target] = pod;
                let mut rooms = self.rooms;
                rooms[room][depth] = -1;
                let distance = (target - room_pos) + depth + 1;
                possible.push((
                    State2 { hallway, rooms },
                    distance as i32 * 10i32.pow(pod as u32),
                ))
            }
        }

        // hallway to room
        for hall in 0..11 {
            let pod = self.hallway[hall];
            if pod < 0 {
                continue;
            }
            let room_pos = (2 + pod * 2) as usize;

            if !self.hallway[(hall + 1).min(room_pos)..hall.max(room_pos)]
                .iter()
                .all(|&f| f < 0)
            {
                continue;
            }

            if !self.room_ok(pod as usize) {
                continue;
            }
            let depth = self.room_depth(pod as usize);
            if depth == 0 {
                continue;
            }

            let mut hallway = self.hallway;
            let mut rooms = self.rooms;
            hallway[hall] = -1;
            rooms[pod as usize][depth - 1] = pod;
            let distance = (hall as isize - room_pos as isize).abs() + depth as isize;
            possible.push((
                State2 { hallway, rooms },
                distance as i32 * 10i32.pow(pod as u32),
            ))
        }
        possible
    }
}

fn min_cost_to2(state: State2, mem: &mut HashMap<State2, Option<i64>>) -> Option<i64> {
    if state.rooms == [[A; 4], [B; 4], [C; 4], [D; 4]] {
        return Some(0);
    }

    if mem.contains_key(&state) {
        return mem[&state];
    }

    let states = state.possible_states();

    let result = states
        .into_iter()
        .filter_map(|(state, cost)| min_cost_to2(state, mem).map(|f| f + cost as i64))
        .min();
    mem.insert(state, result);
    result
}

fn run2(initial: [[i8; 2]; 4]) -> i64 {
    let rooms2: [[i8; 4]; 4] = [
        [initial[0][0], D, D, initial[0][1]],
        [initial[1][0], C, B, initial[1][1]],
        [initial[2][0], B, A, initial[2][1]],
        [initial[3][0], A, C, initial[3][1]],
    ];

    let initial = State2::new(rooms2);

    let mut result_map = HashMap::new();

    let mut next = initial;
    loop {
        let states = next.possible_states();
        if states.is_empty() {
            break;
        }
        let n = states
            .into_iter()
            .filter_map(|(state, cost)| Some((state, cost, (*result_map.get(&state)?)?)))
            .min_by_key(|(_state, _cost, tcost)| *tcost);
        if let Some(n) = n {
            next = n.0;
        } else {
            break;
        }
    }

    min_cost_to2(initial, &mut result_map).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "
#############
#...........#
###B#C#B#D###
  #A#D#C#A#  
  #########  
";

        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 12521);
    }

    #[test]
    fn part_two() {
        let input = "
#############
#...........#
###B#C#B#D###
  #A#D#C#A#  
  #########  
";

        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 44169);
    }
}
