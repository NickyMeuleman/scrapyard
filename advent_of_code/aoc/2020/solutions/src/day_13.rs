use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data((i128, Vec<Option<i128>>));

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut lines = input.lines();
        // first line, mutate lines to remove it by calling .next() on the iterator
        let target = lines
            .next()
            .and_then(|line| line.parse().ok())
            .unwrap_or(0);
        // second line, parse into vec of Option since every position, including the x's are relevant in part 2
        let busses = lines
            .next()
            .unwrap()
            .split(",")
            .map(|bus| match bus {
                "x" => None,
                n => n.parse().ok(),
            })
            .collect();

        Ok(Self((target, busses)))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        // threw my (working, but verbose) implementation away and using @arjandepooter's
        let result = self
            .0
             .1
            .iter()
            .filter_map(|bus| match bus {
                Some(id) => Some((id, id - self.0 .0 % id)),
                None => None,
            })
            .min_by_key(|(_, offset)| *offset)
            .map_or(0, |(id, offset)| id * offset);

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // q: However, with so many bus IDs in your list, surely the actual earliest timestamp will be larger than 100000000000000!
        // conclusion: u32 is not big enough

        // same code as @aceshades

        // starting t at 0 since the first possible waittime is no waittime at all, all busses might match the sequence as they are given
        // starting mod at 1 because for the first iteration num % 1 works out to num
        // the for loop and nested while cause t = the first bus_id, mode = the first bus_id after the first iteration
        // t increments to the first bus_id
        // Mode having a starting value of 1 means mode becomes 7 after the first iteration

        // subsequent iterations:
        // the result is incremented by the mode until t + offset is cleanly divisible by the current bus_id (it fits in the question sequence)
        // it is incremented by the mode and not by 1 because that's the only number that fits the pattern for every previous bus,
        // incrementing by a lower number than the current mode would be wasteful, as the sequence won't fit,

        // after t + offset is cleanly divisible by the bus_id, the mode is updated by multiplying the current mode by the bus_id,
        // as that is the next time all busses wil match up occording to the sequence

        // EXAMPLE RUNTHROUGH:
        // example input: 7,13,x,x,59,x,31,19
        // after the first iteration t = 0, mode = 7

        // the second bus has to arrive on t + 1 (0 + 1 = 1) (the variable named offset in the code is that +1)
        // t is incremented by a time that will fit the previous sequence (mode = 7) until t + offset is cleanly divisibly by 7 (mode = 7)
        // t = 77
        // the time that fits the first bus (mode = 7) is then multiplied by the id of the current bus (bus_id = 13)
        // mode = 7 * 13 = 91
        // since the result of that multiplication is the first time the busses will line up again and fit the sequence for every previously checked bus
        // note: all bus_ids are prime numbers so that logic holds, if they weren't the mode would have to become a single common divisor of bus_id + offset for every bus in the sequence another way.

        // the third has to arrive on 7 + 4 (t = 7, offset = 4, t + offset = 13)
        // t is incremented by a time that will fit the previous sequence (mode = 91) until t + offset fits the sequence
        // t = 350
        // the time that fits the two previous busses (mode = 91) is then multiplied by the id of the current bus (bus_id = 59)
        // since the result of that multiplication is the first time all checked busses will line up again and fit the sequence
        // mode = 7 * 13  * 59 = 91 * 59 = 5.369

        // fourth bus_id = 31, offset = 6
        // t is incremented by mode (= 5.369) until the sequence fits
        // t = 70147
        // mode is then updated to be the next time all checked busses line up
        // mode = 7 * 13 * 59 * 31 = 5.369 * 31 = 166.439

        // fifth and final bus_id = 19, offset = 7
        // t = t + 166.439 until sequence fits
        // t = 1068781
        // since this is the last bus in the sequence, stop here

        let mut t = 0;
        let mut mode = 1;
        let busses = self
            .0
             .1
            .iter()
            .enumerate()
            .filter(|(_, &item)| item != None);
        for (offset, bus_id) in busses {
            while (t + offset as i128) % bus_id.unwrap() != 0 {
                t += mode;
            }
            mode *= bus_id.unwrap();
        }
        Ok(t)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "939
7,13,x,x,59,x,31,19";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "295");
    }

    #[test]
    fn part_2() {
        let input = "939
17,x,13,19";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "3417");
    }
}
