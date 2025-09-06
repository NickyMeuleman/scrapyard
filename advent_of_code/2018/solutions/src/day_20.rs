use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use aoc_core::Solution;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Door {
    N,
    E,
    S,
    W,
}
impl Door {
    const ALL: [Self; 4] = [Self::N, Self::E, Self::S, Self::W];

    fn opposite(&self) -> Self {
        match self {
            Door::N => Door::S,
            Door::E => Door::W,
            Door::S => Door::N,
            Door::W => Door::E,
        }
    }
    fn coord_change(&self) -> (i16, i16) {
        match self {
            Door::N => (0, -1),
            Door::E => (1, 0),
            Door::S => (0, 1),
            Door::W => (-1, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Data(Vec<char>);

fn build_graph(chars: &[char]) -> HashMap<(i16, i16), HashSet<Door>> {
    let mut graph: HashMap<(i16, i16), HashSet<Door>> = HashMap::new();
    let mut curr_positions = vec![(0, 0)];
    let mut branches_startpos = vec![vec![(0, 0)]];
    let mut branches_endpos = vec![];

    for &c in chars {
        match c {
            'N' | 'E' | 'S' | 'W' => {
                let dir = match c {
                    'N' => Door::N,
                    'E' => Door::E,
                    'S' => Door::S,
                    'W' => Door::W,
                    _ => unreachable!(),
                };
                for pos in &mut curr_positions {
                    graph
                        .entry(*pos)
                        .or_default()
                        .insert(dir);
                    let (dx, dy) = dir.coord_change();
                    pos.0 += dx;
                    pos.1 += dy;
                    graph
                        .entry(*pos)
                        .or_default()
                        .insert(dir.opposite());
                }
            }
            '(' => {
                branches_startpos.push(curr_positions.clone());
                branches_endpos.push(Vec::new());
            }
            '|' => {
                // save all explored positions so future paths after the branch take these into account
                // then set the explored positions for the start of the new branch to the state
                // before the branch started
                if let Some(last) = branches_endpos.last_mut() {
                    let to_add: Vec<_> = curr_positions
                        .iter()
                        .copied()
                        .filter(|p| !last.contains(p))
                        .collect();
                    last.extend(to_add);
                    // Reset to the start of the deepest branch
                    curr_positions = branches_startpos.last().unwrap().clone();
                }
            }
            ')' => {
                // take all end positions from the deepest branch and merge them with the current
                // positions, this ensures all explored positions during the deepest level branch
                // are considered in future moves
                if let Some(last) = branches_endpos.pop() {
                    let to_add: Vec<_> = last
                        .into_iter()
                        .filter(|p| !curr_positions.contains(p))
                        .collect();
                    curr_positions.extend(to_add);
                    branches_startpos.pop();
                }
            }
            _ => panic!("Invalid character: {}", c),
        }
    }
    graph
}

fn bfs<F>(start: (i16, i16), map: &HashMap<(i16, i16), HashSet<Door>>, score_fn: F) -> u32
where
    F: Fn(u32, u32) -> u32,
{
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back((start, 0));
    seen.insert(start);

    let mut score = 0;
    while let Some((coord, dist)) = q.pop_front() {
        score = score_fn(score, dist);
        if let Some(doors) = map.get(&coord) {
            for door in Door::ALL {
                if doors.contains(&door) {
                    let (dx, dy) = door.coord_change();
                    let new_coord = (coord.0 + dx, coord.1 + dy);
                    if seen.insert(new_coord) {
                        q.push_back((new_coord, dist + 1));
                    }
                }
            }
        }
    }
    score
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let chars = input
            .chars()
            .take(input.len() - 2)
            .skip(1)
            .collect::<Vec<_>>();

        Ok(Self(chars))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let map = build_graph(&self.0);
        let furthest = bfs((0, 0), &map, |score, dist| score.max(dist));
        Ok(furthest)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let map = build_graph(&self.0);
        let count = bfs(
            (0, 0),
            &map,
            |score, dist| if dist >= 1_000 { score + 1 } else { score },
        );
        Ok(count)
    }

    fn solve(self) -> AoCResult<aoc_core::Solution>
    where
        Self: Sized,
    {
        let map = build_graph(&self.0);
        Ok(Solution {
            part1: Box::new(bfs((0, 0), &map, |score, dist| score.max(dist))),
            part2: Box::new(bfs((0, 0), &map, |score, dist| {
                if dist >= 1000 {
                    score + 1
                } else {
                    score
                }
            })),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "31");
    }
}
