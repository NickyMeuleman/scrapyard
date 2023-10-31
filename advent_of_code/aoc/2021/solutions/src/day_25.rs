use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    map: Vec<Vec<char>>,
}

fn run_sim(map: &[Vec<char>], is_east: bool) -> (Vec<Vec<char>>, bool) {
    let height = map.len();
    let width = map[0].len();
    let active = if is_east { '>' } else { 'v' };

    let mut res = map.to_vec();
    let mut moved = false;

    for (i, row) in map.iter().enumerate() {
        for (j, &elem) in row.iter().enumerate() {
            if elem != active {
                continue;
            }

            let next_row = if is_east { i } else { (i + 1) % height };
            let next_col = if is_east { (j + 1) % width } else { j };

            if map[next_row][next_col] != '.' {
                continue;
            }

            res[i][j] = '.';
            res[next_row][next_col] = active;
            moved = true;
        }
    }

    (res.to_vec(), moved)
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let map = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'v' => Some('v'),
                        '>' => Some('>'),
                        '.' => Some('.'),
                        _ => None,
                    })
                    .collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<Vec<_>>>>()
            .ok_or(AoCError::Parsing)?;

        Ok(Self { map })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut map = self.map.clone();
        let mut count = 0;

        loop {
            count += 1;

            let (east_map, east_moved) = run_sim(&map, true);
            let (south_map, south_moved) = run_sim(&east_map, false);

            if !east_moved && !south_moved {
                return Ok(count);
            }

            map = south_map;
        }
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok("Merry Christmas!")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "58");
    }
}
