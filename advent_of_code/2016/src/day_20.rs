use crate::AoCData;

pub struct Data(Vec<(u64, u64)>);

fn part_2_helper(mut ranges: Vec<(u64, u64)>, largest_possible: u64) -> u64 {
    let mut curr = 0;
    let mut count = 0;
    ranges.sort_unstable();
    for (start, end) in ranges.iter() {
        if curr < *start {
            count = count + start - curr;
            curr = end + 1;
            continue;
        }
        if curr >= *start && curr <= *end {
            curr = end + 1;
        }
    }

    if curr <= largest_possible {
        // largest_possible is inclusive, +1
        count += largest_possible - curr + 1;
    }

    count
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut ranges = Vec::new();
        for line in input.trim().lines() {
            let (start, end) = line.split_once('-')?;
            let start = start.parse().ok()?;
            let end = end.parse().ok()?;
            ranges.push((start, end));
        }
        Some(Self(ranges))
    }

    fn part_1(&self) -> String {
        let mut curr = 0;
        let mut ranges = self.0.clone();
        ranges.sort_unstable();
        for (start, end) in ranges.iter() {
            if curr < *start {
                return curr.to_string();
            }
            if curr >= *start && curr <= *end {
                curr = end + 1;
            }
        }
        String::new()
    }

    fn part_2(&self) -> String {
        part_2_helper(self.0.clone(), 4_294_967_295).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(20);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "3");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(20);
        let data = Data::try_new(input).unwrap();
        assert_eq!(part_2_helper(data.0, 9), 2);
    }
}
