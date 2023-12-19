use std::fmt::Display;

use crate::{AoCData, AoCResult};

use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Debug, Clone, Copy)]
enum Rule<'a> {
    Check(Check<'a>),
    Last(Dest<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Final {
    Accept,
    Reject,
}

#[derive(Debug, Clone, Copy)]
struct Check<'a> {
    part: PartKind,
    operator: Operator,
    rhs: usize,
    dest: Dest<'a>,
}

#[derive(Debug, Clone, Copy)]
enum Dest<'a> {
    WorkFlow(&'a str),
    Final(Final),
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, Copy)]
enum PartKind {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug, Clone)]
struct PartRange {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl Part {
    fn accepted(&self, loc: Dest, workflows: &HashMap<&str, Vec<Rule>>) -> bool {
        match loc {
            Dest::Final(end) => matches!(end, Final::Accept),
            Dest::WorkFlow(name) => {
                for rule in workflows.get(name).unwrap() {
                    match rule {
                        Rule::Check(check) => {
                            let n = match check.part {
                                PartKind::X => self.x,
                                PartKind::M => self.m,
                                PartKind::A => self.a,
                                PartKind::S => self.s,
                            };
                            match check.operator {
                                Operator::LessThan if n < check.rhs => {
                                    return self.accepted(check.dest, workflows)
                                }
                                Operator::GreaterThan if n > check.rhs => {
                                    return self.accepted(check.dest, workflows)
                                }
                                _ => (),
                            }
                        }
                        Rule::Last(dest) => return self.accepted(*dest, workflows),
                    }
                }
                panic!("Could not determine if part is accepted")
            }
        }
    }

