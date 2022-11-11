use crate::AoCData;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Data(Vec<Ip>);

#[derive(Debug)]
struct Ip {
    plain: Vec<String>,
    hypernet: Vec<String>,
}

fn has_abba(s: &str) -> bool {
    s.chars()
        .tuple_windows()
        .any(|(one, two, three, four)| one == four && two == three && one != two)
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut ips = Vec::new();
        for line in input.lines() {
            let mut plains = Vec::new();
            let mut hypernets = Vec::new();
            let mut unparsed = line;
            while !unparsed.is_empty() {
                let (left, rest) = unparsed.split_once('[')?;
                let (hypernet, rest) = rest.split_once(']')?;
                plains.push(left.to_string());
                hypernets.push(hypernet.to_string());
                if !rest.contains(']') {
                    plains.push(rest.to_string());
                    break;
                }
                unparsed = rest;
            }

            ips.push(Ip {
                plain: plains,
                hypernet: hypernets,
            });
        }
        Some(Self(ips))
    }

    fn part_1(&self) -> String {
        let mut count = 0;
        'outer: for ip in self.0.iter() {
            for hypernet in &ip.hypernet {
                if has_abba(hypernet) {
                    continue 'outer;
                }
            }
            for plain in &ip.plain {
                if has_abba(plain) {
                    count += 1;
                    continue 'outer;
                }
            }
        }
        count.to_string()
    }

    fn part_2(&self) -> String {
        let mut count = 0;
        'outer: for ip in &self.0 {
            let mut ab = HashSet::new();
            for supernet in &ip.plain {
                for (a, b, c) in supernet.chars().tuple_windows() {
                    if a == c && a != b {
                        ab.insert((a, b));
                    }
                }
            }
            for hypernet in &ip.hypernet {
                for combo in &ab {
                    for (a, b, c) in hypernet.chars().tuple_windows() {
                        if a == combo.1 && b == combo.0 && c == combo.1 {
                            count += 1;
                            continue 'outer;
                        }
                    }
                }
            }
        }
        count.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(7);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "2");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(7);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "260");
    }
}
