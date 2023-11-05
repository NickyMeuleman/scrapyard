use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(usize, usize);

fn transform_cycle(mut val: usize, subject_number: usize) -> usize {
    val = val * subject_number;
    val % 20201227
}

fn get_loop_size(target: usize) -> usize {
    let mut val = 1;
    let mut loop_size = 0;
    while val != target {
        val = transform_cycle(val, 7);
        loop_size += 1;
    }
    loop_size
}

fn get_encryption_key(public_key: usize, loop_size: usize) -> usize {
    let mut val = 1;
    for _ in 0..loop_size {
        val = transform_cycle(val, public_key)
    }
    val
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut iter = input.lines();
        let card_public_key = iter.next().unwrap();
        let door_public_key = iter.next().unwrap();
        Ok(Self(
            card_public_key.parse().unwrap(),
            door_public_key.parse().unwrap(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let card_loop_size = get_loop_size(self.0);
        let door_loop_size = get_loop_size(self.1);
        let card_encryption_key = get_encryption_key(self.1, card_loop_size);
        let door_encryption_key = get_encryption_key(self.0, door_loop_size);
        assert_eq!(card_encryption_key, door_encryption_key);

        Ok(card_encryption_key)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok("HoHoHo!")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "5764801
17807724";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "14897079");
    }
}
