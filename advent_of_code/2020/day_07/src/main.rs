#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let rules = parse(&input);
    println!("part one answer: {}", part_one(&rules));
}

fn parse(input: &String) -> Vec<HashMap<&str, HashMap<&str, usize>>> {
    input.lines().map(|line| parse_line(line)).collect()
}

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
    let parts: Vec<&str> = input.split(",").map(|part| part.trim()).collect();
    let mut rules: HashMap<&str, usize> = HashMap::new();
    for part in parts {
        let part: Vec<&str> = part.splitn(2, " ").collect();
        // some lines don't have a number, but a "no" instead
        // .ok() converts Result to Option
        if let Some(num) = part[0].parse::<usize>().ok() {
            let color = part[1].trim_end_matches(" bag").trim_end_matches(" bags");
            rules.entry(color).or_insert(num);
        }
    }
    rules
}

fn part_one(rules: &Vec<HashMap<&str, HashMap<&str, usize>>>) -> usize {
    let mut lookup: Vec<&str> = vec!["shiny gold"];
    let mut visited: HashSet<&str> = HashSet::new();

    while lookup.len() > 0 {
        let needle = lookup.pop().unwrap();

        if visited.contains(needle) {
            continue;
        }

        let line: &HashMap<&str, HashMap<&str, usize>> =
            rules.iter().find(|map| map.contains_key(needle)).unwrap();

        for (_, line_rules) in line {
            for (&color, _) in line_rules {
                lookup.push(color)
            }
        }

        visited.insert(needle);
    }
    // I'm finding all the bags that CANT hold a shiny gold aren't I? _deep sigh_
    // fine
    let directly_holds_count = directly_holds_shiny(rules);
    // all rules - shiny gold directly - visited - shiny gold in visited + directly holds shiny
    rules.len() - 1 - visited.len() - 1 + directly_holds_count
}

fn directly_holds_shiny(rules: &Vec<HashMap<&str, HashMap<&str, usize>>>) -> usize {
    let mut directly_holds_set = HashSet::new();
    for rule in rules {
        for (color, subrules) in rule {
            if subrules.contains_key("shiny gold") {
                dbg!(color, "directly holds");
                directly_holds_set.insert(color);
            }
        }
    }
    directly_holds_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_one() {
        let input: String = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            .to_owned();
        let rules = parse(&input);
        assert_eq!(part_one(&rules), 4);
    }
}
