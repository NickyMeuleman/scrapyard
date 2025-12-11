// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2025-day11/

use crate::{AoCData, AoCResult};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct Data<'a>(HashMap<&'a str, Vec<&'a str>>);

fn count_1(graph: &HashMap<&str, Vec<&str>>, from: &str, to: &str) -> usize {
    if from == to {
        return 1;
    }
    graph
        .get(from)
        .map(|outputs| {
            outputs
                .iter()
                .map(|output| count_1(graph, output, to))
                .sum()
        })
        .unwrap_or(0)
}

fn count_2<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    from: &'a str,
    to: &'a str,
    dac: bool,
    fft: bool,
    cache: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    if from == to {
        let count = if dac && fft { 1 } else { 0 };
        return count;
    }
    if let Some(&count) = cache.get(&(from, dac, fft)) {
        return count;
    }

    let new_dac = dac || from == "dac";
    let new_fft = fft || from == "fft";

    let sum = graph
        .get(from)
        .map(|outputs| {
            outputs
                .iter()
                .map(|new_from| count_2(graph, new_from, to, new_dac, new_fft, cache))
                .sum()
        })
        .unwrap_or(0);

    cache.insert((from, new_dac, new_fft), sum);
    sum
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(
            input
                .lines()
                .map(|line| {
                    let (from, to_str) = line.split_once(": ").unwrap();
                    let to_list = to_str.split(" ").collect();
                    (from, to_list)
                })
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(count_1(&self.0, "you", "out"))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(count_2(
            &self.0,
            "svr",
            "out",
            false,
            false,
            &mut HashMap::new(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "5");
    }

    #[test]
    fn part_2() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "2");
    }
}
