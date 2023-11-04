use std::fmt::{self, Display};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

const LETTER_WIDTH: usize = 5;
const ROWS: usize = 6;
const COLS: usize = 50;

#[derive(Debug, Clone)]
pub struct Data {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

struct Screen {
    pixels: [[bool; COLS]; ROWS],
}

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut screen = String::new();
        screen.push('\n');
        for row in &self.pixels {
            for col in row {
                if *col {
                    screen.push('#');
                } else {
                    screen.push('.');
                }
            }
            screen.push('\n');
        }
        write!(f, "{}", screen)
    }
}

impl Screen {
    fn execute(&mut self, ins: &Instruction) {
        match ins {
            Instruction::Rect(cols, rows) => self.rect(*cols, *rows),
            Instruction::RotateCol(idx, amount) => self.rotate_col(*idx, *amount),
            Instruction::RotateRow(idx, amount) => self.rotate_row(*idx, *amount),
        }
    }

    fn rect(&mut self, cols: usize, rows: usize) {
        for row in 0..rows {
            for col in 0..cols {
                self.pixels[row][col] = true;
            }
        }
    }

    fn rotate_col(&mut self, idx: usize, amount: usize) {
        let old = self.pixels;
        let col: Result<[bool; ROWS], _> = old
            .iter()
            .map(|row| row[idx])
            .collect::<Vec<bool>>()
            .try_into();

        if let Ok(mut col) = col {
            col.rotate_right(amount);
            for (row_idx, row) in old.iter().enumerate() {
                for _on in row.iter() {
                    self.pixels[row_idx][idx] = col[row_idx];
                }
            }
        }
    }

    fn rotate_row(&mut self, idx: usize, amount: usize) {
        let row = self.pixels.get(idx).copied();
        if let Some(mut row) = row {
            row.rotate_right(amount);
            self.pixels[idx] = row;
        }
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let (kind, rest) = line
                .split_once(' ')
                .ok_or(AoCError::Parsing)?;
            let instruction = match kind {
                "rect" => {
                    let (cols, rows) = rest
                        .split_once('x')
                        .ok_or(AoCError::Parsing)?;
                    Instruction::Rect(cols.parse()?, rows.parse()?)
                }
                "rotate" => {
                    let (direction, rest) = rest
                        .split_once(' ')
                        .ok_or(AoCError::Parsing)?;
                    let (idx, rest) = rest
                        .split_once(' ')
                        .ok_or(AoCError::Parsing)?;
                    let (_, idx) = idx
                        .split_once('=')
                        .ok_or(AoCError::Parsing)?;
                    let (_, amount) = rest
                        .split_once(' ')
                        .ok_or(AoCError::Parsing)?;
                    let idx = idx.parse()?;
                    let amount = amount.parse()?;
                    match direction {
                        "row" => Instruction::RotateRow(idx, amount),
                        "column" => Instruction::RotateCol(idx, amount),
                        _ => return Err(AoCError::Parsing),
                    }
                }
                _ => return Err(AoCError::Parsing),
            };
            instructions.push(instruction);
        }
        Ok(Self { instructions })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut screen = Screen {
            pixels: [[false; COLS]; ROWS],
        };
        for instruction in &self.instructions {
            screen.execute(instruction);
        }

        let result = screen
            .pixels
            .into_iter()
            .flat_map(|row| row.into_iter())
            .filter(|on| *on)
            .count();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut screen = Screen {
            pixels: [[false; COLS]; ROWS],
        };
        for instruction in &self.instructions {
            screen.execute(instruction);
        }

