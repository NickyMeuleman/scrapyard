use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let numbers = parse(&input);
    println!("part one answer: {}", part_one(&numbers));
    // println!("part two answer: {}", part_two(&rules));
}

fn parse(input: &String) -> Vec<i128> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> i128 {
    line.parse().unwrap()
}

fn part_one(input: &Vec<i128>) -> i128 {
    find_invalid_number(input, 25)
}

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
        //  maybe still use set to avoid duplicates?
        if list.contains(&complement) && &complement != num {
            return true;
        } else {
            continue;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn solves_part_one() {
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
576"
        .to_string();
        let numbers = parse(&input);
        assert_eq!(find_invalid_number(&numbers, 5), 127);
    }
}
