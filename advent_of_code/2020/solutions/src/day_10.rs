use std::{collections::HashMap, fmt::Display};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<u32>);

fn num_of_combinations(adapters: &Vec<u32>, given: u32, memo: &mut HashMap<u32, u128>) -> u128 {
    // return the amount of combinations possible if given was the starting(lowest) number
    let max = adapters.iter().max().unwrap();
    if given == *max {
        // there is only 1 possible ordering if the lowest is also the highest number.
        return 1;
    }
    if memo.contains_key(&given) {
        return *memo.get(&given).unwrap();
    }
    // find every adapter in list that possibly fits the given
    let possible_adapters: Vec<&u32> = adapters
        .into_iter()
        .filter(|&num| {
            let mut keep = false;
            for offset in 1..=3 {
                if *num == given + offset {
                    keep = true
                }
            }
            keep
        })
        .collect();
    // for every possibility, recurse with that possibility as the given parameter and add the result to a vec
    let mut results = Vec::new();
    for &possibility in possible_adapters {
        let result = num_of_combinations(adapters, possibility, memo);
        memo.insert(possibility, result);
        results.push(result);
    }
    // sum the vec of results to return all possible combinations for the function parameters
    results.iter().sum()
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        fn parse_line(line: &str) -> u32 {
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
        let mut adapters = self.0.clone();
        // q: What is the number of 1-jolt differences multiplied by the number of 3-jolt differences?
        // use all adapters
        // each adapter supports a joltage output that is 3 larger
        // are there duplicates in the list?
        adapters.sort();
        // start at wall with a rating of 0
        let mut prev_rating = 0;
        let mut differences: Vec<u32> = Vec::new();
        // repeat for every adapter in list
        while adapters.len() > 0 {
            // remove first item in sorted list (minimum)
            let smallest = adapters.remove(0);
            // note difference in JOLTAGE
            let difference = smallest - prev_rating;
            prev_rating = smallest;
            differences.push(difference);
        }
        let one_difference = differences
            .iter()
            .filter(|&difference| *difference == 1)
            .count();
        let three_difference = differences
            .iter()
            .filter(|&difference| *difference == 3)
            .count();

        // device input is 3 jolts larger than largest adapter
        Ok(one_difference as u32 * (three_difference as u32 + 1))

        // fn part_one_2(adapters: &mut Vec<u32>) -> u32 {
        //     adapters.sort();
        //     // create iterator with 0 at the start
        //     let mut start_vec = adapters.clone();
        //     start_vec.insert(0, 0);
        //     let first_iter = start_vec.iter();

        //     // create iterator with the device rating at the end
        //     let mut end_vec = adapters.clone();
        //     let device_rating = adapters[adapters.len() - 1] + 3;
        //     end_vec.push(device_rating);
        //     let second_iter = end_vec.iter();

        //     // iterate through list that has 2 subsequent adapters
        //     let tuple_iter = first_iter.zip(second_iter);
        //     let (ones, threes) = tuple_iter.fold((0, 0), |(ones, threes), (first, second)| {
        //         match second - first {
        //             1 => (ones + 1, threes),
        //             3 => (ones, threes + 1),
        //             _ => (ones, threes),
        //         }
        //     });
        //     ones * threes
        // }
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut memo: HashMap<u32, u128> = HashMap::new();
        Ok(num_of_combinations(&self.0, 0, &mut memo))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "220");
    }

    #[test]
    fn part_2() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "19208");
    }
}
