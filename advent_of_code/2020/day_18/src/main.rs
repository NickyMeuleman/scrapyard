use std::fs;

type Data = Vec<String>;

fn main() {
    // solve from left to right
    // calculate function:
    // parse left-hand-side operator right-hand-side
    // if a parenthesis is found, recursively call calculate
    // if the lhs and rhs have no parentheses, return the result
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = parse(&input);
    println!("Part one answer: {}", part_one(&data));
    println!("Part two answer: {}", part_two(&data));
}

fn part_one(data: &Data) -> i32 {
    data.iter().map(|line| calculate(line.clone())).sum()
}

fn part_two(data: &Data) -> i32 {
    1
}

fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|s| {
            let mut st = s.to_owned();
            st.retain(|c| !c.is_whitespace());
            st
        })
        .collect()
}

fn calculate(input: String) -> i32 {
    // if the input to this function is a number, no further calculation is needed, return the number
    if let Ok(num) = input.parse() {
        return num;
    }
    // get indexes of deepest set of parentheses:
    // 1. a set of parentheses were found in the input, in that case the value of indexes is Some((usize, usize))
    // and block is the block those parentheses enclose
    // 2. a set of parentheses were not found in the input, in that case the value of indexes is None
    // and block is the same thing you passed into this function
    let (indexes, block) = get_first_deepest_block(&input);
    // turn that block without parentheses into a number
    let num = evaluate_without_parenthesis(block);
    // create new string with that number instead of the parentheses block:
    // 1. indexes is None, the num is returned (as a String type)
    // 2. indexes is Some, the num replaces whatever is in the input at those indexes
    let new_input = replace_part(input, indexes, num);
    // recursively call calculate with a single block evaluated
    calculate(new_input)
}

fn get_first_deepest_block(input: &String) -> (Option<(usize, usize)>, String) {
    // if a block is passed without parenthesis, return None for indexes of the matching pair, and the input block
    // if a block is passed with parentheses, return a Some with the indexes of the matching pair, and the contained block
    
    // testing code to see if things blow up
    let deepest_block = "1+2".to_owned();
    let indexes = (0, deepest_block.len() - 1);
    (Some(indexes), deepest_block)
}

fn evaluate_without_parenthesis(input: String) -> i32 {
    1
}

fn replace_part(input: String, indexes: Option<(usize, usize)>, num: i32) -> String {
    if let Some(tup) = indexes {
        let mut result = input.clone();
        result.replace_range(tup.0..=tup.1, &num.to_string());
        result
    } else {
        num.to_string()
    }
}
