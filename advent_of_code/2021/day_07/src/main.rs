use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Data {
    distances: Vec<usize>,
}

impl Data {
    fn part_one(&self) -> usize {
        let min = *self.distances.iter().min().unwrap();
        let max = *self.distances.iter().max().unwrap();

        let mut min_total: Option<usize> = None;

        for target_pos in min..=max {
            let mut total: usize = 0;

            for start_pos in &self.distances {
                // get amount a crab has to move to get to target_pos
                let distance = (target_pos as isize - *start_pos as isize).abs();

                // get cost for that distance
                let cost = distance;

                // add cost to total
                total += cost as usize;
            }

            // set the min_total to the new total only if it is smaller than the current min_total,
            // or it's the first total
            min_total = match min_total {
                Some(num) => Some(num.min(total)),
                None => Some(total),
            }
        }

        min_total.unwrap()
    }

    fn part_two(&self) -> usize {
        let min = *self.distances.iter().min().unwrap();
        let max = *self.distances.iter().max().unwrap();

        let mut min_total: Option<usize> = None;

        for target_pos in min..=max {
            let mut total: usize = 0;
            for start_pos in &self.distances {
                // get amount a crab has to move to get to target_pos
                let distance = (target_pos as isize - *start_pos as isize).abs();

                // get cost for that distance
                // This is the sum of an arithmetic sequence: https://en.wikipedia.org/wiki/Arithmetic_progression
                let n = distance;
                let a1 = 1;
                let a2 = distance;
                let cost = n * (a1 + a2) / 2;

                // add cost to total
                total += cost as usize;
            }

            // set the min_total to the new total only if it is smaller than the current min_total,
            // or it's the first total
            min_total = match min_total {
                Some(num) => Some(num.min(total)),
                None => Some(total),
            }
        }

        min_total.unwrap()
    }
}

impl FromStr for Data {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            distances: input.split(",").filter_map(|s| s.parse().ok()).collect(),
        })
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = input.parse().unwrap();
    println!("Part one answer: {}", data.part_one());
    println!("Part two answer: {}", data.part_two());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 37);
    }

    #[test]

    fn part_two_example() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 168);
    }
}
