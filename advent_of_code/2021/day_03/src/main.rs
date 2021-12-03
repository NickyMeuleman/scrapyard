use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data = parse(&input);
    println!("Part one answer: {}", part_one(data.clone()));
    println!("Part two answer: {}", part_two(data));
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part_one(data: Vec<&str>) -> usize {
    let len = data[0].len();
    let (gamma, epsilon) = (0..len).fold(
        (Vec::with_capacity(len), Vec::with_capacity(len)),
        |(mut gamma, mut epsilon), idx| {
            // find least and most common
            let (zeroes, ones) = data.iter().fold((0, 0), |(zeroes, ones), binary_number| {
                let bit = binary_number.chars().nth(idx).unwrap();
                if bit == '1' {
                    (zeroes, ones + 1)
                } else {
                    (zeroes + 1, ones)
                }
            });
            // add least common to epsilon, most common to gamma
            if zeroes > ones {
                gamma.push("0");
                epsilon.push("1");
            } else {
                gamma.push("1");
                epsilon.push("0");
            }
            (gamma, epsilon)
        },
    );

    let gamma = usize::from_str_radix(&gamma.join(""), 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon.join(""), 2).unwrap();

    gamma * epsilon
}

struct Candidates<'a> {
    o: Vec<&'a str>,
    co2: Vec<&'a str>,
}

struct FinalNums {
    o: Option<u32>,
    co2: Option<u32>,
}

fn part_two(data: Vec<&str>) -> u32 {
    let len = data[0].len();

    let (_, nums) = (0..len).fold(
        (
            Candidates {
                o: data.clone(),
                co2: data.clone(),
            },
            FinalNums { o: None, co2: None },
        ),
        |(mut candidates, mut final_nums), idx| {
            let (o_zeroes, o_ones) = count_ones_zeroes(&candidates.o, idx);
            let most_common = if o_ones >= o_zeroes { '1' } else { '0' };

            let (co2_zeroes, co2_ones) = count_ones_zeroes(&candidates.co2, idx);
            let least_common = if co2_zeroes <= co2_ones { '0' } else { '1' };

            candidates.o = filter_candidates(candidates.o, idx, most_common);
            candidates.co2 = filter_candidates(candidates.co2, idx, least_common);

            // if a final number has not yet been found and the candidate vector has length 1, set it
            if let None = final_nums.co2 {
                if candidates.co2.len() == 1 {
                    final_nums.co2 = Some(u32::from_str_radix(candidates.co2[0], 2).unwrap());
                }
            }
            if let None = final_nums.o {
                if candidates.o.len() == 1 {
                    final_nums. o= Some(u32::from_str_radix(candidates.o[0], 2).unwrap());
                }
            }

            (candidates, final_nums)
        },
    );

    nums.o.unwrap() * nums.co2.unwrap()
}

/// returns the sum of all zeroes and ones at a given index for each item in the data parameter
fn count_ones_zeroes(data: &Vec<&str>, idx: usize) -> (u32, u32) {
    data.iter().fold((0, 0), |(zeroes, ones), binary_number| {
        let bit = binary_number.chars().nth(idx).unwrap();
        if bit == '1' {
            (zeroes, ones + 1)
        } else {
            (zeroes + 1, ones)
        }
    })
}

/// return only the candidates with the given bit at a the given index
fn filter_candidates(candidates: Vec<&str>, idx: usize, bit: char) -> Vec<&str> {
    candidates
        .into_iter()
        .filter(|binary_num| binary_num.chars().nth(idx).unwrap() == bit)
        .collect()
}

// initial solution with more looping
// fn part_two(data: Vec<&str>) -> usize {
//     let len = data[0].len();
//     let mut oxygen_candidates = data.clone();
//     // find oxygen value
//     for idx in 0..len {
//         // find most common bit
//         let (zeroes, ones) =
//             oxygen_candidates
//                 .iter()
//                 .fold((0, 0), |(zeroes, ones), binary_number| {
//                     let bit = binary_number.chars().nth(idx).unwrap();
//                     if bit == '1' {
//                         (zeroes, ones + 1)
//                     } else {
//                         (zeroes + 1, ones)
//                     }
//                 });
//         let most_common = if ones >= zeroes { '1' } else { '0' };
//         // filter candidates to ones that only have most common bit at idx
//         oxygen_candidates = oxygen_candidates
//             .into_iter()
//             .filter(|binary_num| binary_num.chars().nth(idx).unwrap() == most_common)
//             .collect();
//     }
//     // oxygen_candidates should have len 1 here
//     let oxygen = usize::from_str_radix(&oxygen_candidates[0], 2).unwrap();

//     let mut co2_candidates = data.clone();
//     // find co2 value
//     for idx in 0..len {
//         // find most common bit
//         let (zeroes, ones) = co2_candidates
//             .iter()
//             .fold((0, 0), |(zeroes, ones), binary_number| {
//                 let bit = binary_number.chars().nth(idx).unwrap();
//                 if bit == '1' {
//                     (zeroes, ones + 1)
//                 } else {
//                     (zeroes + 1, ones)
//                 }
//             });
//         let least_common = if zeroes <= ones { '0' } else { '1' };
//         // filter candidates to ones that only have most common bit at idx
//         co2_candidates = co2_candidates
//             .into_iter()
//             .filter(|binary_num| binary_num.chars().nth(idx).unwrap() == least_common)
//             .collect();

//         if co2_candidates.len() == 1 {
//             break;
//         }
//     }
//     // co2_candidates should have len 1 here
//     let co2 = usize::from_str_radix(co2_candidates[0], 2).unwrap();
//     oxygen * co2
// }

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let data = parse(input);
        assert_eq!(part_one(data), 198);
    }

    #[test]
    fn part_two_example() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let data = parse(input);
        assert_eq!(part_two(data), 230);
    }
}
