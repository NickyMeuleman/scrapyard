use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(Vec<HashMap<&'a str, HashMap<&'a str, usize>>>);

fn find_parents<'a>(
    rules: &'a Vec<HashMap<&'a str, HashMap<&'a str, usize>>>,
    child_color: &str,
) -> Vec<&'a str> {
    let mut parent_set: HashSet<&str> = HashSet::new();
    for rule in rules {
        for (&color, subrules) in rule {
            if subrules.contains_key(child_color) {
                parent_set.insert(color);
            }
        }
    }
    parent_set.into_iter().collect()
}

fn get_rule<'a>(
    rules: &'a Vec<HashMap<&str, HashMap<&str, usize>>>,
    color: &str,
) -> &'a HashMap<&'a str, usize> {
    let rule = rules
        .into_iter()
        .find(|&rule| rule.contains_key(color))
        .unwrap();
    rule.get(color).unwrap()
}

fn count_bags(
    rules: &Vec<HashMap<&str, HashMap<&str, usize>>>,
    color: &str,
    mut tally: usize,
) -> usize {
    let curr = tally;
    let rule = get_rule(rules, color);
    for (&color, &amount) in rule {
        tally += amount + amount * count_bags(rules, color, curr);
    }
    tally
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        fn parse_line(line: &str) -> HashMap<&str, HashMap<&str, usize>> {
            let mut line: Vec<&str> = line.splitn(2, " contain ").collect();
            let bag = line[0].trim_end_matches(" bags");
            line[1] = line[1].trim_end_matches(".");
            let contains = parse_contains(line[1]);
            let mut rules = HashMap::new();
            rules.entry(bag).or_insert(contains);
            rules
        }

        fn parse_contains(input: &str) -> HashMap<&str, usize> {
            let parts: Vec<&str> = input
                .split(",")
                .map(|part| part.trim())
                .collect();
            let mut rules: HashMap<&str, usize> = HashMap::new();
            for part in parts {
                let part: Vec<&str> = part.splitn(2, " ").collect();
                // some lines don't have a number, but a "no" instead
                // .ok() converts Result to Option
                if let Some(num) = part[0].parse::<usize>().ok() {
                    let color = part[1]
                        .trim_end_matches(" bag")
                        .trim_end_matches(" bags");
                    rules.entry(color).or_insert(num);
                }
            }
            rules
        }
        let res = input
            .lines()
            .map(|line| parse_line(line))
            .collect();

        Ok(Self(res))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut lookup: Vec<&str> = find_parents(&self.0, "shiny gold");
        let mut visited: HashSet<&str> = HashSet::new();

        while lookup.len() > 0 {
            let needle = lookup.pop().unwrap();

            if visited.contains(needle) {
                continue;
            }

            let mut parents = find_parents(&self.0, needle);
            lookup.append(&mut parents);

            visited.insert(needle);
        }

        Ok(visited.len())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(count_bags(&self.0, "shiny gold", 0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "4");
    }

    #[test]
    fn part_2() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "126");
    }
}
