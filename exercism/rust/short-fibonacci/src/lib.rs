const FIRST_FIB: u8 = 1;
const SECOND_FIB: u8 = 1;

/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    Vec::new()
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut result = vec![FIRST_FIB, SECOND_FIB];
    let mut last = FIRST_FIB;
    let mut current = SECOND_FIB;
    // range starts at 3, because the first two iterations are handled as constants
    // The position is a one based number, not zero based
    for _ in 3..=5 {
        let new = last + current;
        result.push(new);
        last = current;
        current = new;
    }
    result
}
