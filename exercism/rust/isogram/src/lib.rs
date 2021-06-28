use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let letters: Vec<char> = candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();
    letters.len() == letters.into_iter().collect::<HashSet<char>>().len()
}

// pub fn check(candidate: &str) -> bool {
//     let letters: Vec<char> = candidate
//         .to_lowercase()
//         .chars()
// This errors out for reasons I don't fully understand
// https://doc.rust-lang.org/nomicon/hrtb.html#:~:text=Higher%2DRank%20Trait%20Bounds%20(HRTBs)&text=for%20can%20be,sugar%20for%20the%20common%20cases
//         .filter(char::is_alphabetic)
//         .collect();
//     letters.len() == letters.into_iter().collect::<HashSet<char>>().len()
// }