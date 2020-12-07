#[allow(unused_imports)]
use std::collections::HashMap;
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
    let bag = line[0].trim_end_matches("bags");
    line[1] = line[1].trim_end_matches(".");
    let contains = parse_contains(line[1]);
    let mut rules = HashMap::new();
    rules.entry(bag).or_insert(contains);
    rules
}

fn parse_contains(input: &str) -> HashMap<&str, usize> {
    // Vec<HashMap<&str, usize>>
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
    1
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
        assert_eq!(part_one(rules), 4);
    }
}
