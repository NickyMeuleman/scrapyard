/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let numbers: Vec<u32> = isbn
        .chars()
        .filter_map(|c| match c {
            'X' if Some('X') == isbn.chars().last() => Some(10),
            _ => c.to_digit(10),
        })
        .collect();
    if numbers.len() != 10 {
        return false;
    };
    let sum: u32 = (1..=10).rev().zip(numbers).map(|(n1, n2)| n1 * n2).sum();
    sum % 11 == 0
}
