fn main() {
    let input = "418976235";
    let data = parse(&input);
    println!("Part one answer: {}", part_one(data.clone()));
    println!("Part two answer: {}", part_two(data));
}

fn parse(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn part_one(data: Vec<u32>) -> i32 {
    let first_label = data[0] as i32;
    let mut cups = get_cups(data, 9);
    // simulate 100 moves. What are the labels on the cups after cup 1?
    play_game(&mut cups, first_label, 100);
    let mut current_index = cups[0];
    // some math where the acc is multiplied by 10 to add a 0 at the end
    // then adding the current number (since it's from 1 to 9)
    // remember: every number is one less, so that's the +1 in the calculation
    (0..8).fold(0, |acc, _| {
        let new = (acc * 10) + (current_index + 1);
        current_index = cups[current_index as usize];
        new
    })
}

fn part_two(data: Vec<u32>) -> usize {
    let first_label = data[0] as i32;
    let mut cups = get_cups(data, 1_000_000);
    play_game(&mut cups, first_label, 10_000_000);
    let first_after_1: usize = cups[0] as usize;
    let second_after_1: usize = cups[first_after_1] as usize;
    (first_after_1 + 1) * (second_after_1 + 1)
}

fn play_game(cups: &mut Vec<i32>, first_label: i32, number_rounds: usize) {
    let number_cups = cups.len() as i32;
    // initialize to first number in input
    // input starts with 4, but every number is one lower, so start with 3
    let mut current_cup = first_label - 1;
    for _ in 0..number_rounds {
        // Next 3 cups
        let next_1 = cups[current_cup as usize];
        let next_2 = cups[next_1 as usize];
        let next_3 = cups[next_2 as usize];

        // Get target
        // using rem euclid since current_cup - 1 can be -1, the % operator is not the same for negative numbers!
        let mut target_cup = (current_cup - 1).rem_euclid(number_cups);
        while [next_1, next_2, next_3].contains(&target_cup) {
            target_cup = (target_cup - 1).rem_euclid(number_cups);
        }

        // Update indices
        cups.swap(current_cup as usize, next_3 as usize);
        cups.swap(next_3 as usize, target_cup as usize);

        // Increment current
        current_cup = cups[current_cup as usize];
    }
}

fn get_cups(input: Vec<u32>, amount: usize) -> Vec<i32> {
    // create vector where the number at an index is the next number

    // subtracting one from each input
    // this avoids the problem where index 0 would remain untouched.
    // every index and every number will then be 1 less
    let input: Vec<i32> = input.iter().map(|&n| n as i32 - 1).collect();
    let mut cups = vec![0; input.len()];
    for idx in 0..input.len() {
        let current_num = input[idx];
        // % input.len() to loop around at the end (instead of index 9, it becomes index 0)
        let next_num = input[(idx + 1) % input.len()];
        cups[current_num as usize] = next_num;
    }
    if input.len() == amount {
        return cups;
    }
    // From the question:
    // Your labeling is still correct for the first few cups;
    // after that, the remaining cups are just numbered in an increasing fashion
    // starting from the number after the highest number in your list and proceeding one by one until one million is reached.
    cups.resize(amount, 0);
    // remember: the numbers in the vec are one less at every point, so here that means starting from 9 until 999_999 is reached.
    // rewrite last item, instead of pointing to the first item, it should now point to the next in the bigger vector
    let last_num_in_input = input[input.len() - 1] as usize;
    cups[last_num_in_input] = input.len() as i32;
    // extend cups vector according the rules above
    for idx in input.len()..amount {
        cups[idx] = idx as i32 + 1;
    }
    // make last item in extended list point at first number in input
    cups[amount - 1] = input[0];
    cups
}

#[cfg(test)]
mod tests {
    use super::*;
    // The part1 test doesn't pass as you have to adjust the magic number for the amount of rounds first.
    // 100 for part1 in the actual question.
    // 10 for part1 in the test.
    #[test]
    fn solves_part_one() {
        let input = "389125467";
        let data = parse(&input);
        assert_eq!(part_one(data), 92658374)
    }

    #[test]
    fn solves_part_two() {
        let input = "389125467";
        let data = parse(&input);
        assert_eq!(part_two(data), 149245887792)
    }
}
