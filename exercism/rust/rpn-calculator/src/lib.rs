#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    inputs
        .iter()
        .try_fold(Vec::new(), |mut acc, input| match input {
            CalculatorInput::Value(n) => {
                acc.push(*n);
                Some(acc)
            }
            operation => match (acc.pop(), acc.pop()) {
                (Some(one), Some(two)) => {
                    let result = match operation {
                        CalculatorInput::Add => two + one,
                        CalculatorInput::Subtract => two - one,
                        CalculatorInput::Multiply => two * one,
                        CalculatorInput::Divide => two / one,
                        CalculatorInput::Value(_) => unreachable!(),
                    };
                    acc.push(result);
                    Some(acc)
                }
                _ => None,
            },
        })
        .map(|mut stack| if stack.len() == 1 { stack.pop() } else { None })
        .flatten()
}
