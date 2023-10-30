use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
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
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
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

        Ok(total)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
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

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "13140");
    }

    #[test]
    fn part_2() {
        let input = "addx 1
addx 4
addx 1
noop
noop
addx 4
addx 1
addx 4
noop
noop
addx 5
noop
noop
noop
addx -3
addx 9
addx -1
addx 5
addx -28
addx 29
addx 2
addx -28
addx -7
addx 10
noop
noop
noop
noop
noop
addx -2
addx 2
addx 25
addx -18
addx 3
addx -2
addx 2
noop
addx 3
addx 2
addx 5
addx 2
addx 2
addx 3
noop
addx -15
addx 8
addx -28
noop
noop
noop
addx 7
addx -2
noop
addx 5
noop
noop
noop
addx 3
noop
addx 3
addx 2
addx 5
addx 2
addx 3
addx -2
addx 3
addx -31
addx 37
addx -28
addx -9
noop
noop
noop
addx 37
addx -29
addx 4
noop
addx -2
noop
noop
noop
addx 7
noop
noop
noop
addx 5
noop
noop
noop
addx 4
addx 2
addx 4
addx 2
addx 3
addx -2
noop
noop
addx -34
addx 6
noop
noop
noop
addx -4
addx 9
noop
addx 5
noop
noop
addx -2
noop
addx 7
noop
addx 2
addx 15
addx -14
addx 5
addx 2
addx 2
addx -32
addx 33
addx -31
addx -2
noop
noop
addx 1
addx 3
addx 2
noop
addx 2
noop
addx 7
noop
addx 5
addx -6
addx 4
addx 5
addx 2
addx -14
addx 15
addx 2
noop
addx 3
addx 4
noop
addx 1
noop
noop
";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "EZFPRAKL");
    }
}
