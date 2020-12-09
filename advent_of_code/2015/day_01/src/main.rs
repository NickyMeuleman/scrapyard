use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("part one answer: {}", part_one(&input));
    println!("part two answer: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => panic!("invalid input"),
    })
}

fn part_two(input: &str) -> i32 {
    input
        .chars()
        .enumerate()
        .try_fold(0i32, |acc, (idx, c)| {
            Ok(match c {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => panic!("invalid input"),
            })
            .and_then(|acc| {
                if acc < 0 {
                    Err(idx as i32 + 1)
                } else {
                    Ok(acc)
                }
            })
        })
        .unwrap_err()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_examples() {
        let examples = vec![
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];
        for example in examples {
            let input = example.0;
            assert_eq!(part_one(input), example.1);
        }
    }

    #[test]
    fn part_two_example_1() {
        let input = ")".to_string();
        assert_eq!(part_two(&input), 1);
    }

    #[test]
    fn part_two_example_2() {
        let input = "()())".to_string();
        assert_eq!(part_two(&input), 5);
    }
}
