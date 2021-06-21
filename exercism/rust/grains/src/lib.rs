pub fn square(s: u32) -> u64 {
    // find num in geometric series that starts at 1 and has a ratio of 2
    match s {
       1..=64 => 2u64.pow(s - 1),
       _ => panic!("Square must be between 1 and 64")
    }
}

pub fn total() -> u128 {
    // sum of the geometric series
    2u128.pow(64) - 1
    // The sum is exactly u64::MAX, but I didn't know that so I made it a u128 and subtracted 1
}
