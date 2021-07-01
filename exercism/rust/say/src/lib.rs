pub fn encode(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        15 => "fifteen".to_string(),
        18 => "eighteen".to_string(),
        14 | 16 | 17 | 19 => format!("{}teen", encode(n - 10)),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        80 => "eighty".to_string(),
        60 | 70 | 90 => format!("{}ty", encode(n / 10)),
        21..=99 => format!("{}-{}", encode(10 * (n / 10)), encode(n % 10)),
        100..=u64::MAX => {
            let (order_str, order_num) = get_order(n);
            let before_order = n / order_num;
            let rest_of_num = n % order_num;
            match rest_of_num {
                0 => format!("{} {}", encode(before_order), order_str),
                _ => format!(
                    "{} {} {}",
                    encode(before_order),
                    order_str,
                    encode(rest_of_num)
                ),
            }
        }
    }
}

fn get_order<'a>(n: u64) -> (&'a str, u64) {
    match n {
        100..=999 => ("hundred", 100),
        1000..=999_999 => ("thousand", 1_000),
        1_000_000..=999_999_999 => ("million", 1_000_000),
        1_000_000_000..=999_999_999_999 => ("billion", 1_000_000_000),
        1_000_000_000_000..=999_999_999_999_999 => ("trillion", 1_000_000_000_000),
        1_000_000_000_000_000..=999_999_999_999_999_999 => ("quadrillion", 1_000_000_000_000_000),
        1_000_000_000_000_000_000..=u64::MAX => ("quintillion", 1_000_000_000_000_000_000),
        _ => panic!("Called function with number that is too small"),
    }
}
