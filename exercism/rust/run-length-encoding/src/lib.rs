pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut chars = source.chars().peekable();
    let mut num = 0;
    while let Some(c) = chars.next() {
        num += 1;
        if chars.peek() != Some(&c) {
            if num > 1 {
                result.push_str(&num.to_string())
            }
            result.push(c);
            num = 0;
        }
    }
    result
}

// pub fn decode(source: &str) -> String {
//     let mut result = String::new();
//     let mut num = String::new();
//     for c in source.chars() {
//         if c.is_numeric() {
//             num.push(c);
//         } else {
//             let repeat = num.parse().unwrap_or(1);
//             let repeated = c.to_string().repeat(repeat);
//             result.push_str(&repeated);
//             // reset num for next loop
//             num.clear();
//         }
//     }
//     result
// }

pub fn decode(source: &str) -> String {
    let nums = source
        .split(|c: char| !c.is_numeric())
        .map(|num| num.parse().unwrap_or(1));
    let chars = source.chars().filter(|&c: &char| !c.is_numeric());
    // equivalent method to get chars, with split(), like nums above
    // let chars = source
    //     .split(char::is_numeric)
    //     .flat_map(str::chars);
    nums.zip(chars)
        .map(|(n, c)| c.to_string().repeat(n))
        .collect()
}
