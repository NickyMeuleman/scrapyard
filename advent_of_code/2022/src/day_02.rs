use crate::AoCData;

pub struct Data(String);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input))
    }

    // shapes: 0 for rock, 1 for paper, 2 for scissors
    // outcomes: 0 for loss, 1 for draw, 2 for win
    // a game of Rock, Paper, Scissors can be expressed as the following equation
    // outcome = my_shape - opponent_shape + 1 (mod 3)
    // for part2, a reordering of terms
    // my_shape = opponent_shape + outcome - 1 (mod 3)

    fn part_1(&self) -> String {
        self.0
            .lines()
            // map every line to the score for that round
            .map(|line| {
                // transform A B C and X Y Z to 0 1 2 respectively by using their ASCII order
                let bytes = line.as_bytes();
                let left = (bytes[0] - b'A') as i8;
                let right = (bytes[2] - b'X') as i8;

                // 0 for rock, 1 for paper, 2 for scissors
                // 0 for loss, 1 for draw, 2 for win
                let opponent_shape = left;
                let my_shape = right;
                let outcome = (my_shape - opponent_shape + 1).rem_euclid(3);

                let shape_score = my_shape + 1;
                let outcome_score = 3 * outcome;
                (shape_score + outcome_score) as u32
            })
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.0
            .lines()
            // map every line to the score for that round
            .map(|line| {
                // transform A B C and X Y Z to 0 1 2 respectively by using their ASCII order
                let bytes = line.as_bytes();
                let left = (bytes[0] - b'A') as i8;
                let right = (bytes[2] - b'X') as i8;

                // 0 for rock, 1 for paper, 2 for scissors
                // 0 for loss, 1 for draw, 2 for win
                let opponent_shape = left;
                let outcome = right;
                let my_shape = (opponent_shape - 1 + outcome).rem_euclid(3);

                let shape_score = my_shape + 1;
                let outcome_score = 3 * outcome;
                (shape_score + outcome_score) as u32
            })
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(2);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "15");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(2);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "12");
    }
}
