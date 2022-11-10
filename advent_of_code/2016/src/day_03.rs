use crate::AoCData;

pub struct Data(Vec<Vec<u32>>);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut triangles = Vec::new();
        for line in input.lines() {
            let mut sides = Vec::new();
            for num in line.split_whitespace() {
                let num: u32 = num.parse().ok()?;
                sides.push(num);
            }
            triangles.push(sides);
        }
        Some(Self(triangles))
    }

    fn part_1(&self) -> String {
        self.0
            .iter()
            .filter(|sides| {
                let total: u32 = sides.iter().sum();
                !(0..3).any(|i| sides[i] >= total - sides[i])
            })
            .count()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.0
            // turn horizontal groups of 3 numbers into vertical groups of 3 numbers
            .chunks(3)
            .flat_map(|chunk| {
                (0..3).map(|i| chunk.iter().map(|nums| nums[i]).collect::<Vec<u32>>())

                // alternative method with plain for loops
                // let mut triangles = Vec::new();
                // for i in 0..3 {
                //     let mut triangle = Vec::new();
                //     for triplet in chunk {
                //         triangle.push(triplet[i]);
                //     }
                //     triangles.push(triangle);
                // }
                // triangles
            })
            .filter(|sides| {
                let total: u32 = sides.iter().sum();
                !(0..3).any(|i| sides[i] >= total - sides[i])
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
        let input = utils::get_input(3);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "1050");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(3);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "1921");
    }
}
