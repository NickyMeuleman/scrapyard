use crate::AoCData;

pub struct Data(Vec<Monkey>);

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
struct Test {
    divisible: u64,
    true_recipient: usize,
    false_recipient: usize,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> Option<Self> {
        let mut monkeys = Vec::new();
        for block in input.split("\n\n") {
            let mut lines = block.lines().skip(1);

            let (_, items) = lines.next()?.split_once(": ")?;
            let items = items
                .split_terminator(", ")
                .filter_map(|s| s.parse().ok())
                .collect();

            let (_, operation) = lines.next()?.split_once("= old ")?;
            let (operator, operand) = operation.split_once(' ')?;
            let operand = match operand {
                "old" => Value::Old,
                _ => {
                    let n = operand.parse().ok()?;
                    Value::Num(n)
                }
            };

            let (_, divisible) = lines.next()?.rsplit_once(' ')?;
            let divisible = divisible.parse().ok()?;
            let (_, true_recipient) = lines.next()?.rsplit_once(' ')?;
            let true_recipient = true_recipient.parse().ok()?;
            let (_, false_recipient) = lines.next()?.rsplit_once(' ')?;
            let false_recipient = false_recipient.parse().ok()?;

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

        Some(Self(monkeys))
    }

    fn part_1(&self) -> String {
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
        inspections.iter().rev().take(2).product::<u64>().to_string()
    }

    fn part_2(&self) -> String {
        let mut monkeys = self.0.clone();
        let mut inspections = vec![0; monkeys.len()];
        let common_multiple: u64 = monkeys.iter().map(|monkey| monkey.test.divisible).product();
    
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
        inspections.iter().rev().take(2).product::<u64>().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(11);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "10605");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(11);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_2(), "2713310158");
    }
}
