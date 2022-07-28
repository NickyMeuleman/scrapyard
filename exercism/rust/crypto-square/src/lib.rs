pub fn encrypt(input: &str) -> String {
    let normalized: Vec<char> = input
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();
    let width = (normalized.len() as f64).sqrt().ceil() as usize;

    (0..width)
        .map(|i| {
            normalized
                .chunks(width)
                .map(|chunk| chunk.get(i).unwrap_or(&' '))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
