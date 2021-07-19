// lightly edited version of Kosteg's solution
const VOWEL_SOUNDS: [&str; 7] = ["a", "e", "i", "o", "u", "xr", "yt"];

pub fn translate(input: &str) -> String {
    input
        .split(' ')
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn translate_word(word: &str) -> String {
    let (consonants, remains) = word.split_at(first_vowel_index(word));
    format!("{}{}ay", remains, consonants)
}

pub fn first_vowel_index(word: &str) -> usize {
    if VOWEL_SOUNDS.iter().any(|&s| word.starts_with(s)) {
        0
    } else if word.starts_with("qu") {
        2
    } else if word.chars().nth(1) == Some('y') {
        1
    } else {
        1 + first_vowel_index(&word[1..])
    }
}

// const VOWELS: &str = "aeuio";

// pub fn translate_word(word: &str) -> String {
//     let (consonants, remains) = word.split_at(first_vowel_index(word));
//     format!("{}{}ay", remains, consonants)
// }

// pub fn first_vowel_index(word: &str) -> usize {
//     if word.is_empty()
//         || word.starts_with("xr")
//         || word.starts_with("yt")
//         || VOWELS.contains(word.chars().nth(0).unwrap())
//     {
//         0
//     } else if word.starts_with("qu") {
//         2
//     } else if word.chars().nth(1) == Some('y') {
//         1
//     } else {
//         1 + first_vowel_index(&word[1..])
//     }
// }

// pub fn first_vowel_index(word: &str) -> usize {
//     if word.is_empty() || word.starts_with("xr") || word.starts_with("yt") {
//         return 0;
//     }

//     let mut count = 0;

//     for i in 0..word.len() - 1 {
//         let current = word.chars().nth(i).unwrap();
//         let next = word.chars().nth(i + 1).unwrap();

//         if VOWELS.contains(current) {
//             return count;
//         }
//         if current == 'q' && next == 'u' {
//             return count + 2;
//         }

//         count += 1;

//         if next == 'y' {
//             return count;
//         }
//     }

//     return count;
// }
