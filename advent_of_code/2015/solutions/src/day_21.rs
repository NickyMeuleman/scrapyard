use std::{fmt::Display, iter};

use aoc_core::{AoCError, Solution};
use itertools::{iproduct, Itertools};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Stats);

#[derive(Debug, Clone)]
struct Stats {
    hit_points: u16,
    damage: u16,
    armor: u16,
}

#[derive(Debug, Clone)]
struct Item<'a> {
    name: &'a str,
    cost: u16,
    damage: u16,
    armor: u16,
}

const WEAPONS: [Item; 5] = [
    Item {
        name: "Dagger",
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        name: "Shortsword",
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        name: "Warhammer",
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        name: "Longsword",
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        name: "Greataxe",
        cost: 74,
        damage: 8,
        armor: 0,
    },
];
const ARMORS: [Item; 5] = [
    Item {
        name: "Leather",
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Chainmail",
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Splintmail",
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        name: "Bandedmail",
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        name: "Platemail",
        cost: 102,
        damage: 0,
        armor: 5,
    },
];
const RINGS: [Item; 6] = [
    Item {
        name: "Damage +1",
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        name: "Damage +2",
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        name: "Damage +3",
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        name: "Defense +1",
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Defense +2",
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Defense +3",
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

const fn divide_rounding_up(dividend: u16, divisor: u16) -> u16 {
    (dividend + (divisor - 1)) / divisor
}

struct Character<'a> {
    weapon: Item<'a>,
    armor: Option<Item<'a>>,
    ring1: Option<Item<'a>>,
    ring2: Option<Item<'a>>,
}

impl<'a> Character<'a> {
    fn new(
        weapon: Item<'a>,
        armor: Option<Item<'a>>,
        ring1: Option<Item<'a>>,
        ring2: Option<Item<'a>>,
    ) -> Self {
        Character {
            weapon,
            armor,
            ring1,
            ring2,
        }
    }

    fn cost(&self) -> u16 {
        self.weapon.cost
            + self
                .armor
                .as_ref()
                .map_or(0, |item| item.cost)
            + self
                .ring1
                .as_ref()
                .map_or(0, |item| item.cost)
            + self
                .ring2
                .as_ref()
                .map_or(0, |item| item.cost)
    }

    fn stats(&self, hit_points: u16) -> Stats {
        let damage = self.weapon.damage
            + self
                .armor
                .as_ref()
                .map(|a| a.damage)
                .unwrap_or(0)
            + self
                .ring1
                .as_ref()
                .map(|a| a.damage)
                .unwrap_or(0)
            + self
                .ring2
                .as_ref()
                .map(|a| a.damage)
                .unwrap_or(0);
        let armor = self.weapon.armor
            + self
                .armor
                .as_ref()
                .map(|a| a.armor)
                .unwrap_or(0)
            + self
                .ring1
                .as_ref()
                .map(|a| a.armor)
                .unwrap_or(0)
            + self
                .ring2
                .as_ref()
                .map(|a| a.armor)
                .unwrap_or(0);
        Stats {
            hit_points,
            damage,
            armor,
        }
    }

    fn defeats(&self, other: &Stats) -> bool {
        let player = self.stats(100);
        // ugh, dealing with overflow, so check damage and armor first
        let player_rounds_to_win = {
            if player.damage <= other.armor {
                other.hit_points
            } else {
                divide_rounding_up(other.hit_points, player.damage - other.armor)
            }
        };
        let other_rounds_to_win = {
            if other.damage <= player.armor {
                player.hit_points
            } else {
                divide_rounding_up(player.hit_points, other.damage - player.armor)
            }
        };

        player_rounds_to_win <= other_rounds_to_win
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let (hit_points, damage, armor) = input
            .lines()
            .map(|line| line.split_once(": ").unwrap().1)
            .collect_tuple()
            .ok_or(AoCError::Parsing)?;

        Ok(Self(Stats {
            hit_points: hit_points.parse()?,
            damage: damage.parse()?,
            armor: armor.parse()?,
        }))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let boss = self.0.clone();

        let weapon_choices = WEAPONS.iter();
        let armor_choices = ARMORS
            .iter()
            .map(Some)
            .chain(iter::once(None));
        let ring_choices = RINGS
            .iter()
            .map(Some)
            .chain(iter::once(None));

        let cheapest_win = iproduct!(
            weapon_choices,
            armor_choices,
            ring_choices.clone(),
            ring_choices
        )
        .filter(|(_, _, r1, r2)| {
            // if there are 2 rings, they must be different ones
            // empty ring slots are always allowed
            match (r1, r2) {
                (Some(r1), Some(r2)) => r1.name != r2.name,
                _ => true,
            }
        })
        .map(|(w, a, r1, r2)| Character::new(w.clone(), a.cloned(), r1.cloned(), r2.cloned()))
        .filter(|player| player.defeats(&boss))
        .min_by(|a, b| a.cost().cmp(&b.cost()))
        .unwrap();

        Ok(cheapest_win.cost())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let boss = self.0.clone();

        let weapon_choices = WEAPONS.iter();
        let armor_choices = ARMORS
            .iter()
            .map(Some)
            .chain(iter::once(None));
        let ring_choices = RINGS
            .iter()
            .map(Some)
            .chain(iter::once(None));

        let most_expensive_loss = iproduct!(
            weapon_choices,
            armor_choices,
            ring_choices.clone(),
            ring_choices
        )
        .filter(|(_, _, r1, r2)| {
            // if there are 2 rings, they must be different ones
            // empty ring slots are always allowed
            match (r1, r2) {
                (Some(r1), Some(r2)) => r1.name != r2.name,
                _ => true,
            }
        })
        .map(|(w, a, r1, r2)| Character::new(w.clone(), a.cloned(), r1.cloned(), r2.cloned()))
        .filter(|player| !player.defeats(&boss))
        .max_by(|a, b| a.cost().cmp(&b.cost()))
        .unwrap();

        Ok(most_expensive_loss.cost())
    }

    fn solve(self) -> AoCResult<Solution>
    where
        Self: Sized,
    {
        let boss = self.0;

        let weapon_choices = WEAPONS.iter();
        let armor_choices = ARMORS
            .iter()
            .map(Some)
            .chain(iter::once(None));
        let ring_choices = RINGS
            .iter()
            .map(Some)
            .chain(iter::once(None));

        let (winners, losers): (Vec<Character>, Vec<Character>) = iproduct!(
            weapon_choices,
            armor_choices,
            ring_choices.clone(),
            ring_choices
        )
        .filter(|(_, _, r1, r2)| {
            // if there are 2 rings, they must be different ones
            // empty ring slots are always allowed
            match (r1, r2) {
                (Some(r1), Some(r2)) => r1.name != r2.name,
                _ => true,
            }
        })
        .map(|(w, a, r1, r2)| Character::new(w.clone(), a.cloned(), r1.cloned(), r2.cloned()))
        .partition(|player| player.defeats(&boss));

        Ok(Solution {
            part1: Box::new(
                winners
                    .iter()
                    .map(|char| char.cost())
                    .min()
                    .unwrap_or(u16::MAX),
            ),
            part2: Box::new(
                losers
                    .iter()
                    .map(|char| char.cost())
                    .max()
                    .unwrap_or(0),
            ),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Hit Points: 100
Damage: 8
Armor: 2
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "91");
    }

    #[test]
    fn part_2() {
        let input = "Hit Points: 100
Damage: 8
Armor: 2
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "158");
    }

    #[test]
    fn solve() {
        let input = "Hit Points: 100
Damage: 8
Armor: 2
";
        let data = Data::try_new(input).unwrap();
        let Solution { part1, part2 } = data.solve().unwrap();
        assert_eq!(part1.to_string(), "91");
        assert_eq!(part2.to_string(), "158");
    }
}
