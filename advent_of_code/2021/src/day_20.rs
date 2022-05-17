use crate::AoCData;

#[derive(Debug, Clone)]
pub struct Data {
    algorithm: Vec<bool>,
    image: Image,
}

#[derive(Debug, Clone)]
struct Image {
    data: Vec<Vec<bool>>,
}

impl Image {
    fn new(data: Vec<Vec<bool>>) -> Self {
        Self { data }
    }

    // we like this image, so we put a ring on it
    fn pad(&mut self, state: bool) {
        // a row filled with "state"
        let row: Vec<bool> = vec![state; self.data[0].len()];
        // add a first row full of "state"
        self.data.insert(0, row.clone());
        // add a last row full of "state"
        self.data.push(row);

        // add a left column full of "state"
        // add a right column full of "state"
        for row in &mut self.data {
            row.insert(0, state);
            row.push(state);
        }
    }

    fn enhance(&mut self, algorithm: &[bool]) -> Self {
        // to deal with the infinite grid, expand it with the current state of the infinite surrounding
        // this state is the same as the state of the very first point in every step
        // a . on every odd step, and a # on every even step
        let current_infinity_state = self.data[0][0];
        self.pad(current_infinity_state);

        // create a new in-progress image for this step
        let mut data = self.data.clone();

        // go through every pixel in the unpadded image and calculate what the new pixel should be
        for row in 1..self.data.len() - 1 {
            for col in 1..self.data[row].len() - 1 {
                // construct the 3x3 square this step considers
                let square = vec![
                    // up left
                    self.data[row - 1][col - 1],
                    // up
                    self.data[row - 1][col],
                    // up right
                    self.data[row - 1][col + 1],
                    // left
                    self.data[row][col - 1],
                    // center
                    self.data[row][col],
                    // right
                    self.data[row][col + 1],
                    // down left
                    self.data[row + 1][col - 1],
                    // down
                    self.data[row + 1][col],
                    // down right
                    self.data[row + 1][col + 1],
                ];

                // concatenate every point in the square to get the index in the algorithm
                // doing it this way because collecting that square into a string and then parsing that as binary is slow
                let idx = square.into_iter().fold(0, |acc, state| {
                    // shift the binary so far one to the left
                    let acc = acc * 2;
                    // add 1 to it if the current state is true
                    acc | if state { 1 } else { 0 }
                });
                // equivalent would be:
                // .fold(0, |a, b| a << 1 | (b) as usize);
                // that << is the same as multiplying by 2, but more computer sciency, because binary numbers go brrrrrr

                // the value in the algorithm at that index is the value that replaces the middle point in the in-progress image
                data[row][col] = algorithm[idx];
            }
        }

        // we padded with the current infinity state at the start of this function, but didn't calculate what to replace those points with in our loop
        // the infinity can flip state each enhancement, let's check and apply that before we end the function

        // the state the infinity turned into this step
        let new_infinity_state = algorithm[vec![current_infinity_state; 9]
            .into_iter()
            .fold(0, |acc, state| acc << 1 | state as usize)];

        // store the amount of rows before we mutably borrow self
        let last = data.len() - 1;

        // overwrite the first and last rows
        // overwrite the first and last colums
        for (idx, row) in data.iter_mut().enumerate() {
            row[0] = new_infinity_state;
            row[last] = new_infinity_state;
            if idx == 0 || idx == last {
                for state in row {
                    *state = new_infinity_state
                }
            }
        }

        Image::new(data)
    }

    fn count_on(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|&v| *v).count())
            .sum()
    }
}

impl AoCData for Data {
    fn new(input: String) -> Self {
        let (algorithm, image) = input.trim().split_once("\n\n").unwrap();
        let algorithm = algorithm
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => unreachable!("invalid input"),
            })
            .collect();
        let image = image
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!("invalid input"),
                    })
                    .collect()
            })
            .collect();

        Self {
            algorithm,
            image: Image::new(image),
        }
    }

    fn part_1(&self) -> String {
        let mut img = self.image.clone();
        // curse you, infinite grid after 1 step
        img.pad(false);

        for _ in 0..2 {
            img = img.enhance(&self.algorithm);
        }

        img.count_on().to_string()
    }

    fn part_2(&self) -> String {
        let mut img = self.image.clone();
        // curse you, infinite grid after 1 step
        img.pad(false);

        for _ in 0..50 {
            img = img.enhance(&self.algorithm);
        }

        img.count_on().to_string()
    }

    fn solve(self) -> (String, String) {
        let mut img = self.image.clone();
        let mut part_1 = 0;
        // curse you, infinite grid after 1 step
        img.pad(false);

        for i in 0..50 {
            img = img.enhance(&self.algorithm);
            if i == 1 {
                part_1 = img.count_on();
            }
        }

        (part_1.to_string(), img.count_on().to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(20);
        let data = Data::new(input);
        assert_eq!(data.part_1(), "35");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(20);
        let data = Data::new(input);
        assert_eq!(data.part_2(), "3351");
    }
}
