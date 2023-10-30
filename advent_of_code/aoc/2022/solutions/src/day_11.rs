use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Monkey>);

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

#[derive(Debug, Clone)]
enum Operation {
    Mul(Value),
    Add(Value),
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        match &self {
            Operation::Add(val) => old + val.number(old),
            Operation::Mul(val) => old * val.number(old),
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    Old,
    Num(u64),
}

impl Value {
    fn number(&self, old: u64) -> u64 {
        match self {
            Value::Num(n) => *n,
            Value::Old => old,
        }
    }
}
#[derive(Debug, Clone)]
struct Test {
    divisible: u64,
    true_recipient: usize,
    false_recipient: usize,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut monkeys = Vec::new();
        for block in input.split("\n\n") {
            let mut lines = block.lines().skip(1);

            let (_, items) = lines
                .next()
                .ok_or(AoCError::new("Failed Parsing"))?
                .split_once(": ")
                .ok_or(AoCError::new("Failed Parsing"))?;
            let items = items
                .split_terminator(", ")
                .filter_map(|s| s.parse().ok())
                .collect();

            let (_, operation) = lines
                .next()
                .ok_or(AoCError::new("Failed Parsing"))?
                .split_once("= old ")
                .ok_or(AoCError::new("Failed Parsing"))?;
            let (operator, operand) = operation
                .split_once(' ')
                .ok_or(AoCError::new("Failed Parsing"))?;
            let operand = match operand {
                "old" => Value::Old,
                _ => {
                    let n = operand.parse()?;
                    Value::Num(n)
                }
            };

            let (_, divisible) = lines
                .next()
                .ok_or(AoCError::new("Failed Parsing"))?
                .rsplit_once(' ')
                .ok_or(AoCError::new("Failed Parsing"))?;
            let divisible = divisible.parse()?;
            let (_, true_recipient) = lines
                .next()
                .ok_or(AoCError::new("Failed Parsing"))?
                .rsplit_once(' ')
                .ok_or(AoCError::new("Failed Parsing"))?;
            let true_recipient = true_recipient.parse()?;
            let (_, false_recipient) = lines
                .next()
                .ok_or(AoCError::new("Failed Parsing"))?
                .rsplit_once(' ')
                .ok_or(AoCError::new("Failed Parsing"))?;
            let false_recipient = false_recipient.parse()?;

            let operation = match operator {
                "+" => Operation::Add(operand),
                "*" => Operation::Mul(operand),
                _ => panic!("Inalid input"),
            };

            let test = Test {
                divisible,
                true_recipient,
                false_recipient,
            };

            let monkey = Monkey {
                items,
                operation,
                test,
            };

            monkeys.push(monkey);
        }

        Ok(Self(monkeys))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut monkeys = self.0.clone();
        let mut inspections = vec![0; monkeys.len()];

        for _ in 0..20 {
            for idx in 0..monkeys.len() {
                let items: Vec<u64> = monkeys[idx].items.drain(..).collect();
                let monkey = monkeys[idx].clone();
                for old in items {
                    // inspect
                    inspections[idx] += 1;
                    // operation
                    let new = monkey.operation.apply(old);
                    // relieved
                    let new = new / 3;
                    // test
                    let idx = if new % monkey.test.divisible == 0 {
                        monkey.test.true_recipient
                    } else {
                        monkey.test.false_recipient
                    };
                    let receiver = &mut monkeys[idx];
                    // throw
                    receiver.items.push(new);
                }
            }
        }

        inspections.sort_unstable();
        let result: u64 = inspections
            .iter()
            .rev()
            .take(2)
            .product();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut monkeys = self.0.clone();
        let mut inspections = vec![0; monkeys.len()];
        let common_multiple: u64 = monkeys
            .iter()
            .map(|monkey| monkey.test.divisible)
            .product();

        for _ in 0..10_000 {
            for idx in 0..monkeys.len() {
                let items: Vec<u64> = monkeys[idx].items.drain(..).collect();
                let monkey = monkeys[idx].clone();
                for old in items {
                    // inspect
                    inspections[idx] += 1;
                    // operation
                    let new = monkey.operation.apply(old);
                    // not relieved
                    let new = new % common_multiple;
                    // test
                    let idx = if new % monkey.test.divisible == 0 {
                        monkey.test.true_recipient
                    } else {
                        monkey.test.false_recipient
                    };
                    let receiver = &mut monkeys[idx];
                    // throw
                    receiver.items.push(new);
                }
            }
        }

        inspections.sort_unstable();
        let result: u64 = inspections
            .iter()
            .rev()
            .take(2)
            .product();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "10605");
    }

    #[test]
    fn part_2() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "2713310158");
    }
}
