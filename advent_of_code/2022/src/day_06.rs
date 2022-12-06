use crate::AoCData;

pub struct Data(String);

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

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> String {
        helper(&self.0, 4).to_string()
    }

    fn part_2(&self) -> String {
        helper(&self.0, 14).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(6);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "11");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(6);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "26");
    }
}
