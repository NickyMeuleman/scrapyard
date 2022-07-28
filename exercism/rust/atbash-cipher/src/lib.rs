/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            // use the ASCII table
            if c.is_ascii_alphabetic() {
                (b'z' - c as u8 + b'a') as char
            } else {
                c
            }
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// "Decipher" with the
/// Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            // use the ASCII table
            if c.is_ascii_alphabetic() {
                (b'z' - c as u8 + b'a') as char
            } else {
                c
            }
        })
        .collect()
}