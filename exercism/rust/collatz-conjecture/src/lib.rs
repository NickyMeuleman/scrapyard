pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => {
            let num = n.checked_div(2)?;
            collatz(num).and_then(|steps| steps.checked_add(1))
        }
        _ => {
            let num = n.checked_mul(3).and_then(|n| n.checked_add(1))?;
            collatz(num).and_then(|steps| steps.checked_add(1))
        }
    }
}
