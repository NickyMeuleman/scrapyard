use crate::AoCData;

pub struct Data(String);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> String {
        let vowels = &['a', 'e', 'i', 'o', 'u'];
        let naughty_patterns = &["ab", "cd", "pq", "xy"];

        self.0
            .lines()
            // contains at least three vowels
            .filter(|line| line.chars().filter(|c| vowels.contains(c)).count() >= 3)
            // contains a letter that appears twice in a row
            .filter(|line| {
                line.as_bytes()
                    .windows(2)
                    .any(|window| window[0] == window[1])
            })
            // does not include a naughty pattern
            .filter(|line| !naughty_patterns.iter().any(|pat| line.contains(pat)))
            .count()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.0
            .lines()
            // contains a pair of any two letters that appears at least twice in the string without overlapping
            .filter(|line| {
                line.as_bytes().windows(2).enumerate().any(|(i, pair)| {
                    if let Some(idx) = line.rfind(std::str::from_utf8(pair).unwrap()) {
                        idx > i + 1
                    } else {
                        false
                    }
                })
            })
            // contains at least one letter which repeats with exactly one letter between the
            .filter(|line| {
                line.as_bytes()
                    .windows(3)
                    .any(|window| window[0] == window[2])
            })
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(5);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(5);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "");
    }
}
