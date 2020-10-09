// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

// what about foreign alphabets? characters with diacritics?
fn sentence_to_pig_latin(string: &str) -> String {
    string
        .split(" ")
        .map(word_to_pig_latin)
        .collect::<Vec<_>>()
        .join(" ")
}

fn word_to_pig_latin(string: &str) -> String {
    // 1. entire word is alphabetic
    // 2. entire word is not alphabetic
    // 3. contains mix: split in two parts and use recursion to handle part of the word.
    if string == "" {
        "".to_string()
    } else if string.chars().all(|c| c.is_alphabetic()) {
        let mut iter = string.chars();
        let first_letter = iter.next().unwrap();
        if is_vowel(&first_letter) {
            format!("{}{}-hay", first_letter, iter.as_str())
        } else {
            format!("{}-{}ay", iter.as_str(), first_letter)
        }
    } else if string.chars().all(|c| !c.is_alphabetic()) {
        string.to_string()
    } else {
        // if you only use the symbol_index the stack will overflow on a symbol followed by a letter eg: 'a
        // if you only use the alphabetic_index the stack will overflow on a letter followed by a symbol eg: a'
        let symbol_index = string.find(|c: char| !c.is_alphabetic()).unwrap();
        let alphabetic_index = string.find(|c: char| c.is_alphabetic()).unwrap();
        // get max to prevent the index being 0
        // can't just set the index to 1, because the symbol might be longer than one character, that's why we look to the alphabetic one too
        let index = symbol_index.max(alphabetic_index);
        let (left, right) = string.split_at(index);
        format!("{}{}", word_to_pig_latin(left), word_to_pig_latin(right))
    }
}

fn is_vowel(letter: &char) -> bool {
    // TODO: include other languages
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    // the following assumes to_lowercase will return an iterator of size one
    let letter = &letter.to_lowercase().next().unwrap();
    VOWELS.contains(letter)
}

fn main() {
    let mut sentence = String::new();
    println!("Please enter a sentence to translate into pig latin.");
    std::io::stdin()
        .read_line(&mut sentence)
        .expect("Cannot read the input");

    let translated = sentence_to_pig_latin(&sentence.trim());
    println!("Original sentence: {}", sentence);
    println!("Translated: {}", translated);
}
