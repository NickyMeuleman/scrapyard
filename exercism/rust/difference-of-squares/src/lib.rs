pub fn square_of_sum(n: u32) -> u32 {
    // Sum of arithmetic sequence from 1 to n, squared
    (n * (1 + n) / 2).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // https://en.wikipedia.org/wiki/Square_pyramidal_number
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
