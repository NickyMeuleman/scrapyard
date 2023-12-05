use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

use aoc_core::AoCError;
use itertools::Itertools;

struct Rule {
    destination: i64,
    source: i64,
    range: i64,
}

#[derive(Debug, Clone)]
struct Range {
    from: i64,
    to: i64,
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let (seeds_str, maps_str) = self
            .0
            .split_once("\n\n")
            .ok_or(AoCError::Parsing)?;
        let seeds = seeds_str
            .strip_prefix("seeds: ")
            .ok_or(AoCError::Parsing)?;
        let seeds = seeds
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok());

        let maps = maps_str
            .split("\n\n")
            .map(|block| {
                block
                    .lines()
                    .skip(1)
                    .map(|line| {
                        let mut nums = line.splitn(3, " ");
                        Ok(Rule {
                            destination: nums
                                .next()
                                .ok_or(AoCError::Parsing)?
                                .parse()
                                .map_err(|_| AoCError::Parsing)?,
                            source: nums
                                .next()
                                .ok_or(AoCError::Parsing)?
                                .parse()
                                .map_err(|_| AoCError::Parsing)?,
                            range: nums
                                .next()
                                .ok_or(AoCError::Parsing)?
                                .parse()
                                .map_err(|_| AoCError::Parsing)?,
                        })
                    })
                    .collect()
            })
            .collect::<AoCResult<Vec<Vec<Rule>>>>()?;

        seeds
            // map each seed to a location
            .map(|seed| {
                maps.iter().fold(seed, |curr, rules| {
                    if let Some(rule) = rules
                        .iter()
                        .find(|rule| curr >= rule.source && curr <= rule.source + rule.range)
                    {
                        let offset = curr - rule.source;
                        rule.destination + offset
                    } else {
                        curr
                    }
                })
            })
            .min()
            .ok_or(AoCError::Solving)
    }

    // alternative solution
    // pub fn part_1(input: &str) -> i64 {
    //     let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();
    //     let seeds = seeds_str.strip_prefix("seeds: ").unwrap();
    //     let seeds = seeds.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    //     let mut maps = Vec::new();
    //     for block in maps_str.split("\n\n") {
    //         // ignore header of map
    //         let (_, rules) = block.split_once("\n").unwrap();
    //         let mut map = Vec::new();
    //         for line in rules.lines() {
    //             let mut nums = line.splitn(3, " ");
    //             let destination: i64 = nums.next().unwrap().parse().unwrap();
    //             let source: i64 = nums.next().unwrap().parse().unwrap();
    //             let range: i64 = nums.next().unwrap().parse().unwrap();
    //             map.push(Rule {
    //                 destination,
    //                 source,
    //                 range,
    //             });
    //         }
    //         maps.push(map);
    //     }

    fn part_2(&self) -> AoCResult<impl Display> {
        let (seeds_str, maps_str) = self
            .0
            .split_once("\n\n")
            .ok_or(AoCError::Parsing)?;
        let seeds = seeds_str
            .strip_prefix("seeds: ")
            .ok_or(AoCError::Parsing)?;
        let seeds = seeds
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .chunks(2);
        let seeds = seeds
            .into_iter()
            .filter_map(|mut chunk| {
                let from = chunk.next()?;
                let range = chunk.next()?;
                Some(Range {
                    from,
                    to: from + range,
                })
            });

        let maps = maps_str
            .split("\n\n")
            .map(|block| {
                let mut map = block
                    .lines()
                    .skip(1)
                    .map(|line| {
                        let mut nums = line.splitn(3, " ");
                        Ok(Rule {
                            destination: nums
                                .next()
                                .ok_or(AoCError::Parsing)?
                                .parse()
                                .map_err(|_| AoCError::Parsing)?,
                            source: nums
                                .next()
                                .ok_or(AoCError::Parsing)?
                                .parse()
                                .map_err(|_| AoCError::Parsing)?,
                            range: nums
                                .next()
                                .ok_or(AoCError::Parsing)?
                                .parse()
                                .map_err(|_| AoCError::Parsing)?,
                        })
                    })
                    .collect::<AoCResult<Vec<Rule>>>()?;

                map.sort_by(|a, b| a.source.cmp(&b.source));
                Ok(map)
            })
            .collect::<AoCResult<Vec<Vec<Rule>>>>()?;

        let mut ranges: Vec<Range> = seeds.clone().collect();
        for map in &maps {
            let mut new_ranges: Vec<Range> = vec![];

            for range in ranges {
                let mut curr = range.clone();
                for rule in map {
                    let offset = rule.destination - rule.source;
                    if curr.from <= curr.to
                        && curr.from < rule.source + rule.range
                        && rule.source <= curr.to
                    {
                        if curr.from < rule.source {
                            new_ranges.push(Range {
                                from: curr.from,
                                to: rule.source - 1,
                            });
                            curr.from = rule.source;
                            if curr.to < rule.source + rule.range {
                                new_ranges.push(Range {
                                    from: curr.from + offset,
                                    to: curr.to + offset,
                                });
                                curr.from = curr.to + 1;
                            } else {
                                new_ranges.push(Range {
                                    from: curr.from + offset,
                                    to: rule.source + rule.range - 1 + offset,
                                });
                                curr.from = rule.source + rule.range;
                            }
                        } else if curr.to < rule.source + rule.range {
                            new_ranges.push(Range {
                                from: curr.from + offset,
                                to: curr.to + offset,
                            });
                            curr.from = curr.to + 1;
                        } else {
                            new_ranges.push(Range {
                                from: curr.from + offset,
                                to: rule.source + rule.range - 1 + offset,
                            });
                            curr.from = rule.source + rule.range;
                        }
                    }
                }
                if curr.from <= curr.to {
                    new_ranges.push(curr);
                }
            }
            ranges = new_ranges;
        }

        ranges
            .iter()
            .map(|range| range.from)
            .min()
            .ok_or(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "35");
    }

    #[test]
    fn part_2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "46");
    }
}
