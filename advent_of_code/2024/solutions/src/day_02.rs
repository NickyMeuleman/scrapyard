use crate::{AoCData, AoCResult};
use itertools::*;
use std::fmt::Display;

// fn is_safe(report: &[i32]) -> bool {
//     let dir = (report[0] - report[1]).signum();
//     for (a, b) in report.iter().tuple_windows() {
//         let diff = a - b;
//         if diff == 0 || diff.abs() > 3 || diff.signum() != dir {
//             return false;
//         }
//     }
//     true
// }

fn is_safe(report: &[i32]) -> bool {
    let dir = (report[0] - report[1]).signum();
    report
        .iter()
        .tuple_windows()
        .all(|(&a, &b)| {
            let diff = a - b;
            diff != 0 && diff.abs() <= 3 && diff.signum() == dir
        })
}

// fn is_safe_p2(report: &[i32]) -> bool {
//     if is_safe(report) {
//         return true;
//     }
//     for idx in 0..report.len() {
//         let mut report = report.to_vec();
//         report.remove(idx);
//         if is_safe(&report) {
//             return true;
//         }
//     }
//     false
// }

fn is_safe_p2(report: &[i32]) -> bool {
    (0..report.len()).any(|idx| {
        let mut report = report.to_vec();
        report.remove(idx);
        is_safe(&report)
    })
}

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<i32>>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect()
                })
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        // let mut count = 0;
        // for line in input.lines() {
        //     let report = parse_line(line);
        //     if is_safe(&report) {
        //         count += 1;
        //     }
        // }
        // count
        Ok(self
            .0
            .iter()
            .filter(|&report| is_safe(report))
            .count())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // let mut count = 0;
        // for line in input.lines() {
        //     let report = parse_line(line);
        //     if is_safe_p2(report) {
        //         count += 1;
        //     }
        // }
        // count
        Ok(self
            .0
            .iter()
            .filter(|report| is_safe_p2(report))
            .count())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "2");
    }

    #[test]
    fn part_2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "4");
    }
}
