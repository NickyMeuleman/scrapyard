use std::fmt::Display;

use md5::{Digest, Md5};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
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

        Ok(num)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
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

        Ok(num)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "pqrstuv";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "1048970");
    }

    #[test]
    fn part_2() {
        let input = "pqrstuv";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "5714438");
    }
}
