use std::collections::HashMap;

fn main() {
    let input = "1,0,16,5,17,4".to_owned();
    let start_nums = parse(&input);
    println!("Part one answer: {}", part_one(&start_nums));
    println!("Part two answer: {}", part_two(&start_nums));
}

fn parse(input: &str) -> Vec<u64> {
    input.split(",").map(|s| s.parse().unwrap()).collect()
}

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

fn part_one(start_nums: &Vec<u64>) -> u64 {
    get_num(start_nums, 2020)
}

fn part_two(start_nums: &Vec<u64>) -> u64 {
    get_num(start_nums, 30_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_one() {
        let examples: [(String, u64); 6] = [
            ("1,3,2".to_string(), 1),
            ("2,1,3".to_string(), 10),
            ("1,2,3".to_string(), 27),
            ("2,3,1".to_string(), 78),
            ("3,2,1".to_string(), 438),
            ("3,1,2".to_string(), 1836),
        ];
        for (input, answer) in examples.iter() {
            let starting_nums = parse(&input);
            assert_eq!(part_one(&starting_nums), *answer);
        }
    }

    #[test]
    fn solves_part_two() {
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
            let starting_nums = parse(&input);
            assert_eq!(part_two(&starting_nums), *answer);
        }
    }
}
