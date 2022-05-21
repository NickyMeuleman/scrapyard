use crate::AoCData;

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

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let map = input
            .trim()
            .lines()
            .map(|line| line.chars().map(|c| match c {
                'v' => Some('v'),
                '>' => Some('>'),
                '.' => Some('.'),
                _ => None
            }).collect::<Option<Vec<_>>>())
            .collect::<Option<Vec<Vec<_>>>>()?;

        Some(Self { map })
    }

    fn part_1(&self) -> String {
        let mut map = self.map.clone();
        let mut count = 0;

        loop {
            count += 1;

            let (east_map, east_moved) = run_sim(&map, true);
            let (south_map, south_moved) = run_sim(&east_map, false);

            if !east_moved && !south_moved {
                return count.to_string();
            }

            map = south_map;
        }
    }

    fn part_2(&self) -> String {
        "Merry Christmas!".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(25);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "58");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(25);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "Merry Christmas!");
    }
}
