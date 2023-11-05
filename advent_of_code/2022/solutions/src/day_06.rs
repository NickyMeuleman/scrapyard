use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

fn helper(s: &str, size: usize) -> usize {
    s.as_bytes()
        .windows(size)
        .position(|window| {
            window
                .iter()
                .enumerate()
                .all(|(idx, c)| !window[..idx].contains(c))
        })
        .unwrap_or(s.len())
        + size
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(helper(self.0, 4))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(helper(self.0, 14))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "11");
    }

    #[test]
    fn part_2() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "26");
    }
}
