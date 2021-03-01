enum Boi {
    Opening(BoiType),
    Closing(BoiType),
}

#[derive(PartialEq)]
enum BoiType {
    Curly,
    Round,
    Square,
}

pub fn brackets_are_balanced(string: &str) -> bool {
    fn parse(string: &str) -> impl Iterator<Item = Boi> + '_ {
        string.chars().filter_map(|c| match c {
            '{' => Some(Boi::Opening(BoiType::Curly)),
            '[' => Some(Boi::Opening(BoiType::Square)),
            '(' => Some(Boi::Opening(BoiType::Round)),
            '}' => Some(Boi::Closing(BoiType::Curly)),
            ']' => Some(Boi::Closing(BoiType::Square)),
            ')' => Some(Boi::Closing(BoiType::Round)),
            _ => None,
        })
    }

    parse(string)
        .try_fold(Vec::new(), |mut acc, boi| match boi {
            Boi::Opening(boi_type) => {
                acc.push(boi_type);
                Some(acc)
            }
            Boi::Closing(boi_type) => {
                if acc.pop() == Some(boi_type) {
                    Some(acc)
                } else {
                    None
                }
            }
        })
        .map_or(false, |v| v.is_empty())
}

// The joyless, boiless version below 👇
// pub fn brackets_are_balanced(string: &str) -> bool {
//     let mut stack = Vec::new();
//     for c in string.chars() {
//         match c {
//             '(' | '[' | '{' => stack.push(c),
//             ')' => {
//                 if stack.pop() != Some('(') {
//                     return false;
//                 }
//             }
//             ']' => {
//                 if stack.pop() != Some('[') {
//                     return false;
//                 }
//             }
//             '}' => {
//                 if stack.pop() != Some('{') {
//                     return false;
//                 }
//             }
//             _ => (),
//         }
//     }
//     stack.is_empty()
// }
