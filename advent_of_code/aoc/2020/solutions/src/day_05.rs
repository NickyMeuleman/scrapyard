use std::fmt::Display;

use crate::{AoCData, AoCResult};

const ROW_COUNT: u32 = 127;
const COL_COUNT: u32 = 7;

#[derive(Debug, Clone)]
pub struct Data<'a>(Vec<&'a str>);

fn seat_id(pass: &str) -> u32 {
    let row_steps: Vec<char> = pass[..7].chars().collect();
    let col_steps: Vec<char> = pass[7..].chars().collect();
    let row = search(row_steps, 0, ROW_COUNT);
    let col = search(col_steps, 0, COL_COUNT);
    row * 8 + col
}

fn search(mut steps: Vec<char>, min: u32, max: u32) -> u32 {
    if steps.len() == 1 {
        match steps[0] {
            'F' | 'L' => return min,
            'B' | 'R' => return max,
            _ => panic!("invalid last step"),
        }
    }

    let mid = (min + max) / 2;
    let step = steps[0];
    let steps = steps.drain(1..).collect();

    match step {
        'F' | 'L' => search(steps, min, mid),
        'B' | 'R' => search(steps, mid + 1, max),
        _ => panic!("invalid intermediary step"),
    }
}

fn find_gap(mut passes: Vec<u32>) -> u32 {
    passes.sort();

    let mut prev = passes[0];
    let passes: Vec<u32> = passes.drain(1..).collect();
    for curr in passes {
        if prev != curr - 1 {
            break;
        }
        prev += 1;
    }
    prev + 1
}

fn find_gap_2(mut passes: Vec<u32>) -> u32 {
    passes.sort();

    let passes_one_over = passes.clone().into_iter().skip(1);
    passes
        .into_iter()
        .zip(passes_one_over)
        .find_map(|(prev, curr)| {
            if curr - prev == 2 {
                Some(curr - 1)
            } else {
                None
            }
        })
        .unwrap_or(0)
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input.lines().collect()))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result = self
            .0
            .iter()
            .map(|pass| seat_id(pass))
            .max()
            .unwrap_or(0);

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let passes: Vec<u32> = self
            .0
            .iter()
            .map(|pass| seat_id(pass))
            .collect();
        // test out 2 methods
        let num1 = find_gap(passes.clone());
        let num2 = find_gap_2(passes);
        assert_eq!(num1, num2);
        Ok(num1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "FBFBBFFRLR";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "357");
    }
}
