use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_lowercase()
        .chars()
        .filter(|&c| c >= 'a' && c <= 'z')
        .collect::<HashSet<char>>()
        .len()
        == 26
}

// pub fn is_pangram(sentence: &str) -> bool {
//     let sentence = sentence.to_ascii_lowercase();
//     ('a'..='z').all(|c| sentence.contains(c))
// }