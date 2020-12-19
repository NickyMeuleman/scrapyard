use std::fs;

type Data = Vec<String>;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = parse(&input);
    println!("Part one answer: {}", part_one(&data));
    // println!("Part two answer: {}", part_two(&data));
}

fn part_one(data: &Data) -> i128 {
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

fn calculate(input: String) -> i128 {
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
    let mut start_idx: Option<usize> = None;
    let mut end_idx: Option<usize> = None;
    for (idx, c) in input.chars().enumerate() {
        if c == '(' {
            start_idx = Some(idx);
        }
        // take index of first found closing paren and return
        if c == ')' {
            end_idx = Some(idx);
            // if we get here, the indexes will both be Some, unwrapping won't panic here, fingers crossed
            let deepest_block = &input
                [start_idx.expect("no starting index") + 1..end_idx.expect("no ending index")]
                .to_owned();
            let indexes = (start_idx.unwrap(), end_idx.unwrap());
            return (Some(indexes), deepest_block.clone());
        }
    }
    (None, input.clone())
}

fn evaluate_without_parenthesis(input: String) -> i128 {
    // I went recursion happy this day, the size of my stack is gonna be huge I tell you, huuuuge.
    // if the input to this function is a number, no further calculation is needed, return the number
    if let Ok(num) = input.parse() {
        return num;
    }

    // keep track of index of the operator, and which one was found
    let mut operator: (Option<usize>, char) = (None, '+');
    // stop_idx is inclusive!
    let mut stop_idx: Option<usize> = None;
    for (idx, c) in input.chars().enumerate() {
        // get first operator
        if operator.0 == None && (c == '+' || c == '*') {
            operator = (Some(idx), c);
        } else if operator.0 != None && (c == '+' || c == '*') {
            // get potential second operator
            stop_idx = Some(idx - 1);
            break;
        } else if idx == input.len() - 1 {
            // if no second operator was found, everything to the right of the first operator is the rhs
            stop_idx = Some(input.len() - 1);
        }
    }
    // everything to the left of that index is lhs
    let lhs: i128 = input[..operator.0.unwrap()]
        .parse()
        .expect("lhs was not a number");
    // everything to the right until you hit either the ending or the next operator is rhs
    let rhs: i128 = input[(operator.0.unwrap() + 1)..=stop_idx.unwrap()]
        .parse()
        .expect("rhs was not a number");
    let result = match operator.1 {
        '+' => lhs + rhs,
        '*' => lhs * rhs,
        _ => panic!("Unknown operator found"),
    };
    // create new input with the resulting number replacing the first operation
    // TODO: get start_idx
    let indexes = Some((0, stop_idx.unwrap()));
    let new_input = replace_part(input, indexes, result);
    // recursively call calculate with a single operation evaluated
    evaluate_without_parenthesis(new_input)
}

fn replace_part(input: String, indexes: Option<(usize, usize)>, num: i128) -> String {
    if let Some(tup) = indexes {
        let mut result = input.clone();
        result.replace_range(tup.0..=tup.1, &num.to_string());
        result
    } else {
        num.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_one() {
        let input = "2 * 3 + (4 * 5)".to_owned();
        let data = parse(&input);

        assert_eq!(part_one(&data), 437);
    }
}
