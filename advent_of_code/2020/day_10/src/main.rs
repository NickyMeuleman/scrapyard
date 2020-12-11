use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let adapters: Vec<u32> = parse(&input);
    println!("Part one answer: {}", part_one(&mut adapters.clone()));
    println!("Part two answer: {}", part_two(&adapters.clone()));
}

fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> u32 {
    line.parse().unwrap()
}

fn part_one(adapters: &mut Vec<u32>) -> u32 {
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
    one_difference as u32 * (three_difference as u32 + 1)
}

fn part_one_2(adapters: &mut Vec<u32>) -> u32 {
    adapters.sort();
    // create iterator with 0 at the start
    let mut start_vec = adapters.clone();
    start_vec.insert(0, 0);
    let first_iter = start_vec.iter();

    // create iterator with the device rating at the end
    let mut end_vec = adapters.clone();
    let device_rating = adapters[adapters.len() - 1] + 3;
    end_vec.push(device_rating);
    let second_iter = end_vec.iter();

    // iterate through list that has 2 subsequent adapters
    let tuple_iter = first_iter.zip(second_iter);
    let (ones, threes) = tuple_iter.fold((0, 0), |(ones, threes), (first, second)| {
        match second - first {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        }
    });
    ones * threes
}

fn part_two(adapters: &Vec<u32>) -> u128 {
    let mut memo: HashMap<u32, u128> = HashMap::new();
    num_of_combinations(adapters, 0, &mut memo)
}

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
