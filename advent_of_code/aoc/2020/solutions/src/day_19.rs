use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use regex::Regex;

use crate::{AoCData, AoCResult};

type Todo = HashMap<i32, String>;
type Rules = HashMap<i32, String>;
type Messages = Vec<String>;

#[derive(Debug, Clone)]
pub struct Data(Todo, Rules, Messages);

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
            if nums
                .iter()
                .all(|n| rules.contains_key(n))
            {
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
            let idx = messages
                .iter()
                .position(|x| *x == message)
                .unwrap();
            messages.remove(idx);
        }
        if to_pop.len() == 0 {
            return count;
        }
    }
    count
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let parts: Vec<&str> = input.split("\n\n").collect();
        let mut rules: HashMap<i32, String> = HashMap::new();
        let mut todo: HashMap<i32, String> = HashMap::new();
        let messages: Vec<String> = parts[1]
            .lines()
            .map(|s| s.to_owned())
            .collect();
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

        Ok(Self(todo, rules, messages))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let regex = make_regex(self.0.clone(), self.1.clone(), false);
        let result = self
            .2
            .iter()
            .filter(|m| Regex::new(&regex).unwrap().is_match(m))
            .count();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // 8: 42 | 42 8
        // can be read as 1 or more occurrences of 42

        // 11: 42 31 | 42 11 31
        // can be read as 42 {n} 31 {n} where n is replaced with a number
        let regex = make_regex(self.0.clone(), self.1.clone(), true);

        Ok(count_valid(regex, self.2.clone()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "2");
    }

    #[test]
    fn part_2() {
        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "12");
    }
}
