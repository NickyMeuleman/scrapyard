use crate::AoCData;

pub struct Data<'a>(&'a str);

const COLS: usize = 40;
const ROWS: usize = 6;
const SPRITE_WIDTH: u32 = 3;
const LETTER_WIDTH: usize = 5;

fn get_pixel(cycle: usize, x: i32) -> bool {
    let curr_col = (cycle - 1) % COLS;
    (curr_col as i32).abs_diff(x) <= SPRITE_WIDTH / 2
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> String {
        let mut x = 1;
        let mut cycle = 1;
        let mut total = 0;

        for line in self.0.lines() {
            if cycle % 40 == 20 {
                total += cycle * x;
            }
            cycle += 1;

            if let Some(("addx", num)) = line.split_once(' ') {
                if cycle % 40 == 20 {
                    total += cycle * x;
                }
                let num: i32 = num.parse().unwrap();
                x += num;
                cycle += 1;
            }
        }

        total.to_string()
    }

    fn part_2(&self) -> String {
        let mut x = 1;
        let mut cycle = 1;
        let mut screen = [false; COLS * ROWS];

        for line in self.0.lines() {
            screen[cycle - 1] = get_pixel(cycle, x);
            cycle += 1;

            if let Some(("addx", num)) = line.split_once(' ') {
                screen[cycle - 1] = get_pixel(cycle, x);
                cycle += 1;
                let num: i32 = num.parse().unwrap();
                x += num;
            }
        }

        let pixels: [[bool; 40]; 6] = screen
            .chunks_exact(40)
            .filter_map(|chunk| chunk.try_into().ok())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let mut letters = vec![];
        for letter_idx in 0..COLS / LETTER_WIDTH {
            let mut letter: Vec<&[bool]> = vec![];
            for col_idx in 0..ROWS {
                let line = &pixels[col_idx]
                    [letter_idx * LETTER_WIDTH..letter_idx * LETTER_WIDTH + LETTER_WIDTH];
                letter.push(line);
            }
            letters.push(letter);
        }

        let mut result = String::new();
        for letter in letters.iter() {
            #[rustfmt::skip]
                let recognized = match letter[..] {
                    [
                        [false, true, true, false, false],
                        [true, false, false, true, false],
                        [true, false, false, true, false],
                        [true, true, true, true, false],
                        [true, false, false, true, false],
                        [true, false, false, true, false]
                    ] => { 'A' }
                    [
                        [true, true, true, false, false],
                        [true, false, false, true, false],
                        [true, true, true, false, false],
                        [true, false, false, true, false],
                        [true, false, false, true, false],
                        [true, true, true, false, false]
                    ] => { 'B' }
                    [
                        [false, true, true, false, false],
                        [true, false, false, true, false],
                        [true, false, false, false, false],
                        [true, false, false, false, false],
                        [true, false, false, true, false],
                        [false, true, true, false, false]
                    ] => { 'C' }
                    [
                        [true, true, true, true, false],
                        [true, false, false, false, false],
                        [true, true, true, false, false],
                        [true, false, false, false, false],
                        [true, false, false, false, false],
                        [true, true, true, true, false]
                    ] => { 'E' }
                    [
                        [true, true, true, true, false],
                        [true, false, false, false, false],
                        [true, true, true, false, false],
                        [true, false, false, false, false],
                        [true, false, false, false, false],
                        [true, false, false, false, false]
                    ] => { 'F' }
                    [
                        [false, true, true, false, false],
                        [true, false, false, true, false],
                        [true, false, false, false, false],
                        [true, false, true, true, false],
                        [true, false, false, true, false],
                        [false, true, true, true, false]
                    ] => { 'G' }
                    [
                        [true, false, false, true, false],
                        [true, false, false, true, false],
                        [true, true, true, true, false],
                        [true, false, false, true, false],
                        [true, false, false, true, false],
                        [true, false, false, true, false]
                    ] => { 'H' }
                    [
                        [false, true, true, true, false],
                        [false, false, true, false, false],
                        [false, false, true, false, false],
                        [false, false, true, false, false],
                        [false, false, true, false, false],
                        [false, true, true, true, false]
                    ] => { 'I' }
                    [
                        [false, false, true, true, false],
                        [false, false, false, true, false],
                        [false, false, false, true, false],
                        [false, false, false, true, false],
                        [true, false, false, true, false],
                        [false, true, true, false, false]
                    ] => { 'J' }
                    [
                        [true, false, false, true, false],
                        [true, false, true, false, false],
                        [true, true, false, false, false],
                        [true, false, true, false, false],
                        [true, false, true, false, false],
                        [true, false, false, true, false]
                    ] => { 'K' }
                    [
                        [true, false, false, false, false],
                        [true, false, false, false, false],
                        [true, false, false, false, false],
                        [true, false, false, false, false],
                        [true, false, false, false, false],
                        [true, true, true, true, false]
                    ] => { 'L' }
                    [
                        [false, true, true, false, false],
                        [true, false, false, true, false],
                        [true, false, false, true, false],
                        [true, false, false, true, false],
                        [true, false, false, true, false],
                        [false, true, true, false, false]
                    ] => { 'O' }
                    [
                        [true, true, true, false, false],
                        [true, false, false, true, false],
                        [true, false, false, true, false],
                        [true, true, true, false, false],
                        [true, false, false, false, false],
                        [true, false, false, false, false]
                    ] => { 'P' }
                    [
                        [true, true, true, false, false],
                        [true, false, false, true, false],
                        [true, false, false, true, false],
                        [true, true, true, false, false],
                        [true, false, true, false, false],
                        [true, false, false, true, false]
                    ] => { 'R' }
                    [
                        [false, true, true, true, false],
                        [true, false, false, false, false],
                        [true, false, false, false, false],
                        [false, true, true, false, false],
                        [false, false, false, true, false],
                        [true, true, true, false, false]
                    ] => { 'S' },
                    [
                        [true, false, false, true, false],
                        [true, false, false, true, false],
                        [true, false, false, true, false],
                        [true, false, false, true, false],
                        [true, false, false, true, false],
                        [false, true, true, false, false]
                    ] => { 'U' },
                    [
                        [true, false, false, false, true],
                        [true, false, false, false, true],
                        [false, true, false, true, false],
                        [false, false, true, false, false],
                        [false, false, true, false, false],
                        [false, false, true, false, false],
                    ] => { 'Y' },
                    [
                        [true, true, true, true, false],
                        [false, false, false, true, false],
                        [false, false, true, false, false],
                        [false, true, false, false, false],
                        [true, false, false, false, false],
                        [true, true, true, true, false]
                    ] => { 'Z' }
                    _ => '?',
                };
            result.push(recognized);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(10);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "13140");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(10);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_2(), "EZFPRAKL");
    }
}
