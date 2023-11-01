use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<i128>);

fn find_invalid_number(input: &Vec<i128>, preamble_length: usize) -> i128 {
    // skip preamble
    let remaining_list = &input[preamble_length..];
    for (idx, num) in remaining_list.iter().enumerate() {
        // take previous numbers
        let list = &input[0 + idx..preamble_length + idx];
        // check if sum possible with those
        if has_sum(&num, list.to_vec()) {
            continue;
        } else {
            // return first num where it isn't
            return *num;
        }
    }
    0
}

fn has_sum(target: &i128, list: Vec<i128>) -> bool {
    for num in &list {
        let complement = target - num;
        if list.contains(&complement) && &complement != num {
            return true;
        } else {
            continue;
        }
    }
    false
}

fn find_encryption_weakness(input: &Vec<i128>, preamble_length: usize) -> i128 {
    let invalid_number = find_invalid_number(input, preamble_length);
    let sequence = find_sum_sequence(input, invalid_number);
    let min = sequence.iter().min().unwrap();
    let max = sequence.iter().max().unwrap();
    min + max
}

fn find_sum_sequence(list: &Vec<i128>, target_sum: i128) -> Vec<i128> {
    // initialize indexes to a contiguous sublist of the input at 0 and 1 (list length at least 2)
    // sum the resulting list
    // if the sum is correct, return that list
    // if that sum is too small, increment the ending index
    // if that sum is too big, increment the starting index
    let mut start_idx = 0;
    let mut stop_idx = 1;
    loop {
        let sum: i128 = list[start_idx..=stop_idx].iter().sum();
        if sum == target_sum {
            break list[start_idx..=stop_idx].to_vec();
        } else if sum > target_sum {
            start_idx += 1;
            continue;
        } else {
            stop_idx += 1;
            continue;
        }
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        fn parse_line(line: &str) -> i128 {
            line.parse().unwrap()
        }
        Ok(Self(
            input
                .lines()
                .map(|line| parse_line(line))
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(find_invalid_number(&self.0, 25))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(find_encryption_weakness(&self.0, 25))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let data = Data::try_new(input).unwrap();
        assert_eq!(find_invalid_number(&data.0, 5), 127);
    }

    #[test]
    fn part_2() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let data = Data::try_new(input).unwrap();
        assert_eq!(find_encryption_weakness(&data.0, 5), 62);
    }
}
