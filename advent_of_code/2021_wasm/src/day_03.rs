use crate::AoCData;

pub struct Data {
    nums: Vec<Vec<u8>>,
}

fn to_u32(bits: &[u8]) -> u32 {
    bits.iter()
        .fold(0, |result, &bit| (result << 1) ^ bit as u32)
}

fn get_digit_bias(values: &[Vec<u8>], idx: usize) -> i64 {
    values.iter().fold(0, |acc, num| match num[idx] {
        0 => acc - 1,
        1 => acc + 1,
        _ => unreachable!("invalid input"),
    })
}

/// return only the candidates with the given bit at a the given index
fn filter_candidates(candidates: Vec<Vec<u8>>, idx: usize, wanted: u8) -> Vec<Vec<u8>> {
    candidates
        .into_iter()
        .filter(|val| val[idx] == wanted)
        .collect()
}

impl AoCData for Data {
    fn new(input: &str) -> Self {
        Self {
            nums: input
                .trim()
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '1' => 1,
                            '0' => 0,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn part_1(&self) -> String {
        let num_digits = self.nums[0].len();

        let (gamma, epsilon) = (0..num_digits)
            .map(|idx| {
                // create iterator of numbers that are positive if 1 is most common
                // negative if 0 is most common
                // and 0 if they are equally as common (this does not happen in the input)
                get_digit_bias(&self.nums, idx)
            })
            // for each entry, set the corresponding bit index in gamma and epsilon
            // gamma gets a 1 if the number is positive, epsilon a 0
            // gamma gets a 0 if the number is negative, epsilon a 1
            .fold((Vec::new(), Vec::new()), |(mut gamma, mut epsilon), num| {
                if num > 0 {
                    gamma.push(1);
                    epsilon.push(0)
                } else {
                    gamma.push(0);
                    epsilon.push(1)
                }

                (gamma, epsilon)
            });

        let result = to_u32(&gamma) * to_u32(&epsilon);
        result.to_string()
    }

    fn part_2(&self) -> String {
        let (o2, co2) = (0..self.nums[0].len()).fold(
            (self.nums.clone(), self.nums.clone()),
            |(mut o2_candidates, mut co2_candidates), idx| {
                if o2_candidates.len() > 1 {
                    let bias = get_digit_bias(&o2_candidates, idx);
                    let wanted = if bias >= 0 { 1 } else { 0 };
                    o2_candidates = filter_candidates(o2_candidates, idx, wanted);
                }
                if co2_candidates.len() > 1 {
                    let bias = get_digit_bias(&co2_candidates, idx);
                    let wanted = if bias >= 0 { 0 } else { 1 };
                    co2_candidates = filter_candidates(co2_candidates, idx, wanted);
                }
                (o2_candidates, co2_candidates)
            },
        );

        let result = to_u32(&o2[0]) * to_u32(&co2[0]);
        result.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(3);
        let data = Data::new(&input);
        assert_eq!(data.part_1(), "198");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(3);
        let data = Data::new(&input);
        assert_eq!(data.part_2(), "230");
    }
}
