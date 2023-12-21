use std::fmt::Display;

use aoc_core::AoCError;
use itertools::Itertools;

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

#[derive(Debug, Clone)]
enum Apply {
    FullPass,
    FullFail,
    Split { pass: PartRange, fail: PartRange },
}

impl Part {
    fn accepted(&self, loc: Dest, workflows: &HashMap<&str, Vec<Rule>>) -> AoCResult<bool> {
        match loc {
            Dest::Final(end) => Ok(matches!(end, Final::Accept)),
            Dest::WorkFlow(name) => {
                for rule in workflows
                    .get(name)
                    .ok_or(AoCError::Solving)?
                {
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
                Err(AoCError::Solving)
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

fn parse_workflows(s: &str) -> AoCResult<HashMap<&str, Vec<Rule>>> {
    s.lines()
        .map(|line| {
            let line = line
                .strip_suffix("}")
                .ok_or(AoCError::Parsing)?;
            let (name, rules) = line
                .split_once("{")
                .ok_or(AoCError::Parsing)?;
            let (checks, final_dest) = rules
                .rsplit_once(",")
                .ok_or(AoCError::Parsing)?;
            let last_dest = match final_dest {
                "A" => Dest::Final(Final::Accept),
                "R" => Dest::Final(Final::Reject),
                name => Dest::WorkFlow(name),
            };

            let rules: Vec<Rule> = checks
                .split(",")
                .map(|check| {
                    let (check, dest) = check
                        .split_once(":")
                        .ok_or(AoCError::Parsing)?;
                    let part = match &check[0..1] {
                        "x" => PartKind::X,
                        "m" => PartKind::M,
                        "a" => PartKind::A,
                        "s" => PartKind::S,
                        _ => return Err(AoCError::Parsing),
                    };
                    let operator = match &check[1..2] {
                        "<" => Operator::LessThan,
                        ">" => Operator::GreaterThan,
                        _ => return Err(AoCError::Parsing),
                    };
                    let rhs: usize = check[2..]
                        .parse()
                        .map_err(|_| AoCError::Parsing)?;
                    let dest = match dest {
                        "A" => Dest::Final(Final::Accept),
                        "R" => Dest::Final(Final::Reject),
                        name => Dest::WorkFlow(name),
                    };

                    Ok(Check {
                        part,
                        operator,
                        rhs,
                        dest,
                    })
                })
                .map_ok(|check| Rule::Check(check))
                .collect::<AoCResult<_>>()?;

            let rules = rules
                .into_iter()
                .chain(std::iter::once(match last_dest {
                    Dest::WorkFlow(_) => Rule::Last(last_dest),
                    Dest::Final(_) => Rule::Last(last_dest),
                }))
                .collect();

            Ok((name, rules))
        })
        .collect()
}

fn parse_parts(s: &str) -> AoCResult<Vec<Part>> {
    s.lines()
        .map(|line| {
            let line = line
                .strip_prefix("{")
                .ok_or(AoCError::Parsing)?
                .strip_suffix("}")
                .ok_or(AoCError::Parsing)?;

            let part: Part = line
                .split(",")
                .map(|s| {
                    s.split_once("=")
                        .ok_or(AoCError::Parsing)
                })
                .try_fold(
                    Part {
                        x: 0,
                        m: 0,
                        a: 0,
                        s: 0,
                    },
                    |mut part, item| {
                        let (xmas, n) = item?;
                        let n = n
                            .parse()
                            .map_err(|_| AoCError::Parsing)?;
                        match xmas {
                            "x" => part.x = n,
                            "m" => part.m = n,
                            "a" => part.a = n,
                            "s" => part.s = n,
                            _ => return Err(AoCError::Parsing),
                        };
                        Ok(part)
                    },
                )?;

            Ok(part)
        })
        .collect()
}

impl PartRange {
    fn set_xmas(mut self, kind: &PartKind, range: RangeInclusive<usize>) -> Self {
        match kind {
            PartKind::X => self.x = range,
            PartKind::M => self.m = range,
            PartKind::A => self.a = range,
            PartKind::S => self.s = range,
        };
        self
    }

    fn applies_to(&self, check: &Check) -> Apply {
        let range = match check.part {
            PartKind::X => self.x.clone(),
            PartKind::M => self.m.clone(),
            PartKind::A => self.a.clone(),
            PartKind::S => self.s.clone(),
        };
        match check.operator {
            Operator::LessThan => {
                if *range.end() < check.rhs {
                    // start -- end -- rhs
                    Apply::FullPass
                } else if check.rhs <= *range.start() {
                    // rhs -- start -- end
                    Apply::FullFail
                } else {
                    // start -- rhs -- end
                    // passing: start..=rhs-1
                    // failing: rhs..=end
                    let pass_range = *range.start()..=(check.rhs - 1);
                    let fail_range = check.rhs..=*range.end();
                    Apply::Split {
                        pass: self
                            .clone()
                            .set_xmas(&check.part, pass_range),
                        fail: self
                            .clone()
                            .set_xmas(&check.part, fail_range),
                    }
                }
            }
            Operator::GreaterThan => {
                if *range.start() > check.rhs {
                    // rhs -- start -- end
                    Apply::FullPass
                } else if *range.end() <= check.rhs {
                    // start -- end -- rhs
                    Apply::FullFail
                } else {
                    // start -- rhs -- end
                    // passing: rhs+1..=end
                    // failing: start..=rhs
                    let pass_range = (check.rhs + 1)..=*range.end();
                    let fail_range = *range.start()..=check.rhs;
                    Apply::Split {
                        pass: self
                            .clone()
                            .set_xmas(&check.part, pass_range),
                        fail: self
                            .clone()
                            .set_xmas(&check.part, fail_range),
                    }
                }
            }
        }
    }

    fn accepted(&self, loc: Dest, workflows: &HashMap<&str, Vec<Rule>>) -> AoCResult<Vec<Self>> {
        match loc {
            Dest::Final(end) => {
                if matches!(end, Final::Accept) {
                    Ok(vec![self.clone()])
                } else {
                    Ok(vec![])
                }
            }
            Dest::WorkFlow(name) => {
                // keep track of a valid range inside this workflow
                let mut working_range = self.clone();
                let mut valid_ranges = Vec::new();

                for rule in workflows
                    .get(name)
                    .ok_or(AoCError::Solving)?
                {
                    match rule {
                        Rule::Check(check) => {
                            match working_range.applies_to(check) {
                                Apply::FullPass => valid_ranges
                                    .extend(working_range.accepted(check.dest, workflows)?),
                                Apply::FullFail => (),
                                Apply::Split { pass, fail } => {
                                    // move onto new destination with passing range
                                    valid_ranges.extend(pass.accepted(check.dest, workflows)?);
                                    // move onto next check in this workflow, update the working range
                                    working_range = fail;
                                }
                            }
                        }
                        Rule::Last(dest) => {
                            valid_ranges.extend(working_range.accepted(*dest, workflows)?)
                        }
                    }
                }

                Ok(valid_ranges)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Data<'a>(HashMap<&'a str, Vec<Rule<'a>>>, Vec<Part>);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        let (workflows, parts) = input
            .split_once("\n\n")
            .ok_or(AoCError::Parsing)?;

        Ok(Self(parse_workflows(workflows)?, parse_parts(parts)?))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let sum = self
            .1
            .iter()
            .map(|&part| {
                let pass = part.accepted(Dest::WorkFlow("in"), &self.0)?;
                Ok((pass, part))
            })
            // non-recursive version
            // .filter(|part| part.accepted(&workflows))
            .filter_ok(|(pass, _)| *pass)
            .map_ok(|(_, part)| part.x + part.m + part.a + part.s)
            .sum::<AoCResult<usize>>()?;

        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let valid_ranges = PartRange {
            x: (1..=4_000),
            m: (1..=4_000),
            a: (1..=4_000),
            s: (1..=4_000),
        }
        .accepted(Dest::WorkFlow("in"), &self.0)?;

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
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "19114");
    }

    #[test]
    fn part_2() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "167409079868000");
    }
}
