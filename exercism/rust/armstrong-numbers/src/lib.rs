pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let num_len = num_str.len() as u32;

    num == num_str
        .chars()
        .filter_map(|n| n.to_digit(10))
        .map(|n| n.pow(num_len))
        .sum()
}

// pub fn is_armstrong_number(num: u32) -> bool {
//     let digits: Vec<_> = num
//         .to_string()
//         .chars()
//         .filter_map(|n| n.to_digit(10))
//         .collect();
//     let num_digits = digits.len() as u32;
//     num == digits.iter().map(|n| n.pow(num_digits)).sum()
// }

// worse readability and worse performance
// pub fn is_armstrong_number(num: u32) -> bool {
//     num == num
//         .to_string()
//         .chars()
//         .filter_map(|n| n.to_digit(10))
//         .map(|n| n.pow(num.to_string().len() as u32))
//         .sum()
// }

// digits is an iterator
// pub fn is_armstrong_number(num: u32) -> bool {
//     let num_str = num.to_string();
//     let digits = num_str.chars().filter_map(|n| n.to_digit(10));
//     let num_digits = digits.clone().count();
//     num == digits.map(|n| n.pow(num_digits as u32)).sum()
// }

// pub fn is_armstrong_number(num: u32) -> bool {
//     let mut curr = num;
//     let num_digits = num.to_string().chars().count();
//     let mut sum = 0;
//     while curr > 0 {
//         let digit = curr % 10;
//         curr /= 10;
//         sum += digit.pow(num_digits as u32)
//     }
//     sum == num
// }
