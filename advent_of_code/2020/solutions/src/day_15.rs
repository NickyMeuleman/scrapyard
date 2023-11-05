use std::{collections::HashMap, fmt::Display};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<u64>);

fn get_num(start_nums: &Vec<u64>, iterations: u64) -> u64 {
    // create HashMap with keys for the played number and value for the turn it was last played at
    let mut map: HashMap<u64, u64> = HashMap::new();
    let mut curr_idx = 0;
    // initialise prev_num to a random num as it will be overwritten by the starting nums
    let mut prev_num = 0;

    // loop through starting nums before beginning
    for &num in start_nums {
        map.insert(num, curr_idx);
        curr_idx += 1;
        prev_num = num;
    }

    while curr_idx < iterations {
        // every turn, check if the previously said number was played before
        let mut curr_num = 0;
        if let Some(&prev_idx) = &map.get(&prev_num) {
            // if played before, the number becomes how far apart the previous number was said
            curr_num = curr_idx - 1 - prev_idx;
        }
        // insert the number for the last iteration into the map
        map.insert(prev_num, curr_idx - 1);
        // prep for next iteration
        prev_num = curr_num;
        curr_idx += 1;
    }
    prev_num
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(get_num(&self.0, 2020))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(get_num(&self.0, 30_000_000))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let examples: [(String, u64); 6] = [
            ("1,3,2".to_string(), 1),
            ("2,1,3".to_string(), 10),
            ("1,2,3".to_string(), 27),
            ("2,3,1".to_string(), 78),
            ("3,2,1".to_string(), 438),
            ("3,1,2".to_string(), 1836),
        ];
        for (input, answer) in examples.iter() {
            let data = Data::try_new(input).unwrap();
            let result = data.part_1().unwrap().to_string();
            assert_eq!(result, answer.to_string());
        }
    }

    #[test]
    fn part_2() {
        // since the same brute force solution is used, these take a few minutes to complete
        let examples: [(String, u64); 7] = [
            ("0,3,6".to_string(), 175594),
            ("1,3,2".to_string(), 2578),
            ("2,1,3".to_string(), 3544142),
            ("1,2,3".to_string(), 261214),
            ("2,3,1".to_string(), 6895259),
            ("3,2,1".to_string(), 18),
            ("3,1,2".to_string(), 362),
        ];
        for (input, answer) in examples.iter() {
            let data = Data::try_new(input).unwrap();
            let result = data.part_2().unwrap().to_string();
            assert_eq!(result, answer.to_string());
        }
    }
}
