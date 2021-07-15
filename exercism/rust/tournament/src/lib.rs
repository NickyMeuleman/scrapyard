use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt::Display;
use std::iter;

#[derive(Default)]
struct Team {
    wins: u8,
    losses: u8,
    draws: u8,
}

impl Team {
    fn points(&self) -> u8 {
        self.wins * 3 + self.draws
    }

    fn played(&self) -> u8 {
        self.wins + self.losses + self.draws
    }
}

fn table_format(
    args: (
        impl Display,
        impl Display,
        impl Display,
        impl Display,
        impl Display,
        impl Display,
    ),
) -> String {
    format!(
        "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
        args.0, args.1, args.2, args.3, args.4, args.5
    )
}

pub fn tally(match_results: &str) -> String {
    let results: BTreeMap<&str, Team> =
        match_results
            .lines()
            .fold(BTreeMap::new(), |mut acc, line| {
                let parts: Vec<&str> = line.split(";").collect();
                let [name1, name2, outcome] = <[&str; 3]>::try_from(parts).unwrap();

                match outcome {
                    "win" => {
                        let mut team1 = acc.entry(name1).or_default();
                        team1.wins += 1;
                        let mut team2 = acc.entry(name2).or_default();
                        team2.losses += 1;
                    }
                    "loss" => {
                        let mut team1 = acc.entry(name1).or_default();
                        team1.losses += 1;
                        let mut team2 = acc.entry(name2).or_default();
                        team2.wins += 1;
                    }
                    "draw" => {
                        let mut team1 = acc.entry(name1).or_default();
                        team1.draws += 1;
                        let mut team2 = acc.entry(name2).or_default();
                        team2.draws += 1;
                    }
                    _ => unreachable!(),
                };

                acc
            });

    let mut results: Vec<(&str, Team)> = results.into_iter().collect();
    results.sort_by(|(_, team1), (_, team2)| team2.points().cmp(&team1.points()));

    iter::once(table_format(("Team", "MP", "W", "D", "L", "P")))
        .chain(results.into_iter().map(|(name, team)| {
            table_format((
                name,
                team.played(),
                team.wins,
                team.draws,
                team.losses,
                team.points(),
            ))
        }))
        .collect::<Vec<String>>()
        .join("\n")
}
