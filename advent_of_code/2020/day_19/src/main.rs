use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

type Todo = HashMap<i32, String>;
type Rules = HashMap<i32, String>;
type Messages = Vec<String>;
type Data = (Todo, Rules, Messages);

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = parse(&input);
    println!("Part one answer: {}", part_one(data.clone()));
    println!("Part two answer: {}", part_two(data));
}

fn parse(input: &str) -> (Todo, Rules, Messages) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut rules: HashMap<i32, String> = HashMap::new();
    let mut todo: HashMap<i32, String> = HashMap::new();
    let messages: Vec<String> = parts[1].lines().map(|s| s.to_owned()).collect();
    for line in parts[0].lines() {
        let parts: Vec<&str> = line.split(":").collect();
        let num = parts[0].parse().unwrap();
        let rule = parts[1].to_owned();
        if rule.contains("\"") {
            rules.insert(num, rule.trim()[1..2].to_owned());
        } else {
            todo.insert(num, rule.trim().to_owned());
        }
    }
    (todo, rules, messages)
}

fn part_one(data: Data) -> usize {
    let regex = make_regex(data.0, data.1, false);
    data.2
        .iter()
        .filter(|m| Regex::new(&regex).unwrap().is_match(m))
        .count()
}

fn part_two(data: Data) -> i32 {
    // 8: 42 | 42 8
    // can be read as 1 or more occurrences of 42

    // 11: 42 31 | 42 11 31
    // can be read as 42 {n} 31 {n} where n is replaced with a number
    let regex = make_regex(data.0, data.1, true);
    dbg!(&regex);
    count_valid(regex, data.2)
}

fn make_regex(mut todo: Todo, mut rules: Rules, p2: bool) -> String {
    while !rules.contains_key(&0) {
        let mut done: Vec<i32> = Vec::new();
        for (n, r) in &todo {
            // create iterator of every digit match for a certain todo
            let nums: HashSet<i32> = Regex::new("\\d+")
                .unwrap()
                .find_iter(r)
                .map(|s| s.as_str().parse().unwrap())
                .collect();
            // ALL nums have to be in rules
            if nums.iter().all(|n| rules.contains_key(n)) {
                let mut res = "(".to_owned();
                if *n == 11 && p2 {
                    for part in r.split_whitespace() {
                        res.push('(');
                        let num = part.parse().unwrap();
                        let rule = rules.get(&num).unwrap();
                        res.push_str(rule);
                        res.push(')');
                        // {} is a quantifier, it matches the previous token of a regex a number of times
                        // put x as the number to replace later
                        res.push_str("{x}")
                    }
                } else {
                    for part in r.split_whitespace() {
                        if part == "|" {
                            res.push_str(part);
                        } else {
                            let num = part.parse().unwrap();
                            // should always succeed since we checked earlier
                            let rule = rules.get(&num).unwrap();
                            res.push_str(rule);
                        }
                    }
                }
                res.push(')');
                if *n == 8 && p2 {
                    res.push('+');
                }
                done.push(*n);
                rules.insert(*n, res.clone());
            }
        }
        for d in &done {
            todo.remove(d);
        }
    }
    format!("^{}$", rules.get(&0).unwrap())
}

fn count_valid(regex: String, mut messages: Messages) -> i32 {
    let mut count = 0;
    // this takes veeeeeeeeeeeery long
    // the python code I tried to copy completes quite rapid, TODO: search for bottlenock
    for i in 1..100 {
        let mut to_pop = Vec::new();
        for message in messages.clone() {
            let regex = regex.replace("x", &format!("{}", i));
            let re = Regex::new(&regex).unwrap();
            if re.is_match(&message) {
                count += 1;
                to_pop.push(message.clone());
            }
        }
        for message in to_pop.clone() {
            let idx = messages.iter().position(|x| *x == message).unwrap();
            messages.remove(idx);
        }
        if to_pop.len() == 0 {
            return count;
        }
    }
    count
}
