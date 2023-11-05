use std::{collections::HashMap, fmt::Display};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

const P1_WIN_TRESHOLD: u16 = 1000;
const P2_WIN_TRESHOLD: u8 = 21;
const BOARD_LEN: u8 = 10;
// all possible dirac 3-roll sums
// 3 4 4 5 5 5 6 6 7
// 4 5 5 6 6 6 7 7 8
// 5 6 6 7 7 7 8 8 9
// many of them occur multiple times, this maps a roll-sum to the amount of times it occurs
const HISTOGRAM: [(u8, u8); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Debug, Clone)]
pub struct Data {
    p1_start: u8,
    p2_start: u8,
}

struct DeterministicDie {
    rolls: std::iter::Cycle<std::ops::RangeInclusive<u8>>,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Player {
    pos: u8,
    score: u16,
}

struct Game {
    p1: Player,
    p2: Player,
    turn: u32,
    die: DeterministicDie,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    p1: Player,
    p2: Player,
    is_p1_turn: bool,
}

impl State {
    fn count_wins(self, memo: &mut HashMap<State, (i64, i64)>) -> (i64, i64) {
        if self.p1.score >= P2_WIN_TRESHOLD.into() {
            return (1, 0);
        }
        if self.p2.score >= P2_WIN_TRESHOLD.into() {
            return (0, 1);
        }

        let mut res = (0, 0);

        for (sum_rolls, freq) in HISTOGRAM {
            let mut next_state = self;
            // apply turn
            if self.is_p1_turn {
                next_state.p1.pos = (self.p1.pos + sum_rolls - 1) % BOARD_LEN + 1;
                next_state.p1.score = self.p1.score + next_state.p1.pos as u16;
            } else {
                next_state.p2.pos = (self.p2.pos + sum_rolls - 1) % BOARD_LEN + 1;
                next_state.p2.score = self.p2.score + next_state.p2.pos as u16;
            }
            next_state.is_p1_turn = !next_state.is_p1_turn;
            // check how many wins each player has with that new state
            let (p1_wins, p2_wins) = match memo.get(&next_state) {
                Some(&res) => res,
                None => {
                    // calculate result
                    let res = next_state.count_wins(memo);
                    // cache result
                    memo.insert(next_state, res);
                    res
                }
            };
            // that result occurred freq times, add it to the total
            res.0 += p1_wins * freq as i64;
            res.1 += p2_wins * freq as i64;
        }
        res
    }
}

impl DeterministicDie {
    fn roll(&mut self) -> u8 {
        self.rolls.next().unwrap()
    }
}

impl Game {
    fn turn(&mut self) -> Option<(u16, u32)> {
        let (active_player, other_player) = if self.turn % 2 == 1 {
            (&mut self.p1, &self.p2)
        } else {
            (&mut self.p2, &self.p1)
        };
        let roll_sum =
            u16::from(self.die.roll()) + u16::from(self.die.roll()) + u16::from(self.die.roll());

        let new_pos =
            ((u16::from(active_player.pos) + roll_sum - 1) % u16::from(BOARD_LEN) + 1) as u8;

        active_player.pos = new_pos;
        active_player.score += u16::from(new_pos);

        if active_player.score >= P1_WIN_TRESHOLD {
            return Some((other_player.score, self.turn));
        }

        self.turn += 1;

        None
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let starts: Vec<u8> = input
            .trim()
            .lines()
            .map(|line| {
                let (_, num) = line.rsplit_once(" ")?;
                num.parse().ok()
            })
            .collect::<Option<Vec<u8>>>()
            .ok_or(AoCError::Parsing)?;
        let p1_start = *starts.get(0).ok_or(AoCError::Parsing)?;
        let p2_start = *starts.get(1).ok_or(AoCError::Parsing)?;

        Ok(Self { p1_start, p2_start })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut game = Game {
            p1: Player {
                pos: self.p1_start,
                score: 0,
            },
            p2: Player {
                pos: self.p2_start,
                score: 0,
            },
            die: DeterministicDie {
                rolls: (1..=100).cycle(),
            },
            turn: 1,
        };

        loop {
            if let Some((losing_score, num_turns)) = game.turn() {
                // 3 rolls of the die per turn
                return Ok(u32::from(losing_score) * num_turns * 3);
            }
        }
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let starting_state = State {
            p1: Player {
                pos: self.p1_start,
                score: 0,
            },
            p2: Player {
                pos: self.p2_start,
                score: 0,
            },
            is_p1_turn: true,
        };
        let (p1_wins, p2_wins) = starting_state.count_wins(&mut HashMap::new());

        Ok(p1_wins.max(p2_wins))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Player 1 starting position: 4
Player 2 starting position: 8";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "739785");
    }

    #[test]
    fn part_2() {
        let input = "Player 1 starting position: 4
Player 2 starting position: 8";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "444356092776315");
    }
}
