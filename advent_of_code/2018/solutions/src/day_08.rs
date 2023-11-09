use std::{collections::VecDeque, fmt::Display};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(VecDeque<i32>);

fn sum_metadata(node: &mut VecDeque<i32>) -> i32 {
    let mut sum = 0;
    let children = node.pop_front().unwrap();
    let entries = node.pop_front().unwrap();

    for _ in 0..children {
        sum += sum_metadata(node);
    }

    for _ in 0..entries {
        sum += node.pop_front().unwrap();
    }

    sum
}

fn node_value(node: &mut VecDeque<i32>) -> i32 {
    let mut value = 0;
    let children = node.pop_front().unwrap();
    let entries = node.pop_front().unwrap();

    if children == 0 {
        // If a node has no child nodes, its value is the sum of its metadata entries.
        for _ in 0..entries {
            value += node.pop_front().unwrap();
        }
    } else {
        // if a node does have child nodes, the metadata entries become indexes which refer to those child nodes.
        let mut child_vals = Vec::new();
        for _ in 0..children {
            let val = node_value(node);
            child_vals.push(val);
        }
        for _ in 0..entries {
            let index = node.pop_front().unwrap();
            if index > 0 && index <= children {
                value += child_vals[index as usize - 1];
            }
        }
    }

    value
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut nums = self.0.clone();

        Ok(sum_metadata(&mut nums))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut nums = self.0.clone();

        Ok(node_value(&mut nums))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "138");
    }

    #[test]
    fn part_2() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "66");
    }
}
