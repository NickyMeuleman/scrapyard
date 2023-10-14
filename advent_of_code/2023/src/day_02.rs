use std::fmt::Display;

use crate::AoCData;
#[derive(Debug)]
pub struct Data(Vec<i32>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> Option<Self> {
        Some(Self(
            input
                .split(",")
                .filter_map(|s| s.parse().ok())
                .collect(),
        ))
    }

    fn part_1(&self) -> impl Display {
        let mut pointer = 0;
        let mut list = self.0.clone();

        list[1] = 12;
        list[2] = 2;

        loop {
            let opcode = list[pointer];
            if opcode == 99 {
                break;
            }

            let pos1 = list[pointer + 1];
            let pos2 = list[pointer + 2];
            let pos3 = list[pointer + 3];
            let num1 = list[pos1 as usize];
            let num2 = list[pos2 as usize];
            let result = match opcode {
                1 => num1 + num2,
                2 => num1 * num2,
                _ => panic!("At the disco"),
            };

            list[pos3 as usize] = result;
            pointer += 4;
        }

        list[0]
    }

    fn part_2(&self) -> impl Display {
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut list = self.0.clone();

                list[1] = noun;
                list[2] = verb;

                let mut pointer = 0;

                loop {
                    let opcode = list[pointer];
                    if opcode == 99 {
                        break;
                    }

                    let pos1 = list[pointer + 1];
                    let pos2 = list[pointer + 2];
                    let pos3 = list[pointer + 3];
                    let num1 = list[pos1 as usize];
                    let num2 = list[pos2 as usize];
                    let result = match opcode {
                        1 => num1 + num2,
                        2 => num1 * num2,
                        _ => panic!("At the disco"),
                    };

                    list[pos3 as usize] = result;
                    pointer += 4;
                }

                if list[0] == 19690720 {
                    return (100 * noun + verb).to_string();
                }
            }
        }
        "No solution was found".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::get_input;

    #[test]
    fn part_1() {
        let input = get_input(2, true).unwrap();
        let data = Data::try_new(&input).unwrap();
        let result = data.part_1().to_string();
        assert_eq!(result, "");
    }

    #[test]
    fn part_2() {
        let input = get_input(2, true).unwrap();
        let data = Data::try_new(&input).unwrap();
        let result = data.part_2().to_string();
        assert_eq!(result, "");
    }
}
