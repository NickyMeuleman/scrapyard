use crate::AoCData;

pub struct Data {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    on: bool,
    cuboid: Cuboid,
}

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
}

impl Cuboid {
    fn volume(&self) -> i64 {
        // +1 because a min of 1 and max of 2 have length 2. max - min + 1
        (self.x_max - self.x_min + 1)
            * (self.y_max - self.y_min + 1)
            * (self.z_max - self.z_min + 1)
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        // check if cubes have an overlapping axis
        if self.x_max < other.x_min
            || self.y_max < other.y_min
            || self.z_max < other.z_min
            || self.x_min > other.x_max
            || self.y_min > other.y_max
            || self.z_min > other.z_max
        {
            return None;
        }

        // take max of min-axes and min of max-axis to get the axes of the overlap
        Some(Cuboid {
            x_min: self.x_min.max(other.x_min),
            x_max: self.x_max.min(other.x_max),
            y_min: self.y_min.max(other.y_min),
            y_max: self.y_max.min(other.y_max),
            z_min: self.z_min.max(other.z_min),
            z_max: self.z_max.min(other.z_max),
        })
    }

    /// returns the volume of self with all remaining cuboid overlaps subtracted
    fn corrected_volume(&self, rest: &[Cuboid]) -> i64 {
        let intersections: Vec<Cuboid> = rest.iter().filter_map(|c| c.intersection(self)).collect();

        let volume_intersections: i64 = intersections
            .iter()
            .enumerate()
            .map(|(i, c)| c.corrected_volume(&intersections[i + 1..]))
            .sum();

        self.volume() - volume_intersections
    }
}

impl Data {
    // similar, slightly slower solution
    // fn count_on(&self) -> i64 {
    //     // keep a vec of all seen instructions so far (existing)
    //     // loop through given instructions (self.instructions)
    //     // check for intersection of curr cube with all existing cubes
    //     // if intersection, add a new cube that cancels out the existing one
    //     // then add the current cube so it overrides the cube we just added
    //     let mut existing: Vec<Instruction> = Vec::new();

    //     for curr in &self.instructions {
    //         for old in existing.clone() {
    //             if let Some(intersection) = curr.cuboid.intersection(&old.cuboid) {
    //                 // add the intersection with an instruction that cancels out the existing cube,
    //                 // the wanted instruction for this intersection will be determined by the curr_instruction later
    //                 existing.push(Instruction {
    //                     on: !old.on,
    //                     cuboid: intersection,
    //                 });
    //             }
    //         }

    //         // only add the current cube if it's an "on" instruction
    //         // all "off" instructions will be added in our loop where we add intersecting cubes
    //         if curr.on {
    //             existing.push(*curr);
    //         }
    //     }

    //     // the total volume can be calculated by adding every "on" cube, and subtracting every "off" cube
    //     existing
    //         .iter()
    //         .map(|instruction| {
    //             let sign = if instruction.on { 1 } else { -1 };
    //             sign * instruction.cuboid.volume()
    //         })
    //         .sum()
    // }

    fn count_on(&self) -> i64 {
        (0..self.instructions.len())
            .filter(|&i| self.instructions[i].on)
            .map(|i| {
                let cuboid = &self.instructions[i].cuboid;
                let rest: Vec<Cuboid> = self.instructions[i + 1..]
                    .iter()
                    .map(|instruction| instruction.cuboid)
                    .collect();

                cuboid.corrected_volume(&rest)
            })
            .sum()
    }
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let instructions = input
            .trim()
            .lines()
            .map(|line| {
                let (on, rest) = line.split_once(" ")?;
                let pairs = rest
                    .split(',')
                    .map(|pair| {
                        // cut off x= part, split on .., take numbers
                        let (min, max) = pair[2..].split_once("..")?;
                        let min = min.parse().ok()?;
                        let max = max.parse().ok()?;
                        Some((min, max))
                    })
                    .collect::<Option<Vec<_>>>()?;
                let on = on == "on";
                let x_min = pairs.get(0)?.0;
                let x_max = pairs.get(0)?.1;
                let y_min = pairs.get(1)?.0;
                let y_max = pairs.get(1)?.1;
                let z_min = pairs.get(2)?.0;
                let z_max = pairs.get(2)?.1;
                let cuboid = Cuboid {
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    z_min,
                    z_max,
                };

                Some(Instruction { on, cuboid })
            })
            .collect::<Option<Vec<_>>>()?;

        Some(Self { instructions })
    }

    fn part_1(&self) -> String {
        // filter out instructions that are out of bounds first
        let instructions = self
            .instructions
            .iter()
            .filter(|instruction| {
                instruction.cuboid.x_min >= -50
                    && instruction.cuboid.y_min >= -50
                    && instruction.cuboid.z_min >= -50
                    && instruction.cuboid.x_max <= 50
                    && instruction.cuboid.y_max <= 50
                    && instruction.cuboid.z_max <= 50
            })
            .cloned()
            .collect();

        Data { instructions }.count_on().to_string()
    }

    fn part_2(&self) -> String {
        self.count_on().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(22);
        let data = Data::try_new(input).unwrap();
        // ran on sample data from p2!
        assert_eq!(data.part_1(), "474140");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(22);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "2758514936282235");
    }
}
