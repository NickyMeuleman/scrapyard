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

    if string.chars().all(|c| c.is_alphabetic()) {
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
        // I hate this part of the code, YUCK
        // TODO: punctation has more options than the comma
        let parts: Vec<_> = string.splitn(2, ",").collect();
        format!(
            "{},{}",
            if parts[0] == "" {
                "".to_string()
            } else {
                word_to_pig_latin(parts[0])
            },
            if parts[1] == "" {
                "".to_string()
            } else {
                word_to_pig_latin(parts[1])
            }
        )
    }
}

fn is_vowel(letter: &char) -> bool {
    // TODO: include other languages
    // weird things happen with letter.to_lowercase() so the const has both lower and uppercase
    const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    VOWELS.contains(letter)
}

fn main() {
    // Oya Lele, Ik voel me plots weer zo oya lele, Doe het nog een keer zo oya lele, zing met ons mee
    let mut sentence = String::new();
    println!("Please enter a sentence to translate into pig latin.");
    std::io::stdin()
        .read_line(&mut sentence)
        .expect("Cannot read the input");
    let translated = sentence_to_pig_latin(&sentence.trim());

    println!("Original sentence: {}", sentence);
    println!("Translated: {}", translated);
}
