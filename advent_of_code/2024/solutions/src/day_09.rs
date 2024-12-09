// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2024-day09/

use crate::{AoCData, AoCResult};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Item {
    Empty,
    File(u64),
}

#[derive(Debug, Clone)]
pub struct Data(Vec<(u64, Item)>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut filesystem = Vec::new();
        let mut file_id = 0;
        for (i, b) in input.bytes().enumerate() {
            let item = if i % 2 == 0 {
                let file = Item::File(file_id);
                file_id += 1;
                file
            } else {
                Item::Empty
            };
            filesystem.push(((b - b'0') as u64, item));
        }
        Ok(Self(filesystem))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        // expand compression, turn (5, Empty) into 5 times Empty
        let mut filesystem: Vec<Item> = self
            .0
            .iter()
            .flat_map(|&(size, item)| (0..size).map(move |_| item))
            .collect();

        let mut i = filesystem.len() - 1;
        while i > 0 {
            if filesystem[i] == Item::Empty {
                i -= 1;
                continue;
            }
            let empty_pos = filesystem[0..i]
                .iter()
                .position(|&item| item == Item::Empty);
            if let Some(j) = empty_pos {
                filesystem.swap(i, j);
            }
            i -= 1;
        }

        Ok(filesystem
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| match item {
                Item::Empty => None,
                Item::File(file_id) => Some(idx as u64 * file_id),
            })
            .sum::<u64>())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut filesystem = self.0.clone();

        let mut i = filesystem.len() - 1;
        while i > 0 {
            let (curr_size, curr_item) = filesystem[i];
            if curr_item == Item::Empty {
                i -= 1;
                continue;
            }
            let empty_pos = filesystem[0..i]
                .iter()
                .position(|&(size, item)| item == Item::Empty && size >= curr_size);
            if let Some(j) = empty_pos {
                let empty_size = filesystem[j].0;
                filesystem[j] = (curr_size, curr_item);
                filesystem[i] = (curr_size, Item::Empty);
                let remaining_empty = empty_size - curr_size;
                // Check for and insert any remaining free space
                if remaining_empty > 0 {
                    filesystem.insert(j + 1, (remaining_empty, Item::Empty));
                }
            }
            i -= 1;
        }

        Ok(filesystem
            .iter()
            // turn (5, Item) into 5 times Item
            .flat_map(|&(size, item)| (0..size).map(move |_| item))
            .enumerate()
            // filter out empty items and turn files into idx * file_id
            .filter_map(|(idx, item)| match item {
                Item::Empty => None,
                Item::File(file_id) => Some(idx as u64 * file_id),
            })
            .sum::<u64>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "2333133121414131402";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "1928");
    }

    #[test]
    fn part_2() {
        let input = "2333133121414131402";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "2858");
    }
}
