use crate::{AoCData, AoCResult};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data(Vec<(u64, Vec<u64>)>);

fn is_reachable(goal: u64, nums: &[u64], concat: bool) -> bool {
    if nums.len() == 1 {
        return goal == nums[0];
    }
    let (&last, rest) = nums.split_last().unwrap();
    if goal % last == 0 && is_reachable(goal / last, rest, concat) {
        return true;
    }
    if goal > last && is_reachable(goal - last, rest, concat) {
        return true;
    }
    if concat {
        let last_len = last.ilog10() + 1;
        let magnitude = 10u64.pow(last_len);
        let goal_len = goal.ilog10() + 1;
        let ending = goal % magnitude;
        if goal_len > last_len && last == ending && is_reachable(goal / magnitude, rest, concat) {
            return true;
        }
    }
    false
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .lines()
                .map(|line| {
                    let (goal, nums) = line.split_once(':').unwrap();
                    (
                        goal.parse().unwrap(),
                        nums.split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect(),
                    )
                })
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(self
            .0
            .iter()
            .filter_map(|(goal, nums)| is_reachable(*goal, nums, false).then_some(goal))
            .sum::<u64>())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(self
            .0
            .iter()
            .filter_map(|(goal, nums)| is_reachable(*goal, nums, true).then_some(goal))
            .sum::<u64>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "3749");
    }

    #[test]
    fn part_2() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "11387");
    }
}
