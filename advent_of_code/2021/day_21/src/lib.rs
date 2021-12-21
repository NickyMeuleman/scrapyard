use std::{convert::Infallible, str::FromStr};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Data {
    starts: (u8, u8),
}

#[derive(Debug, Clone)]
struct Rules {
    min_pos: u8,
    max_pos: u8,
    win_score: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Player {
    pos: u8,
    score: u16,
}

#[derive(Debug, Clone)]
struct Game {
    turn: u32,
    die: u8,
    p1: Player,
    p2: Player,
    rules: Rules,
}

#[derive(Debug, Clone)]
struct Stats {
    p1_wins: u64,
    p2_wins: u64,
}

impl Game {
    fn new(p1_start: u8, p2_start: u8, board_size: u8, win_score: u16) -> Self {
        Self {
            turn: 1,
            die: 1,
            p1: Player::new(p1_start),
            p2: Player::new(p2_start),
            rules: Rules {
                min_pos: 1,
                max_pos: board_size,
                win_score,
            },
        }
    }

    fn turn(&mut self) -> Option<(u16, u32)> {
        let roll = self.die + (self.die + 1) + (self.die + 2);
        self.die = self.rules.wrap(self.die + 3);
        let (active_player, other_player) = if self.turn % 2 == 1 {
            (&mut self.p1, &self.p2)
        } else {
            (&mut self.p2, &self.p1)
        };
        let new_pos = self.rules.wrap(active_player.pos + roll);
        active_player.pos = new_pos;
        active_player.score += u16::from(new_pos);
        if active_player.score >= self.rules.win_score {
            return Some((other_player.score, self.turn));
        }
        self.turn += 1;
        None
    }
}

impl Rules {
    fn wrap(&self, num: u8) -> u8 {
        ((num - self.min_pos) % (self.max_pos + 1 - self.min_pos)) + self.min_pos
    }
}

impl Player {
    fn new(start_pos: u8) -> Self {
        Self {
            pos: start_pos,
            score: 0,
        }
    }

    fn win_stats(
        self,
        other: Player,
        rules: &Rules,
        memo: &mut HashMap<(Player, Player), Stats>,
    ) -> Stats {
        if self.score >= rules.win_score {
            return Stats::new(1, 0);
        }
        if other.score >= rules.win_score {
            return Stats::new(0, 1);
        }

        if let Some(stats) = memo.get(&(self.clone(), other.clone())) {
            return stats.clone();
        }

        let mut result = Stats::new(0, 0);

        for one in 1..=3 {
            for two in 1..=3 {
                for three in 1..=3 {
                    let mut next_player = self.clone();
                    next_player.pos = rules.wrap(self.pos + one + two + three);
                    next_player.score += u16::from(next_player.pos);

                    let next_state = other.clone().win_stats(next_player, rules, memo);

                    result.p1_wins += next_state.p2_wins;
                    result.p2_wins += next_state.p1_wins;
                }
            }
        }

        memo.insert((self, other), result.clone());

        result
    }
}

impl Data {
    pub fn part_one(&self) -> u32 {
        let mut game = Game::new(self.starts.0, self.starts.1, 10, 1000);

        loop {
            if let Some((losing_score, num_turns)) = game.turn() {
                // 3 rolls of the die per turn
                break u32::from(losing_score) * num_turns * 3;
            }
        }
    }

    pub fn part_two(&self) -> u64 {
        // very clever solution from /u/Sykout09 on reddit
        // const P2DIEOUTCOMES: [(usize, usize); 7] =
        //     [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        // fn part2_playersimulate(
        //     pos: usize,
        //     scoreleft: usize,
        //     step: usize,
        //     stepmultiplier: usize,
        //     stepscore: &mut [(usize, usize)],
        // ) {
        //     if let Some(((stepwins, steplose), stepscore)) = stepscore.split_first_mut() {
        //         let step = step + 1;
        //         for (rollnum, multiplier) in P2DIEOUTCOMES {
        //             let pos = pos + rollnum;
        //             let pos = if pos > 9 { pos - 10 } else { pos };
        //             let score = if pos == 0 { 10 } else { pos };
        //             let unipaths = multiplier * stepmultiplier;
        //             if score >= scoreleft {
        //                 *stepwins += unipaths;
        //             } else {
        //                 *steplose += unipaths;
        //                 part2_playersimulate(pos, scoreleft - score, step, unipaths, stepscore);
        //             }
        //         }
        //     }
        // }

        // const POINTS: usize = 21;
        // let mut p1steps = [(0, 0); POINTS];
        // let mut p2steps = [(0, 0); POINTS];
        // part2_playersimulate(self.starts.0 as usize, POINTS, 0, 1, &mut p1steps);
        // part2_playersimulate(self.starts.1 as usize, POINTS, 0, 1, &mut p2steps);
        // let p1wins: usize = p1steps[0..POINTS - 1]
        //     .iter()
        //     .skip(1)
        //     .zip(p2steps[0..POINTS - 1].iter())
        //     .map(|((winspath, _), (_, losepath))| winspath * losepath)
        //     .sum();
        // let p2wins: usize = p2steps[0..POINTS]
        //     .iter()
        //     .zip(p1steps[0..POINTS].iter())
        //     .map(|((winspath, _), (_, losepath))| winspath * losepath)
        //     .sum();
        // if p1wins > p2wins {
        //     p1wins
        // } else {
        //     p2wins
        // }

        let p1 = Player::new(self.starts.0);

        let p2 = Player::new(self.starts.1);

        let rules = Rules {
            min_pos: 1,
            max_pos: 10,
            win_score: 21,
        };

        let mut memo: HashMap<(Player, Player), Stats> = HashMap::new();

        let stats = p1.win_stats(p2, &rules, &mut memo);

        stats.p1_wins.max(stats.p2_wins)
    }
}

impl Stats {
    fn new(p1_wins: u64, p2_wins: u64) -> Self {
        Self { p1_wins, p2_wins }
    }
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let starts: Vec<u8> = input
            .trim()
            .lines()
            .map(|line| {
                let (_, num) = line.rsplit_once(" ").unwrap();
                num.parse().unwrap()
            })
            .collect();

        Ok(Self {
            starts: (starts[0], starts[1]),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "Player 1 starting position: 4
Player 2 starting position: 8";

        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 739785);
    }

    #[ignore]
    #[test]
    fn part_two() {
        let input = "Player 1 starting position: 4
Player 2 starting position: 8";

        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 444356092776315);
    }
}
