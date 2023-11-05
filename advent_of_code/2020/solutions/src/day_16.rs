use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    fields: Vec<Field>,
    my_ticket: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>,
}

#[derive(Debug, Clone)]
struct Range {
    min: i32,
    max: i32,
}
#[derive(Debug, Clone)]
struct Field {
    name: String,
    ranges: Vec<Range>,
}

fn fits_any_range(fields: &Vec<Field>, num: i32) -> bool {
    for field in fields {
        for range in &field.ranges {
            if (range.min..=range.max).contains(&num) {
                return true;
            } else {
                continue;
            }
        }
    }
    false
}

fn is_valid(data: &Data, ticket: &Vec<i32>) -> bool {
    for num in ticket {
        if fits_any_range(&data.fields, *num) {
            continue;
        } else {
            return false;
        };
    }
    true
}

fn get_possibilities(fields: &Vec<Field>, ticket: &Vec<i32>) -> Vec<HashSet<String>> {
    // takes in ticket, returns vector of equal length with field possibilities for every index
    let mut result: Vec<HashSet<String>> = Vec::new();
    for num in ticket {
        // check which fields this num fits for
        let mut set = HashSet::new();
        for field in fields {
            for range in &field.ranges {
                // push fieldname into set
                if (range.min..=range.max).contains(num) {
                    set.insert(field.name.clone());
                }
            }
        }
        // push set into result vec
        result.push(set)
    }
    result
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        fn parse_fields(input: &str) -> Vec<Field> {
            let fields: Vec<&str> = input.split("\n").collect();
            let mut result: Vec<Field> = Vec::new();
            for field in fields {
                let parts: Vec<&str> = field.split(": ").collect();
                let name = parts[0].to_owned();
                let ranges: Vec<Range> = parts[1]
                    .split(" or ")
                    .map(|range| parse_range(range))
                    .collect();
                let field = Field { name, ranges };
                result.push(field)
            }
            result
        }

        fn parse_range(input: &str) -> Range {
            let numbers: Vec<i32> = input
                .split("-")
                .map(|num| num.parse().unwrap())
                .collect();
            Range {
                min: numbers[0],
                max: numbers[1],
            }
        }
        fn parse_ticket(input: &str) -> Vec<i32> {
            input
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect()
        }

        fn parse_nearby_tickets(input: &str) -> Vec<Vec<i32>> {
            let parts: Vec<&str> = input.splitn(2, "\n").collect();
            parts[1]
                .split("\n")
                .map(|ticket| parse_ticket(ticket))
                .collect()
        }
        let parts: Vec<&str> = input.split("\n\n").collect();
        let fields = parts[0];
        let my_ticket = parts[1];
        let nearby_tickets = parts[2];
        let fields = parse_fields(fields);
        let my_ticket: Vec<&str> = my_ticket.split("\n").collect();
        let my_ticket = parse_ticket(my_ticket[1]);
        let nearby_tickets = parse_nearby_tickets(nearby_tickets);

        Ok(Self {
            fields,
            my_ticket,
            nearby_tickets,
        })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result: i32 = self
            .nearby_tickets
            .iter()
            .flatten()
            .filter(|&num| !fits_any_range(&self.fields, *num))
            .sum();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // I got frustrated and started changing &str to String and cloning stuff until the borrow checker was happy

        let valid_tickets: Vec<&Vec<i32>> = self
            .nearby_tickets
            .iter()
            .filter(|&ticket| is_valid(self, ticket))
            .collect();

        let mut all_possibilities: Vec<Vec<HashSet<String>>> = Vec::new();
        for ticket in valid_tickets {
            all_possibilities.push(get_possibilities(&self.fields, ticket));
        }

        let mut final_possibilities: Vec<HashSet<String>> = Vec::new();
        for idx in 0..self.my_ticket.len() {
            // take first item in vec, at index as starting value for the acc
            let first_item = all_possibilities[0].clone();
            let starting_set = first_item[idx].clone();
            let possibilities_for_idx = all_possibilities
                .iter()
                .map(|ticket| ticket[idx].clone())
                .fold(starting_set, |acc, item| {
                    acc.intersection(&item)
                        .map(|s| s.clone())
                        .collect()
                });
            final_possibilities.push(possibilities_for_idx);
        }

        // when a field with a single option is found, add it to a result_map and remove it from all options
        let mut result_map: HashMap<usize, String> = HashMap::new();
        while let Some(item) = &mut final_possibilities
            .iter()
            .enumerate()
            .find(|(_, option)| option.len() == 1)
        {
            let (idx, set) = item;
            // add to result_map
            let name = set.clone().drain().next().unwrap();
            result_map.insert(*idx, name.clone());
            // remove from every set in final_possibilieties
            for set in &mut final_possibilities {
                set.remove(&name);
            }
        }
        result_map.retain(|_, name| name.contains("departure"));
        let departure_indexes: Vec<usize> = result_map.keys().map(|n| *n).collect();

        let result = departure_indexes
            .iter()
            .fold(1, |acc, i| acc * self.my_ticket[*i] as i128);

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "71");
    }
}
