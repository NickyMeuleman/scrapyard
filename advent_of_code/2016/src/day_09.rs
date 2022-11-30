use crate::AoCData;

pub struct Data(String);

fn decompress(input: &str, recurse: bool) -> Option<usize> {
    let mut size = 0;
    let mut chars = input.trim().chars();
    while let Some(c) = chars.next() {
        match c {
            '(' => {
                // by_ref borrows the iterator
                // that way we can consume parts of the iterator with take_while, not the entire iterator
                // take_while also consumes the last char it looked at! (so here: 'x' and ')')
                let len = chars
                    .by_ref()
                    .take_while(|c| *c != 'x')
                    .collect::<String>()
                    .parse()
                    .ok()?;
                let amount: usize = chars
                    .by_ref()
                    .take_while(|c| *c != ')')
                    .collect::<String>()
                    .parse()
                    .ok()?;
                if recurse {
                    let len_str: String = chars.by_ref().take(len).collect();
                    let decompressed_len = decompress(&len_str, recurse)?;
                    size += decompressed_len * amount;
                } else {
                    size += len * amount;
                    chars.advance_by(len).ok()?;
                }
            }
            _ => {
                size += 1;
            }
        }
    }
    Some(size)
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> String {
        if let Some(size) = decompress(&self.0, false) {
            size.to_string()
        } else {
            0.to_string()
        }
    }

    fn part_2(&self) -> String {
        if let Some(size) = decompress(&self.0, true) {
            size.to_string()
        } else {
            0.to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_input(9);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "123908");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(9);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "10755693147");
    }
}
