/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut code = code.to_string();
    code.retain(|c| !c.is_whitespace());
    if code.len() <= 1 || code.contains(|c: char| !c.is_digit(10)) {
        return false;
    }

    let sum: u32 = code
        .chars()
        .filter_map(|c| c.to_digit(10))
        .rev()
        .enumerate()
        .map(|(i, n)| {
            if i % 2 == 0 {
                n
            } else {
                let double = n * 2;
                if double > 9 {
                    double - 9
                } else {
                    double
                }
            }
        })
        .sum();

    sum % 10 == 0 
}