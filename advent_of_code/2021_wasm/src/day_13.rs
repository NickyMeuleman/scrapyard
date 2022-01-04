use crate::AoCData;
use hashbrown::HashSet;

pub struct Data {
    points: HashSet<Point>,
    instructions: Vec<Fold>,
}

enum Fold {
    X(u16),
    Y(u16),
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn mirror(&self, instruction: &Fold) -> Self {
        match instruction {
            Fold::X(x) => Point {
                x: self.x - (2 * (self.x - x)),
                y: self.y,
            },
            Fold::Y(y) => Point {
                x: self.x,
                y: self.y - (2 * (self.y - y)),
            },
        }
    }

    fn should_mirror(&self, f: &Fold) -> bool {
        match f {
            Fold::Y(y) => self.y > *y,
            Fold::X(x) => self.x > *x,
        }
    }
}

fn fold_grid(grid: &HashSet<Point>, instruction: &Fold) -> HashSet<Point> {
    grid.iter()
        .map(|point| {
            if point.should_mirror(&instruction) {
                point.mirror(&instruction)
            } else {
                *point
            }
        })
        .collect()
}

// fn print_grid(points: &HashSet<Point>) {
//     let width = points.iter().map(|p| p.x).max().unwrap();
//     let height = points.iter().map(|p| p.y).max().unwrap();
//     for y in 0..=height {
//         for x in 0..=width {
//             print!(
//                 "{}",
//                 if points.contains(&Point { x, y }) {
//                     '█'
//                 } else {
//                     ' '
//                 }
//             );
//         }
//         println!();
//     }
// }

impl AoCData for Data {
    fn new(input: String) -> Self {
        let (points, instructions) = input.trim().split_once("\n\n").unwrap();
        let points = points
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();
                Point {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect();
        let instructions = instructions
            .lines()
            .map(|line| {
                let (_, instruction) = line.rsplit_once(' ').unwrap();
                match instruction.split_once('=').unwrap() {
                    ("x", num) => Fold::X(num.parse().unwrap()),
                    ("y", num) => Fold::Y(num.parse().unwrap()),
                    _ => panic!("Invalid input"),
                }
            })
            .collect();

        Self {
            points,
            instructions,
        }
    }

    fn part_1(&self) -> String {
        fold_grid(&self.points, &self.instructions[0])
            .len()
            .to_string()
    }

    fn part_2(&self) -> String {
        let points = self
            .instructions
            .iter()
            .fold(self.points.clone(), |acc, instruction| {
                fold_grid(&acc, instruction)
            });

        // print answer to the console:
        // print_grid(&points);

        // extra code to do character recognition, because I want a neat String of English alphabet characters as result, not some ASCII-art
        let width = points.iter().map(|p| p.x).max().unwrap();
        let height = points.iter().map(|p| p.y).max().unwrap();
        const CHAR_WIDTH: usize = 4;
        const CHAR_HEIGHT: usize = 6;
        const NUM_CHARS: usize = 8;
        const GAP: usize = 1;
        let mut letters: [[[char; CHAR_WIDTH]; CHAR_HEIGHT]; NUM_CHARS] =
            [[[' '; CHAR_WIDTH]; CHAR_HEIGHT]; NUM_CHARS];

        for y in 0..=height {
            // there is 1 space gap between characters
            // that means the width is NUM_CHARS - 1 less (no gap after the last one)
            for x in 0..=width {
                let letter = x as usize / (CHAR_WIDTH + GAP);
                // as many gaps as letter + 1
                let col = x as usize % (CHAR_WIDTH + GAP);
                let row = y as usize;
                if col == CHAR_WIDTH {
                    // this is the gap between characters, disregard
                    continue;
                }
                // push the correct thing into the correct letter
                letters[letter][row][col] = match points.contains(&Point { x, y }) {
                    true => '█',
                    false => ' ',
                };
            }
        }

        letters.iter().fold(String::new(), |mut acc, letter| {
            #[rustfmt::skip]
            let recognized = match letter {
                [
                    [' ', '█', '█', ' '],
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', ' ', '█'],
                    ['█', '█', '█', '█'],
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', ' ', '█']
                ] => { 'A' }
                [
                    ['█', '█', '█', ' '],
                    ['█', ' ', ' ', '█'],
                    ['█', '█', '█', ' '],
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', ' ', '█'],
                    ['█', '█', '█', ' ']
                ] => { 'B' }
                [
                    [' ', '█', '█', ' '],
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', ' ', ' '],
                    ['█', ' ', ' ', ' '],
                    ['█', ' ', ' ', '█'],
                    [' ', '█', '█', ' ']
                ] => { 'C' }
                [
                    ['█', '█', '█', '█'],
                    ['█', ' ', ' ', ' '],
                    ['█', '█', '█', ' '],
                    ['█', ' ', ' ', ' '],
                    ['█', ' ', ' ', ' '],
                    ['█', '█', '█', '█']
                ] => { 'E' }
                [
                    ['█', '█', '█', '█'],
                    ['█', ' ', ' ', ' '],
                    ['█', '█', '█', ' '],
                    ['█', ' ', ' ', ' '],
                    ['█', ' ', ' ', ' '],
                    ['█', ' ', ' ', ' ']
                ] => { 'F' }
                [
                    [' ', '█', '█', ' '],
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', ' ', ' '],
                    ['█', ' ', '█', '█'],
                    ['█', ' ', ' ', '█'],
                    [' ', '█', '█', '█']
                ] => { 'G' }
                [
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', ' ', '█'],
                    ['█', '█', '█', '█'],
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', ' ', '█']
                ] => { 'H' }
                [
                    [' ', ' ', '█', '█'],
                    [' ', ' ', ' ', '█'],
                    [' ', ' ', ' ', '█'],
                    [' ', ' ', ' ', '█'],
                    ['█', ' ', ' ', '█'],
                    [' ', '█', '█', ' ']
                ] => { 'J' }
                [
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', '█', ' '],
                    ['█', '█', ' ', ' '],
                    ['█', ' ', '█', ' '],
                    ['█', ' ', '█', ' '],
                    ['█', ' ', ' ', '█']
                ] => { 'K' }
                [
                    ['█', ' ', ' ', ' '],
                    ['█', ' ', ' ', ' '],
                    ['█', ' ', ' ', ' '],
                    ['█', ' ', ' ', ' '],
                    ['█', ' ', ' ', ' '],
                    ['█', '█', '█', '█']
                ] => { 'L' }
                [
                    ['█', '█', '█', ' '],
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', ' ', '█'],
                    ['█', '█', '█', ' '],
                    ['█', ' ', ' ', ' '],
                    ['█', ' ', ' ', ' ']
                ] => { 'P' }
                [
                    ['█', '█', '█', ' '],
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', ' ', '█'],
                    ['█', '█', '█', ' '],
                    ['█', ' ', '█', ' '],
                    ['█', ' ', ' ', '█']
                ] => { 'R' }
                [
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', ' ', '█'],
                    ['█', ' ', ' ', '█'],
                    [' ', '█', '█', ' ']
                ] => { 'U' }
                [
                    ['█', '█', '█', '█'],
                    [' ', ' ', ' ', '█'],
                    [' ', ' ', '█', ' '],
                    [' ', '█', ' ', ' '],
                    ['█', ' ', ' ', ' '],
                    ['█', '█', '█', '█']
                ] => { 'Z' }
                _ => '?',
                };
            acc.push(recognized);
            acc
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(13);
        let data = Data::new(input);
        assert_eq!(data.part_1(), "17");
    }
}
