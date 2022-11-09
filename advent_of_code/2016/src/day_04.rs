use std::collections::HashMap;

use itertools::Itertools;

use crate::AoCData;

pub struct Data(Vec<Room>);

#[derive(Debug)]
struct Room {
    name: String,
    sector: u32,
    checksum: String,
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut rooms = Vec::new();
        for line in input.lines() {
            let (left, right) = line.split_once('[')?;
            let checksum = right.strip_suffix(']')?;
            let (name, sector) = left.rsplit_once('-')?;
            let sector: u32 = sector.parse().ok()?;
            rooms.push(Room {
                checksum: checksum.to_string(),
                name: name.to_string(),
                sector,
            });
        }
        Some(Self(rooms))
    }

    fn part_1(&self) -> String {
        let mut sum = 0;
        for room in &self.0 {
            let mut frequency: HashMap<char, u32> = HashMap::new();
            for c in room.name.chars().filter(|c| c.is_alphabetic()) {
                *frequency.entry(c).or_default() += 1;
            }
            let checksum: String = frequency
                .iter()
                .map(|(k, v)| (std::cmp::Reverse(v), k))
                .sorted_unstable()
                .take(5)
                // .inspect(|item| {
                //     dbg!(&item);
                //     println!()
                // })
                .map(|(_k, v)| v)
                .collect();
            if checksum == room.checksum {
                sum += room.sector;
            }
        }

        sum.to_string()
    }

    fn part_2(&self) -> String {
        for room in &self.0 {
            let mut frequency: HashMap<char, u32> = HashMap::new();
            for c in room.name.chars().filter(|c| c.is_alphabetic()) {
                *frequency.entry(c).or_default() += 1;
            }
            let checksum: String = frequency
                .iter()
                .map(|(k, v)| (std::cmp::Reverse(v), k))
                .sorted_unstable()
                .take(5)
                .map(|(_k, v)| v)
                .collect();
            if checksum == room.checksum {
                let decrypted: String = room
                    .name
                    .chars()
                    .map(|c| match c {
                        '-' => ' ',
                        _ => {
                            (((c as u32 - 'a' as u32 + room.sector) % 26_u32) as u8 + b'a') as char
                        }
                    })
                    .collect();
                if decrypted == "northpole object storage" {
                    return room.sector.to_string();
                }
            }
        }

        String::from("No northpole object storage found")
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
        assert_eq!(data.part_1(), "1514");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(4);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "267");
    }
}
