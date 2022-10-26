use crate::AoCData;
use md5::{Digest, Md5};

pub struct Data(String);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input))
    }

    // [71.718 ms 71.938 ms 72.173 ms]
    fn part_1(&self) -> String {
        // https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7
        let mut hasher = Md5::new();
        let mut num: i32 = 0;
        let key = self.0.as_bytes();
        loop {
            hasher.update(key);
            hasher.update(num.to_string().as_bytes());
            let result = hasher.finalize_reset();
            // an item in the result array is a byte represented by 2 hex characters: 00 to FF
            // check if both hex characters at idx 0 are 0
            // check if both hex characters at idx 1 are 0
            // check if first hex character at idx 2 is 0
            if (result[0] == 0) && (result[1] == 0) && ((result[2] & 0xf0) == 0) {
                break;
            }
            num += 1;
        }

        num.to_string()
    }
    // [2.0655 s 2.0703 s 2.0761 s]
    // not sure how to speed this up. I avoided allocations and string conversions
    fn part_2(&self) -> String {
        let mut hasher = Md5::new();
        let mut num = 0;
        let key = self.0.as_bytes();
        loop {
            hasher.update(key);
            hasher.update(num.to_string().as_bytes());
            let result = hasher.finalize_reset();
            // an item in the result array is a byte represented by 2 hex characters: 00 to FF
            // check if both hex characters at idx 0 are 0
            // check if both hex characters at idx 1 are 0
            // check if both hex character at idx 2 are 0
            if (result[0] == 0) && (result[1] == 0) && (result[2] == 0) {
                break;
            }
            num += 1;
        }

        num.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(4);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "1048970");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(4);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "5714438");
    }
}