    // non-recursive version
    //     fn accepted(&self, workflows: &HashMap<&str, Vec<Rule>>) -> bool {
    //         let mut curr = workflows.get("in").unwrap();
    //         loop {
    //             #[allow(unused_assignments)]
    //             let mut dest = Dest::WorkFlow("in");
    //             for rule in curr {
    //                 match rule {
    //                     Rule::Check(check) => {
    //                         let n = match check.part {
    //                             PartKind::X => self.x,
    //                             PartKind::M => self.m,
    //                             PartKind::A => self.a,
    //                             PartKind::S => self.s,
    //                         };
    //                         match check.operator {
    //                             Operator::LessThan if n < check.rhs => {
    //                                 dest = check.dest;
    //                                 break;
    //                             }
    //                             Operator::GreaterThan if n > check.rhs => {
    //                                 dest = check.dest;
    //                                 break;
    //                             }
    //                             _ => continue,
    //                         }
    //                     }
    //                     Rule::Last(last) => {
    //                         dest = *last;
    //                         break;
    //                     }
    //                 };
    //             }
    //             match dest {
    //                 Dest::WorkFlow(name) => {
    //                     curr = workflows.get(name).unwrap();
    //                 }
    //                 Dest::Final(accepted) => {
    //                     if accepted == Final::Accept {
    //                         return true;
    //                     } else {
    //                         return false;
    //                     }
    //                 }
    //             }
    //         }
    //     }
}

fn parse_workflows(s: &str) -> HashMap<&str, Vec<Rule>> {
    s.lines()
        .map(|line| {
            let line = line.strip_suffix("}").unwrap();
            let (name, rules) = line.split_once("{").unwrap();
            let (checks, final_dest) = rules.rsplit_once(",").unwrap();
            let last_dest = match final_dest {
                "A" => Dest::Final(Final::Accept),
                "R" => Dest::Final(Final::Reject),
                name => Dest::WorkFlow(name),
            };

            let rules = checks
                .split(",")
                .map(|check| {
                    let (check, dest) = check.split_once(":").unwrap();
                    let part = match &check[0..1] {
                        "x" => PartKind::X,
                        "m" => PartKind::M,
                        "a" => PartKind::A,
                        "s" => PartKind::S,
                        _ => panic!("Invalid part kind"),
                    };
                    let operator = match &check[1..2] {
                        "<" => Operator::LessThan,
                        ">" => Operator::GreaterThan,
                        _ => panic!("Invalid operator"),
                    };
                    let rhs: usize = check[2..].parse().unwrap();
                    let dest = match dest {
                        "A" => Dest::Final(Final::Accept),
                        "R" => Dest::Final(Final::Reject),
                        name => Dest::WorkFlow(name),
                    };

                    Check {
                        part,
                        operator,
                        rhs,
                        dest,
                    }
                })
                .map(|check| Rule::Check(check))
                .chain(std::iter::once(match last_dest {
                    Dest::WorkFlow(_) => Rule::Last(last_dest),
                    Dest::Final(_) => Rule::Last(last_dest),
                }))
                .collect();

            (name, rules)
        })
        .collect()
}

fn parse_parts(s: &str) -> Vec<Part> {
    s.lines()
        .map(|line| {
            let line = line
                .strip_prefix("{")
                .unwrap()
                .strip_suffix("}")
                .unwrap();
            line.split(",")
                .map(|s| s.split_once("=").unwrap())
                .fold(
                    Part {
                        x: 0,
                        m: 0,
                        a: 0,
                        s: 0,
                    },
                    |mut part, (xmas, n)| {
                        let n = n.parse().unwrap();
                        match xmas {
                            "x" => part.x = n,
                            "m" => part.m = n,
                            "a" => part.a = n,
                            "s" => part.s = n,
                            _ => panic!("Inval xmas part id"),
                        };
                        part
                    },
                )
        })
        .collect()
}

impl PartRange {
    fn accepted(&self, loc: Dest, workflows: &HashMap<&str, Vec<Rule>>) -> Vec<Self> {
        match loc {
            Dest::Final(end) => {
                if matches!(end, Final::Accept) {
                    vec![self.clone()]
                } else {
                    vec![]
                }
            }
            Dest::WorkFlow(name) => {
                // keep track of a valid range inside this workflow, updated when a rule fails
                let mut working_range = self.clone();
                let mut valid_ranges = Vec::new();

                for rule in workflows.get(name).unwrap() {
                    match rule {
                        Rule::Check(check) => {
                            let range = match check.part {
                                PartKind::X => working_range.x.clone(),
                                PartKind::M => working_range.m.clone(),
                                PartKind::A => working_range.a.clone(),
                                PartKind::S => working_range.s.clone(),
                            };
                            match check.operator {
                                Operator::LessThan => {
                                    if *range.end() < check.rhs {
                                        valid_ranges
                                            .extend(working_range.accepted(check.dest, workflows))
                                    } else {
                                        if *range.start() >= check.rhs {
                                            // total failure, end current workflow
                                            break;
                                        }
                                        let new_range = *range.start()..=check.rhs - 1;
                                        let mut new_partrange = working_range.clone();
                                        match check.part {
                                            PartKind::X => new_partrange.x = new_range,
                                            PartKind::M => new_partrange.m = new_range,
                                            PartKind::A => new_partrange.a = new_range,
                                            PartKind::S => new_partrange.s = new_range,
                                        };
                                        valid_ranges
                                            .extend(new_partrange.accepted(check.dest, workflows));
                                        // update working range so it would have passed
                                        working_range = new_partrange;
                                    }
                                }
                                Operator::GreaterThan => {
                                    if *range.start() > check.rhs {
                                        valid_ranges
                                            .extend(working_range.accepted(check.dest, workflows))
                                    } else {
                                        if *range.end() <= check.rhs {
                                            // total failure, end current workflow
                                            break;
                                        }
                                        let new_range = check.rhs..=*range.end();
                                        let mut new_partrange = working_range.clone();
                                        match check.part {
                                            PartKind::X => new_partrange.x = new_range,
                                            PartKind::M => new_partrange.m = new_range,
                                            PartKind::A => new_partrange.a = new_range,
                                            PartKind::S => new_partrange.s = new_range,
                                        };
                                        valid_ranges
                                            .extend(new_partrange.accepted(check.dest, workflows));
                                        // update working range so it would have passed
                                        working_range = new_partrange;
                                    }
                                }
                            }
                        }
                        Rule::Last(dest) => {
                            valid_ranges.extend(working_range.accepted(*dest, workflows))
                        }
                    }
                }

                valid_ranges
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Data<'a>(HashMap<&'a str, Vec<Rule<'a>>>, Vec<Part>);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        let (workflows, parts) = input.split_once("\n\n").unwrap();
        Ok(Self(parse_workflows(workflows), parse_parts(parts)))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let sum: usize = self
            .1
            .iter()
            .filter(|part| part.accepted(Dest::WorkFlow("in"), &self.0))
            // non-recursive version
            // .filter(|part| part.accepted(&workflows))
            .map(|part| part.x + part.m + part.a + part.s)
            .sum();

        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let valid_ranges = PartRange {
            x: (1..=4_000),
            m: (1..=4_000),
            a: (1..=4_000),
            s: (1..=4_000),
        }
        .accepted(Dest::WorkFlow("in"), &self.0);

        let sum: usize = valid_ranges
            .into_iter()
            .map(|valid_range| {
                (valid_range.x.end() - valid_range.x.start() + 1)
                    * (valid_range.m.end() - valid_range.m.start() + 1)
                    * (valid_range.a.end() - valid_range.a.start() + 1)
                    * (valid_range.s.end() - valid_range.s.start() + 1)
            })
            .sum();

        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "");
    }

    #[test]
    fn part_2() {
        let input = "";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "");
    }
}
