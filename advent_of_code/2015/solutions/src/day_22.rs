use std::{cmp::Ordering, collections::BinaryHeap, fmt::Display};

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Boss);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    mana_spent: u32,
    game: Game,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.mana_spent.cmp(&self.mana_spent)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Boss {
    hit_points: u8,
    damage: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Player {
    hit_points: u8,
    armor: u8,
    mana: u16,
}

#[derive(Debug)]
enum Spell {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    player: Player,
    boss: Boss,
    effect_timers: Effects,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Effects {
    shield: u8,
    poison: u8,
    recharge: u8,
}

impl Game {
    fn new(boss: Boss) -> Self {
        Self {
            player: Player {
                hit_points: 50,
                armor: 0,
                mana: 500,
            },
            boss,
            effect_timers: Effects {
                shield: 0,
                poison: 0,
                recharge: 0,
            },
        }
    }

    fn run_effects(&mut self) {
        self.player.armor = 0;
        if self.effect_timers.shield > 0 {
            self.player.armor = 7;
            self.effect_timers.shield = self
                .effect_timers
                .shield
                .saturating_sub(1);
        }
        if self.effect_timers.poison > 0 {
            self.boss.hit_points = self.boss.hit_points.saturating_sub(3);
            self.effect_timers.poison = self
                .effect_timers
                .poison
                .saturating_sub(1);
        }
        if self.effect_timers.recharge > 0 {
            // is there a max amount of mana? Question doesn't say, using u16::MAX
            self.player.mana = self.player.mana.saturating_add(101);
            self.effect_timers.recharge = self
                .effect_timers
                .recharge
                .saturating_sub(1);
        }
    }

    /// returns the amount of mana that was spent
    fn cast_spell(&mut self, spell: &Spell) -> u8 {
        let mana = match spell {
            Spell::Missile => {
                self.boss.hit_points = self.boss.hit_points.saturating_sub(4);
                53
            }
            Spell::Drain => {
                self.boss.hit_points = self.boss.hit_points.saturating_sub(2);
                // is there a max amount of player hp? Question doesn't say, using u8::MAX
                self.player.hit_points = self.player.hit_points.saturating_add(2);
                73
            }
            Spell::Shield => {
                self.effect_timers.shield = 6;
                113
            }
            Spell::Poison => {
                self.effect_timers.poison = 6;
                173
            }
            Spell::Recharge => {
                self.effect_timers.recharge = 5;
                229
            }
        };
        self.player.mana = self
            .player
            .mana
            .saturating_sub(u16::from(mana));
        mana
    }

    fn possible_spells(&self) -> Vec<Spell> {
        let mut result = Vec::new();
        if self.player.mana >= 53 {
            result.push(Spell::Missile);
        }
        if self.player.mana >= 73 {
            result.push(Spell::Drain);
        }
        if self.player.mana >= 113 && self.effect_timers.shield <= 1 {
            result.push(Spell::Shield);
        }
        if self.player.mana >= 173 && self.effect_timers.poison <= 1 {
            result.push(Spell::Poison);
        }
        if self.player.mana >= 229 && self.effect_timers.recharge <= 1 {
            result.push(Spell::Recharge);
        }
        result
    }

    fn min_mana_to_win(self, hard: bool) -> u32 {
        // dijkstra
        let mut min_cost = u32::MAX;
        // priority queue with ordering so a Node with the lowest mana_spent (because of the Ord trait implementation) gets popped first
        let mut pq = BinaryHeap::new();

        pq.push(Node {
            game: self,
            mana_spent: 0,
        });

        'outer: while let Some(Node {
            mut game,
            mana_spent,
        }) = pq.pop()
        {
            // did we pop a dead boss?
            if game.boss.hit_points == 0 {
                return mana_spent;
            }

            if hard {
                game.player.hit_points = game.player.hit_points.saturating_sub(1);
                if game.player.hit_points == 0 {
                    continue;
                }
            }

            game.run_effects();

            if game.boss.hit_points == 0 {
                // don't return from here, it might not be the best route
                min_cost = min_cost.min(mana_spent);
                pq.push(Node { mana_spent, game });
                continue;
            }

            for spell in game.possible_spells() {
                // new state for each spell
                let mut game = game.clone();

                let spent = game.cast_spell(&spell);

                if game.boss.hit_points == 0 {
                    min_cost = min_cost.min(mana_spent);
                    pq.push(Node {
                        mana_spent: mana_spent + u32::from(spent),
                        game,
                    });
                    continue 'outer;
                }

                // bail out of paths that are longer than the current shortest
                if mana_spent > min_cost {
                    continue;
                }

                game.run_effects();

                if game.boss.hit_points == 0 {
                    min_cost = min_cost.min(mana_spent);
                    pq.push(Node {
                        mana_spent: mana_spent + u32::from(spent),
                        game,
                    });
                    continue 'outer;
                }

                let boss_damage = (game
                    .boss
                    .damage
                    .saturating_sub(game.player.armor))
                .max(1);
                game.player.hit_points = game
                    .player
                    .hit_points
                    .saturating_sub(boss_damage);

                if game.player.hit_points == 0 {
                    continue 'outer;
                }

                // if we reached this neither the player nor the boss is dead, keep going
                pq.push(Node {
                    mana_spent: mana_spent + u32::from(spent),
                    game,
                });
            }
        }
        // should only get here if there is no way to win (so min_cost is MAX)
        min_cost
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let (hit_points, damage) = input
            .lines()
            .map(|line| line.split_once(": ").unwrap().1)
            .collect_tuple()
            .ok_or(AoCError::Parsing)?;

        Ok(Self(Boss {
            hit_points: hit_points.parse()?,
            damage: damage.parse()?,
        }))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(Game::new(self.0).min_mana_to_win(false))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(Game::new(self.0).min_mana_to_win(true))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Hit Points: 58
Damage: 9
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "1269");
    }

    #[test]
    fn part_2() {
        let input = "Hit Points: 58
Damage: 9
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "1309");
    }
}
