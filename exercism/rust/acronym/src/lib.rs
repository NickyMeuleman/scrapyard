use std::iter;

pub fn abbreviate(phrase: &str) -> String {
    let prepended = iter::once(' ').chain(phrase.chars());
    let appended = phrase.chars().chain(iter::once(' '));
    prepended
        .zip(appended)
        .filter_map(|(left, right)| {
            let include_right = "-_ ".contains(left) && right.is_alphabetic()
                || left.is_lowercase() && right.is_uppercase();
            match include_right {
                true => Some(right.to_ascii_uppercase()),
                false => None,
            }
        })
        .collect()
}
