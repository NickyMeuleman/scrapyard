use std::collections::HashMap;

use crate::AoCData;

enum Monkey<'a> {
    Num(i64),
    // (operator, lhs, rhs)
    Calculated(Operator, &'a str, &'a str),
}

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Data<'a>(HashMap<&'a str, Monkey<'a>>);

fn calc_name(name: &str, monkeys: &HashMap<&str, Monkey>) -> i64 {
    match &monkeys[name] {
        Monkey::Num(n) => *n,
        Monkey::Calculated(operator, lhs, rhs) => {
            let lhs_num = calc_name(lhs, monkeys);
            let rhs_num = calc_name(rhs, monkeys);
            match operator {
                Operator::Add => lhs_num + rhs_num,
                Operator::Sub => lhs_num - rhs_num,
                Operator::Mul => lhs_num * rhs_num,
                Operator::Div => lhs_num / rhs_num,
            }
        }
    }
}

fn depends_on_human(name: &str, monkeys: &HashMap<&str, Monkey>) -> bool {
    if name == "humn" {
        return true;
    }
    match &monkeys[name] {
        Monkey::Num(_) => false,
        Monkey::Calculated(_, lhs, rhs) => {
            depends_on_human(lhs, monkeys) || depends_on_human(rhs, monkeys)
        }
    }
}

/// calc human assuming the evaluated name and the passed value are equal
fn calc_human(name: &str, value: i64, monkeys: &HashMap<&str, Monkey>) -> i64 {
    if name == "humn" {
        return value;
    }

    match &monkeys[name] {
        // never gets hit
        Monkey::Num(n) => *n,
        // reorder all operations to solve for unknown side
        Monkey::Calculated(operator, lhs, rhs) => {
            // lhs + rhs = value
            // lhs - rhs = value
            // lhs * rhs = value
            // lhs / rhs = value
            let (new_name, new_value) = if depends_on_human(lhs, monkeys) {
                let rhs_num = calc_name(rhs, monkeys);
                let new_value = match operator {
                    // lhs = value - rhs
                    Operator::Add => value - rhs_num,
                    // lhs = value + rhs
                    Operator::Sub => value + rhs_num,
                    // lhs = value / rhs
                    Operator::Mul => value / rhs_num,
                    // lhs = value * rhs
                    Operator::Div => value * rhs_num,
                };
                (lhs, new_value)
            } else {
                let lhs_num = calc_name(lhs, monkeys);
                let new_value = match operator {
                    // rhs = value - lhs
                    Operator::Add => value - lhs_num,
                    // rhs = lhs - value
                    Operator::Sub => lhs_num - value,
                    // rhs = value / lhs
                    Operator::Mul => value / lhs_num,
                    // rhs = lhs / value
                    Operator::Div => lhs_num / value,
                };
                (rhs, new_value)
            };

            calc_human(new_name, new_value, monkeys)
        }
    }
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> Option<Self> {
        let monkeys = input
            .lines()
            .map(|line| {
                let (name, right) = line.split_once(": ").unwrap();
                let monkey = match right.parse() {
                    Ok(n) => Monkey::Num(n),
                    Err(_) => {
                        let mut iter = right.split_ascii_whitespace();
                        let lhs = iter.next().unwrap();
                        let operator = match iter.next().unwrap() {
                            "+" => Operator::Add,
                            "-" => Operator::Sub,
                            "*" => Operator::Mul,
                            "/" => Operator::Div,
                            _ => panic!("Invalid math operator"),
                        };
                        let rhs = iter.next().unwrap();
                        Monkey::Calculated(operator, lhs, rhs)
                    }
                };

                (name, monkey)
            })
            .collect();

        Some(Self(monkeys))
    }

    fn part_1(&self) -> String {
        calc_name("root", &self.0).to_string()
    }

    fn part_2(&self) -> String {
        // which side of the "tree" (hehe, a monkey tree) is "humn" in
        let Monkey::Calculated(_, lhs, rhs) = &self.0["root"] else {
            panic!("root has to be a calculated monkey");
        };

        let (name, value) = if depends_on_human(lhs, &self.0) {
            let rhs_num = calc_name(rhs, &self.0);
            (lhs, rhs_num)
        } else {
            let lhs_num = calc_name(lhs, &self.0);
            (rhs, lhs_num)
        };

        calc_human(name, value, &self.0).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(21);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "152");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(21);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_2(), "301");
    }
}
