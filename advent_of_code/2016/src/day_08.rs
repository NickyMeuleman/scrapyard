use std::fmt;

use crate::AoCData;

const LETTER_WIDTH: usize = 5;
const ROWS: usize = 6;
const COLS: usize = 50;

pub struct Data {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
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

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let (kind, rest) = line.split_once(' ')?;
            let instruction = match kind {
                "rect" => {
                    let (cols, rows) = rest.split_once('x')?;
                    Instruction::Rect(cols.parse().ok()?, rows.parse().ok()?)
                }
                "rotate" => {
                    let (direction, rest) = rest.split_once(' ')?;
                    let (idx, rest) = rest.split_once(' ')?;
                    let (_, idx) = idx.split_once('=')?;
                    let (_, amount) = rest.split_once(' ')?;
                    let idx = idx.parse().ok()?;
                    let amount = amount.parse().ok()?;
                    match direction {
                        "row" => Instruction::RotateRow(idx, amount),
                        "column" => Instruction::RotateCol(idx, amount),
                        _ => return None,
                    }
                }
                _ => return None,
            };
            instructions.push(instruction);
        }
        Some(Self { instructions })
    }

    fn part_1(&self) -> String {
        let mut screen = Screen {
            pixels: [[false; COLS]; ROWS],
        };
        for instruction in &self.instructions {
            screen.execute(instruction);
        }

        screen
            .pixels
            .into_iter()
            .flat_map(|row| row.into_iter())
            .filter(|on| *on)
            .count()
            .to_string()
    }

    fn part_2(&self) -> String {
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

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_input(8);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "115");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(8);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "EFEYKFRFIJ");
    }
}
