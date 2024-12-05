use aoc_core::AoCError;

use crate::{AoCData, AoCResult};
use std::{cmp::Ordering, collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let (rules, pages) = self
            .0
            .split_once("\n\n")
            .ok_or(AoCError::Parsing)?;

        // let mut orderings: HashMap<u32, HashSet<u32>> = HashMap::new();
        // for rule in rules.lines() {
        //     let (n1, n2) = rule.split_once('|').unwrap();
        //     orderings
        //         .entry(n2.parse().unwrap())
        //         .or_default()
        //         .insert(n1.parse().unwrap());
        // }
        //
        // let mut updates: Vec<Vec<u32>> = vec![];
        // for page in pages.lines() {
        //     let mut update = vec![];
        //     for num in page.split(',') {
        //         update.push(num.parse().unwrap());
        //     }
        //     updates.push(update);
        // }
        //
        // let mut sum = 0;
        //
        // for update in updates {
        //     if update.is_sorted_by(|a, b| orderings[b].contains(a)) {
        //         sum += update[update.len() / 2];
        //     }
        // }
        //
        // sum

        let mut order: HashMap<(u32, u32), Ordering> = HashMap::new();
        for rule in rules.lines() {
            let (n1, n2) = rule
                .split_once('|')
                .ok_or(AoCError::Parsing)?;
            let n1 = n1.parse()?;
            let n2 = n2.parse()?;
            order.insert((n1, n2), Ordering::Less);
            order.insert((n2, n1), Ordering::Greater);
        }

        let mut sum = 0;
        for line in pages.lines() {
            let update: Vec<u32> = line
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            if update.is_sorted_by(|&a, &b| order.get(&(a, b)) == Some(&Ordering::Less)) {
                sum += update[update.len() / 2];
            }
        }
        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let (rules, pages) = self
            .0
            .split_once("\n\n")
            .ok_or(AoCError::Parsing)?;

        // let mut orderings: HashMap<u32, HashSet<u32>> = HashMap::new();
        // for rule in rules.lines() {
        //     let (n1, n2) = rule.split_once('|').unwrap();
        //     orderings
        //         .entry(n2.parse().unwrap())
        //         .or_default()
        //         .insert(n1.parse().unwrap());
        // }
        //
        // let mut updates: Vec<Vec<u32>> = vec![];
        // for page in pages.lines() {
        //     let mut update = vec![];
        //     for num in page.split(',') {
        //         update.push(num.parse().unwrap());
        //     }
        //     updates.push(update);
        // }
        //
        // let mut sum = 0;
        //
        // for mut update in updates {
        //     if !update.is_sorted_by(|a, b| orderings[b].contains(a)) {
        //         update.sort_unstable_by(|a, b| orderings[b].contains(a).cmp(&true));
        //         sum += update[update.len() / 2];
        //     }
        // }
        //
        // sum

        let mut order: HashMap<(u32, u32), Ordering> = HashMap::new();
        for rule in rules.lines() {
            let (n1, n2) = rule
                .split_once('|')
                .ok_or(AoCError::Parsing)?;
            let n1 = n1.parse()?;
            let n2 = n2.parse()?;
            order.insert((n1, n2), Ordering::Less);
            order.insert((n2, n1), Ordering::Greater);
        }

        let mut sum = 0;
        for line in pages.lines() {
            let mut update: Vec<u32> = line
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            if !update.is_sorted_by(|&a, &b| order.get(&(a, b)) == Some(&Ordering::Less)) {
                update.sort_by(|&a, &b| {
                    *order
                        .get(&(a, b))
                        .unwrap_or(&Ordering::Equal)
                });
                sum += update[update.len() / 2];
            }
        }

        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "143");
    }

    #[test]
    fn part_2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "123");
    }
}
