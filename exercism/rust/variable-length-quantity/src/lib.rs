// #[derive(Debug, PartialEq)]
// pub enum Error {
//     IncompleteNumber,
//     Overflow,
// }

// /// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
// pub fn to_bytes(values: &[u32]) -> Vec<u8> {
//     unimplemented!("Convert the values {:?} to a list of bytes", values)
// }

// /// Given a stream of bytes, extract all numbers which are encoded in there.
// pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
//     unimplemented!("Convert the list of bytes {:?} to a list of numbers", bytes)
// }

// I didn't understand the question fully at first, so I copied rsalmei's solution and commented it a bit
// I did learn about a bunch of new things like endians and stuff

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|&num| {
            std::iter::once(0)
                // 80 in hexadecimal is 1000_0000 in binary
                // in other words, it's a byte with the first bit 1 and the rest 0
                // a 1 in that position signals a stopping point
                .chain(std::iter::repeat(0b1000_0000))
                .scan(num, |state, byte| {
                    if *state != 0 || byte == 0 {
                        // if state != 0 byte will be 0b1000_0000, which is 0x80 in hex or 128 in decimal
                        // 7f in hexadecimal is 01111111 in binary
                        // in other words, it's a byte with the first bit 0 and the rest 1
                        // a 0 in that position signals we're not done, this isn't the last byte
                        dbg!(&state);
                        let res = (*state & 0b0111_1111) as u8;
                        // shift the binary representation of the number in state 7 places to the right
                        // the resulting smaller number is stored in state to be used in the next iteration
                        // *state >>= 7;
                        // that's equivalent to dividing by 2 to the 7th power
                        *state /= 2u32.pow(7);
                        Some(byte | res)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
        })
        .collect()
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    bytes
        .iter()
        .map(|&b| Some(b))
        .chain(std::iter::once(None))
        .scan((0u32, false), |(acc, ongoing), x| match x {
            Some(b) if b & 0x80 == 0 => {
                *ongoing = false;
                Some(Some(Ok(std::mem::take(acc) + b as u32)))
            }
            Some(b) => (acc.leading_zeros() >= 7)
                .then(|| {
                    *ongoing = true;
                    *acc = (*acc + (b & 0x7f) as u32) << 7;
                    None
                })
                .or_else(|| Some(Some(Err(Error::Overflow)))),
            None => ongoing.then(|| Some(Err(Error::IncompleteNumber))),
        })
        .flatten()
        .collect::<Result<_, _>>()
}
