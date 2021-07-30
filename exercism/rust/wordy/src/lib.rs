pub fn answer(command: &str) -> Option<i32> {
    let mut instructions = command
        .trim_start_matches("What is ")
        .trim_end_matches('?')
        .split_whitespace()
        .peekable();

    let mut lhs: i32 = instructions.next().and_then(|s| s.parse().ok())?;

    // no order of operations rules, process from left to right
    // continually reassign lhs to the result of the next operation
    // for every operation, return None early if an unexpected iterator item shows up
    while let Some(s) = instructions.next() {
        lhs = match s {
            "plus" => {
                let rhs: i32 = instructions.next().and_then(|s| s.parse().ok())?;
                lhs + rhs
            }
            "minus" => {
                let rhs: i32 = instructions.next().and_then(|s| s.parse().ok())?;
                lhs - rhs
            }
            "multiplied" => {
                instructions.next_if_eq(&"by")?;
                let rhs: i32 = instructions.next().and_then(|s| s.parse().ok())?;
                lhs * rhs
            }
            "divided" => {
                instructions.next_if_eq(&"by")?;
                let rhs: i32 = instructions.next().and_then(|s| s.parse().ok())?;
                lhs / rhs
            }
            "raised" => {
                instructions.next_if_eq(&"to")?;
                instructions.next_if_eq(&"the")?;
                let rhs = instructions
                    .next()
                    .and_then(|s| s.trim_end_matches(char::is_alphabetic).parse().ok())?;
                instructions.next_if_eq(&"power")?;
                lhs.pow(rhs)
            }
            _ => return None,
        }
    }

    Some(lhs)
}
