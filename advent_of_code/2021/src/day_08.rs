use std::collections::HashMap;

use crate::AoCData;

#[derive(Debug)]
struct Pair {
    combinations: Vec<String>,
    digits: Vec<String>,
}

#[derive(Debug)]
pub struct Data {
    pairs: Vec<Pair>,
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        // TODO: figure out out to make lifetimes work so I can return Vec<&str> instead of doing all that to_owned() to get Vec<String>
        let pairs = input
            .trim()
            .lines()
            .map(|line| {
                let (combinations, digits) = line.split_once(" | ")?;
                let combinations = combinations
                    .split_whitespace()
                    .map(|s| s.to_owned())
                    .collect();
                let digits = digits.split_whitespace().map(|s| s.to_owned()).collect();

                Some(Pair {
                    combinations,
                    digits,
                })
            })
            .collect::<Option<Vec<_>>>()?;

        Some(Self { pairs })
    }

    fn part_1(&self) -> String {
        self.pairs
            .iter()
            .flat_map(|pair| {
                pair.digits
                    .iter()
                    .filter(|s| matches!(s.len(), 2 | 4 | 3 | 7))
            })
            .count()
            .to_string()
    }

    fn part_2(&self) -> String {
        // frequency of each segment in 0 to 9
        let top = 8;
        let top_left = 6;
        let top_right = 8;
        let mid = 7;
        let bottom_left = 4;
        let bottom_right = 9;
        let bottom = 7;

        // build map with sum of all segment-frequencies used in a specific digit
        // each sum is unique
        let mut score_to_digit = HashMap::new();
        score_to_digit.insert(
            top + top_left + top_right + bottom_left + bottom_right + bottom,
            0,
        );
        score_to_digit.insert(top_right + bottom_right, 1);
        score_to_digit.insert(top + top_right + mid + bottom_left + bottom, 2);
        score_to_digit.insert(top + top_right + mid + bottom_right + bottom, 3);
        score_to_digit.insert(top_left + mid + top_right + bottom_right, 4);
        score_to_digit.insert(top + top_left + mid + bottom_right + bottom, 5);
        score_to_digit.insert(
            top + top_left + mid + bottom_left + bottom_right + bottom,
            6,
        );
        score_to_digit.insert(top + top_right + bottom_right, 7);
        score_to_digit.insert(
            top + top_left + top_right + mid + bottom_left + bottom_right + bottom,
            8,
        );
        score_to_digit.insert(top + top_left + top_right + mid + bottom_right + bottom, 9);

        assert!(score_to_digit.len() == 10);

        let result: u32 = self
            .pairs
            .iter()
            .map(|pair| {
                // count frequency each segment appears in the 10 possible combinations for this pair
                let segment_frequencies = pair
                    .combinations
                    .iter()
                    .flat_map(|combination| combination.chars())
                    .fold(HashMap::new(), |mut acc, c| {
                        *acc.entry(c).or_default() += 1;
                        acc
                    });

                pair.digits
                    .iter()
                    .map(|digit| {
                        // build map with sum of all segment-frequencies for a single digit display
                        // each sum is unique and maps to a specific digit being shown
                        let digit_score = digit
                            .chars()
                            .map(|c| segment_frequencies.get(&c).unwrap())
                            .sum();

                        score_to_digit.get(&digit_score).unwrap()
                    })
                    // concatenate all shown numbers. With math instead of string concatenation because that's faster
                    .fold(0, |acc, digit| acc * 10 + digit)
            })
            .sum();

        result.to_string()
    }
}

// alternative method for part2 that figures out what each digit is with set logic
// fn part_two(&self) -> usize {
//     // # Steps for each individual line, for each pair in self.pairs:
//     // every digit is represented in the pair.combinations list.
//     // each digit is a BTreeSet
//     // The goal is to figure out which set corresponds to which displayed digit

//     // the length of the set gives a hint to which displayed digit the set represents
//     // len 7 -> display 8
//     // len 3 -> display 7
//     // len 2 -> display 1
//     // len 4 -> display 4

//     // Then, in order, that means:
//     // len 6 and superset of 4-set -> display 9
//     // len 5 and superset of 1-set -> display 3
//     // len 6 and superset of 1-set -> display 0
//     // len 6 -> display 6
//     // len 5 and is subset of 6-set -> display 5
//     // len 5 -> 2

//     // at this point you can translate from set <-> number

//     // loop over the 4 given numbers in pair.digits
//     // each number is a BTreeSet
//     // for each set, translate it to a number
//     // concatenate all these numbers (1 + 2 + 3 + 4 = 1234) to get a result for a single pair

//     // add each result for a pair to a total result
//     // that total is the solution to part2

//     let mut total = 0;

//     for pair in &self.pairs {
//         let mut todo = pair.combinations.clone();
//         let mut map: HashMap<u8, &Combination> = HashMap::new();

//         // len 2 -> display 1
//         let one = todo.iter().find(|set| set.len() == 2).unwrap().clone();
//         map.insert(1, &one);
//         todo.remove(&one);

//         // len 3 -> display 7
//         let seven = todo.iter().find(|set| set.len() == 3).unwrap().clone();
//         map.insert(7, &seven);
//         todo.remove(&seven);

//         // len 4 -> display 4
//         let four = todo.iter().find(|set| set.len() == 4).unwrap().clone();
//         map.insert(4, &four);
//         todo.remove(&four);

//         // len 7 -> display 8
//         let eight = todo.iter().find(|set| set.len() == 7).unwrap().clone();
//         map.insert(8, &eight);
//         todo.remove(&eight);

//         // len 6 and superset of 4-set -> display 9
//         let nine = todo
//             .iter()
//             .find(|set| set.len() == 6 && set.is_superset(map.get(&4).unwrap()))
//             .unwrap()
//             .clone();
//         map.insert(9, &nine);
//         todo.remove(&nine);

//         // len 5 and superset of 1-set -> display 3
//         let three = todo
//             .iter()
//             .find(|set| set.len() == 5 && set.is_superset(map.get(&1).unwrap()))
//             .unwrap()
//             .clone();
//         map.insert(3, &three);
//         todo.remove(&three);

//         // len 6 and superset of 1-set -> display 0
//         let zero = todo
//             .iter()
//             .find(|set| set.len() == 6 && set.is_superset(map.get(&1).unwrap()))
//             .unwrap()
//             .clone();
//         map.insert(0, &zero);
//         todo.remove(&zero);

//         // len 6 -> display 6
//         let six = todo.iter().find(|set| set.len() == 6).unwrap().clone();
//         map.insert(6, &six);
//         todo.remove(&six);

//         // len 5 and is subset of 6-set -> display 5
//         let five = todo
//             .iter()
//             .find(|set| set.len() == 5 && set.is_subset(map.get(&6).unwrap()))
//             .unwrap()
//             .clone();
//         map.insert(5, &five);
//         todo.remove(&five);

//         // len 5 -> 2
//         let two = todo.iter().find(|set| set.len() == 5).unwrap().clone();
//         map.insert(2, &two);
//         todo.remove(&two);

//         assert_eq!(todo.len(), 0);
//         assert_eq!(map.len(), 10);

//         let num: usize = pair
//             .digits
//             .iter()
//             .map(|set| {
//                 map.iter()
//                     .find(|(_, &map_set)| set == map_set)
//                     .unwrap()
//                     .0
//                     .to_string()
//             })
//             .collect::<String>()
//             .parse()
//             .unwrap();

//         total += num;
//     }

//     total
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(8);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "26");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(8);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "61229");
    }
}
