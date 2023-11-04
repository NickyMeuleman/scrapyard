use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let buttons: Vec<Vec<u8>> = (0..3)
            .map(|i| (i * 3 + 1..=i * 3 + 3).collect())
            .collect();
        let initial_pos = (1, 1); // middle of keypad, (mambo) number 5
        let (_pos, code) = self
            .0
            .lines()
            .fold((initial_pos, 0u32), |mut acc, line| {
                let (x, y) = line
                    .chars()
                    .fold(acc.0, |(x, y), c| match c {
                        'U' => (x, if y > 0 { y - 1 } else { y }),
                        'D' => (x, if y < 2 { y + 1 } else { y }),
                        'L' => (if x > 0 { x - 1 } else { x }, y),
                        'R' => (if x < 2 { x + 1 } else { x }, y),
                        _ => (x, y),
                    });
                let key = buttons[y][x];
                acc.1 = acc.1 * 10 + u32::from(key);
                ((x, y), acc.1)
            });

        Ok(code)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // https://github.com/LinAGKar/advent-of-code-2016-rust/blob/master/day2b/src/main.rs
        let mut buttons_iter = ('1'..='9').chain('A'..='D');
        // construct vec that holds all keys (either None, or Some(key) because of the diamond shape)
        let mut buttons = vec![vec![None; 7]; 7];
        // this double loop is really hard to understand
        for (row_idx, i) in (0..=2).chain((0..2).rev()).enumerate() {
            for j in 0..1 + 2 * i {
                buttons[row_idx + 1][3 - i + j] = buttons_iter.next();
            }
        }
        let initial_pos = (1, 3); // the 5 is on the third row, first col

        let (_pos, code) =
            self.0
                .lines()
                .fold((initial_pos, String::new()), |(pos, mut code), line| {
                    let (x, y) = line.chars().fold(pos, |(x, y), c| {
                        let (new_x, new_y) = match c {
                            'U' => (x, y - 1),
                            'D' => (x, y + 1),
                            'L' => (x - 1, y),
                            'R' => (x + 1, y),
                            _ => (x, y),
                        };

                        if buttons[new_y][new_x].is_some() {
                            (new_x, new_y)
                        } else {
                            (x, y)
                        }
                    });

                    if let Some(key) = buttons[y][x] {
                        code.push(key);
                    }

                    ((x, y), code)
                });

        Ok(code)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "ULL
RRDDD
LURDL
UUUUD";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "1985");
    }

    #[test]
    fn part_2() {
        let input = "ULL
RRDDD
LURDL
UUUUD";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "5DB3");
    }
}
