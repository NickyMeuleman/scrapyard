pub fn raindrops(n: u32) -> String {
    let inputs = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

    let mut result = String::new();
    for (factor, sound) in &inputs {
        if n % factor == 0 {
            result.push_str(sound)
        }
    }

    if result.is_empty() {
       return n.to_string();
    }

    result
}
