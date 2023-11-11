use std::fmt::Display;

use itertools::Itertools;

use crate::{intcode::Computer, AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<i32>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .split(",")
                .filter_map(|s| s.parse().ok())
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let phases = [0, 1, 2, 3, 4];
        let mut largest = 0;
        for permutation in phases.iter().permutations(5) {
            let mut val = 0;
            for &phase in permutation {
                let mut computer = Computer::new();
                computer.set_memory(self.0.clone());
                computer.input(phase);
                computer.input(val);
                val = computer.run().unwrap();
            }
            largest = largest.max(val);
        }
        Ok(largest)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let phases = [5, 6, 7, 8, 9];
        let mut largest = 0;
        for permutation in phases.iter().permutations(5) {
            // setup once
            let mut amps = Vec::new();
            for phase in &permutation {
                let mut computer = Computer::new();
                computer.set_memory(self.0.clone());
                computer.input(**phase);
                amps.push(computer);
            }
            // To start the process, a 0 signal is sent to amplifier A's input exactly once.
            let mut signal = 0;
            // run in feedback loop
            'outer: loop {
                for (idx, _) in permutation.iter().enumerate() {
                    let curr_amp = &mut amps[idx];
                    curr_amp.input(signal);
                    if let Some(output) = curr_amp.run() {
                        signal = output;
                    } else {
                        break 'outer;
                    }
                }
            }
            largest = largest.max(signal);
        }
        Ok(largest)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "54321");

        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "43210");
    }

    #[test]
    fn part_2() {
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "139629729");

        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "18216");
    }
}