        let mut letters = vec![];
        for letter_idx in 0..COLS / LETTER_WIDTH {
            let mut letter: Vec<&[bool]> = vec![];
            for col_idx in 0..ROWS {
                let line = &screen.pixels[col_idx]
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

    fn get_input() -> String {
        "rect 1x1
rotate row y=0 by 5
rect 1x1
rotate row y=0 by 6
rect 1x1
rotate row y=0 by 5
rect 1x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 5
rect 2x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 4
rect 1x1
rotate row y=0 by 3
rect 2x1
rotate row y=0 by 7
rect 3x1
rotate row y=0 by 3
rect 1x1
rotate row y=0 by 3
rect 1x2
rotate row y=1 by 13
rotate column x=0 by 1
rect 2x1
rotate row y=0 by 5
rotate column x=0 by 1
rect 3x1
rotate row y=0 by 18
rotate column x=13 by 1
rotate column x=7 by 2
rotate column x=2 by 3
rotate column x=0 by 1
rect 17x1
rotate row y=3 by 13
rotate row y=1 by 37
rotate row y=0 by 11
rotate column x=7 by 1
rotate column x=6 by 1
rotate column x=4 by 1
rotate column x=0 by 1
rect 10x1
rotate row y=2 by 37
rotate column x=19 by 2
rotate column x=9 by 2
rotate row y=3 by 5
rotate row y=2 by 1
rotate row y=1 by 4
rotate row y=0 by 4
rect 1x4
rotate column x=25 by 3
rotate row y=3 by 5
rotate row y=2 by 2
rotate row y=1 by 1
rotate row y=0 by 1
rect 1x5
rotate row y=2 by 10
rotate column x=39 by 1
rotate column x=35 by 1
rotate column x=29 by 1
rotate column x=19 by 1
rotate column x=7 by 2
rotate row y=4 by 22
rotate row y=3 by 5
rotate row y=1 by 21
rotate row y=0 by 10
rotate column x=2 by 2
rotate column x=0 by 2
rect 4x2
rotate column x=46 by 2
rotate column x=44 by 2
rotate column x=42 by 1
rotate column x=41 by 1
rotate column x=40 by 2
rotate column x=38 by 2
rotate column x=37 by 3
rotate column x=35 by 1
rotate column x=33 by 2
rotate column x=32 by 1
rotate column x=31 by 2
rotate column x=30 by 1
rotate column x=28 by 1
rotate column x=27 by 3
rotate column x=26 by 1
rotate column x=23 by 2
rotate column x=22 by 1
rotate column x=21 by 1
rotate column x=20 by 1
rotate column x=19 by 1
rotate column x=18 by 2
rotate column x=16 by 2
rotate column x=15 by 1
rotate column x=13 by 1
rotate column x=12 by 1
rotate column x=11 by 1
rotate column x=10 by 1
rotate column x=7 by 1
rotate column x=6 by 1
rotate column x=5 by 1
rotate column x=3 by 2
rotate column x=2 by 1
rotate column x=1 by 1
rotate column x=0 by 1
rect 49x1
rotate row y=2 by 34
rotate column x=44 by 1
rotate column x=40 by 2
rotate column x=39 by 1
rotate column x=35 by 4
rotate column x=34 by 1
rotate column x=30 by 4
rotate column x=29 by 1
rotate column x=24 by 1
rotate column x=15 by 4
rotate column x=14 by 1
rotate column x=13 by 3
rotate column x=10 by 4
rotate column x=9 by 1
rotate column x=5 by 4
rotate column x=4 by 3
rotate row y=5 by 20
rotate row y=4 by 20
rotate row y=3 by 48
rotate row y=2 by 20
rotate row y=1 by 41
rotate column x=47 by 5
rotate column x=46 by 5
rotate column x=45 by 4
rotate column x=43 by 5
rotate column x=41 by 5
rotate column x=33 by 1
rotate column x=32 by 3
rotate column x=23 by 5
rotate column x=22 by 1
rotate column x=21 by 2
rotate column x=18 by 2
rotate column x=17 by 3
rotate column x=16 by 2
rotate column x=13 by 5
rotate column x=12 by 5
rotate column x=11 by 5
rotate column x=3 by 5
rotate column x=2 by 5
rotate column x=1 by 5
"
        .to_string()
    }

    #[test]
    fn part_1() {
        let input = &get_input();
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "115");
    }

    #[test]
    fn part_2() {
        let input = &get_input();
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "EFEYKFRFIJ");
    }
}
