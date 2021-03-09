use std::collections::HashSet;

fn lowercase_chars(word: &str) -> Vec<char> {
    // a character can have multiple lowercase variants, the more you know 🌈
    word.chars().flat_map(|c| c.to_lowercase()).collect()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = lowercase_chars(word);
    let mut sorted_word = lowercase_word.clone();
    sorted_word.sort_unstable();
    
    possible_anagrams
    .iter()
    .copied()
    .filter(|&item| {
        let mut lowercase_item = lowercase_chars(item);
        if lowercase_word == lowercase_item {
            false
        } else {
            lowercase_item.sort_unstable();
            sorted_word == lowercase_item
        }
    })
    .collect()
}

// less efficient solution using itertools permutations() to construct all possible anagrams first 👇
// use itertools::Itertools;
//
// fn lowercase_chars(word: &str) -> impl Iterator<Item = char> + '_ {
//     word.chars().flat_map(|c| c.to_lowercase())
// }
//
// pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
//     let lowercase_word: String = lowercase_chars(word).collect();
//     let lowercase_anagrams: HashSet<String> = lowercase_chars(word)
//         .permutations(lowercase_chars(word).count())
//         .map(|v| v.iter().collect())
//         .collect();
//     let lowercase_possible_anagrams: HashSet<String> = possible_anagrams
//         .iter()
//         .map(|word| lowercase_chars(word).collect())
//         .collect();
//
//     let lowercase_intersection: HashSet<&String> = lowercase_anagrams
//         .intersection(&lowercase_possible_anagrams)
//         .collect();
//
//     possible_anagrams
//         .iter()
//         .copied()
//         .filter(|item| {
//             let lowercase_item: String = lowercase_chars(item).collect();
//             lowercase_item != lowercase_word && lowercase_intersection.contains(&lowercase_item)
//         })
//         .collect()
// }
