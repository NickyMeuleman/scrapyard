use crate::{AoCData, AoCResult};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Type {
    Bludgeoning,
    Cold,
    Fire,
    Slashing,
    Radiation,
}
impl Type {
    fn new(s: &str) -> Self {
        match s.trim() {
            "bludgeoning" => Self::Bludgeoning,
            "cold" => Self::Cold,
            "fire" => Self::Fire,
            "radiation" => Self::Radiation,
            "slashing" => Self::Slashing,
            s => panic!("invalid input: {s}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Team {
    Immune,
    Infection,
}

#[derive(Debug, Clone)]
struct Group {
    id: usize,
    team: Team,
    amount: u32,
    hp: u32,
    weak: Vec<Type>,
    immune: Vec<Type>,
    atk_dmg: u32,
    atk_type: Type,
    initiative: u32,
    target: Option<usize>,
}

impl Group {
    // Each group also has an effective power: the number of units in that group multiplied by their attack damage.
    fn effective_power(&self) -> u32 {
        self.amount * self.atk_dmg
    }

    // By default, an attacking group would deal damage equal to its effective power to the defending group.
    // However, if the defending group is immune to the attacking group's attack type, the defending group instead takes no damage;
    // if the defending group is weak to the attacking group's attack type, the defending group instead takes double damage.
    fn dmg_against(&self, other: &Self) -> u32 {
        let multiplier = match &self.atk_type {
            atk_type if other.immune.contains(atk_type) => 0,
            atk_type if other.weak.contains(atk_type) => 2,
            _ => 1,
        };
        self.effective_power() * multiplier
    }

    fn new(s: &str, team: Team, id: usize) -> Self {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let amount = parts[0].parse().unwrap();
        let hp = parts[4].parse().unwrap();
        let initiative = parts.last().unwrap().parse().unwrap();
        let atk_dmg = parts[parts.len() - 6].parse().unwrap();
        let atk_type = Type::new(parts[parts.len() - 5]);
        let mut weak = Vec::new();
        let mut immune = Vec::new();

        if let (Some(start), Some(end)) = (s.find('('), s.find(')')) {
            for part in s[start + 1..end].split(';') {
                let part = part.trim();
                if let Some(rest) = part.strip_prefix("weak to ") {
                    weak.extend(rest.split(',').map(Type::new));
                } else if let Some(rest) = part.strip_prefix("immune to ") {
                    immune.extend(rest.split(',').map(Type::new));
                }
            }
        }

        Group {
            id,
            team,
            amount,
            hp,
            weak,
            immune,
            atk_dmg,
            atk_type,
            initiative,
            target: None,
        }
    }
}

fn targeting_phase(groups: &mut [Group]) {
    // Clear previous targets
    for g in groups.iter_mut() {
        g.target = None;
    }

    // In decreasing order of effective power, groups choose their targets;
    // in a tie, the group with the higher initiative chooses first.
    let mut order: Vec<usize> = (0..groups.len()).collect();
    order.sort_unstable_by(|&a, &b| {
        groups[b]
            .effective_power()
            .cmp(&groups[a].effective_power())
            .then_with(|| {
                groups[b]
                    .initiative
                    .cmp(&groups[a].initiative)
            })
    });

    // Defending groups can only be chosen as a target by one attacking group.
    let mut taken = std::collections::HashSet::new();

    for atk_idx in order {
        let attacker = &groups[atk_idx];

        // Groups never have zero or negative units; instead, the group is removed from combat.
        if attacker.amount == 0 {
            continue;
        }

        // Pick target among untaken enemies
        let target_id = groups
            .iter()
            .filter(|group| {
                group.amount > 0 && group.team != attacker.team && !taken.contains(&group.id)
            })
            // The attacking group chooses to target the group in the enemy army
            // to which it would deal the most damage
            // (after accounting for weaknesses and immunities,
            // but not accounting for whether the defending group has enough units to actually receive all of that damage).
            // If an attacking group is considering two defending groups to which it would deal equal damage,
            // it chooses to target the defending group with the largest effective power;
            // if there is still a tie, it chooses the defending group with the highest initiative.
            .max_by(|a, b| {
                attacker
                    .dmg_against(a)
                    .cmp(&attacker.dmg_against(b))
                    .then_with(|| {
                        a.effective_power()
                            .cmp(&b.effective_power())
                    })
                    .then_with(|| a.initiative.cmp(&b.initiative))
            })
            // If it cannot deal any defending groups damage, it does not choose a target.
            .and_then(|target| {
                if attacker.dmg_against(target) > 0 {
                    Some(target.id)
                } else {
                    None
                }
            });

        // Set target for attacker
        if let Some(tgt_id) = target_id {
            groups[atk_idx].target = Some(tgt_id);
            taken.insert(tgt_id);
        }
    }
}

fn attack_phase(groups: &mut [Group]) -> u32 {
    // Groups attack in decreasing order of initiative, regardless of whether they are part of the infection or the immune system.
    let mut order: Vec<usize> = (0..groups.len()).collect();
    order.sort_unstable_by(|&a, &b| {
        groups[b]
            .initiative
            .cmp(&groups[a].initiative)
    });

    let mut total_killed = 0;
    for &atk_idx in &order {
        let attacker = &groups[atk_idx];
        // If a group contains no units, it cannot attack.
        if attacker.amount == 0 {
            continue;
        }
        if let Some(tgt_id) = attacker.target {
            if let Some(def_idx) = groups
                .iter()
                .position(|g| g.id == tgt_id)
            {
                let defender = &groups[def_idx];
                // The defending group only loses whole units from damage;
                // damage is always dealt in such a way that it kills the most units possible,
                // and any remaining damage to a unit that does not immediately kill it is ignored.
                let dmg = attacker.dmg_against(defender);
                let killed = dmg / defender.hp;
                // Deal damage to defender
                if killed > 0 {
                    total_killed += killed;
                    groups[def_idx].amount = defender.amount.saturating_sub(killed);
                }
            }
        }
    }
    total_killed
}

// Return if fight ended: (winning_team, units_left)
fn check_end(groups: &[Group]) -> Option<(Team, u32)> {
    let (immune, infection) = groups
        .iter()
        .fold((0, 0), |(immune, infection), group| match group.team {
            Team::Immune => (immune + group.amount, infection),
            Team::Infection => (immune, infection + group.amount),
        });
    match (immune, infection) {
        (0, _) => Some((Team::Infection, infection)),
        (_, 0) => Some((Team::Immune, immune)),
        _ => None,
    }
}

// Executes fight to completion, either with a winner or a stalemate
fn fight(groups: &mut [Group]) -> Option<(Team, u32)> {
    loop {
        targeting_phase(groups);
        let killed = attack_phase(groups);
        if killed == 0 {
            // stalemate
            return None;
        }
        if let Some(result) = check_end(groups) {
            // one team won
            return Some(result);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Data(Vec<Group>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let (immune, infection) = input.split_once("\n\n").unwrap();
        let mut groups = Vec::new();
        let mut id = 0;
        for line in immune.lines().skip(1) {
            groups.push(Group::new(line, Team::Immune, id));
            id += 1;
        }
        for line in infection.lines().skip(1) {
            groups.push(Group::new(line, Team::Infection, id));
            id += 1;
        }
        Ok(Self(groups))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut groups = self.0.clone();
        if let Some((_, units)) = fight(&mut groups) {
            return Ok(units);
        }
        panic!("Fight does not end")
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        for boost in 1.. {
            let mut groups = self.0.clone();
            for g in &mut groups {
                if g.team == Team::Immune {
                    g.atk_dmg += boost;
                }
            }
            if let Some((Team::Immune, units)) = fight(&mut groups) {
                return Ok(units);
            }
        }
        panic!("No solution found")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "5216");
    }

    #[test]
    fn part_2() {
        let input = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "51");
    }
}
