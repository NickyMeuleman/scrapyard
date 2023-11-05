use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Instruction>);

#[derive(Debug, Clone)]
enum Instruction {
    SwapPos(usize, usize),
    SwapLetter(char, char),
    RotateDir(Direction, usize),
    RotatePos(char), // always rotates to the right
    Reverse(usize, usize),
    Move(usize, usize),
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

fn scramble(instructions: &[Instruction], password: &mut Vec<char>) {
    for ins in instructions.iter() {
        match ins {
            Instruction::SwapPos(left, right) => {
                password.swap(*left, *right);
            }
            Instruction::SwapLetter(left, right) => {
                let left_idx = password
                    .iter()
                    .position(|c| c == left)
                    .expect("swap left");
                let right_idx = password
                    .iter()
                    .position(|c| c == right)
                    .expect("swap right");
                password.swap(left_idx, right_idx);
            }
            Instruction::RotateDir(dir, amount) => match dir {
                Direction::Left => {
                    password.rotate_left(*amount);
                }
                Direction::Right => {
                    password.rotate_right(*amount);
                }
            },
            Instruction::RotatePos(x) => {
                let idx = password
                    .iter()
                    .position(|c| c == x)
                    .expect("rotate pos index");
                let amount = 1 + idx + usize::from(idx >= 4);
                let amount = amount % password.len();
                password.rotate_right(amount);
            }
            Instruction::Reverse(left, right) => {
                let count = right - left + 1;
                for i in 0..count / 2 {
                    password.swap(left + i, right - i);
                }
            }
            Instruction::Move(left, right) => {
                let c = password.remove(*left);
                password.insert(*right, c);
            }
        }
    }
}

fn unscramble(instructions: &[Instruction], password: &mut Vec<char>) {
    // rotate based on position of letter X means that the whole string should be rotated to the right based on the index of letter X (counting from 0)
    // as determined before this instruction does any rotations. Once the index is determined,
    // rotate the string to the right one time,
    // plus a number of times equal to that index, plus one additional time if the index was at least 4.

    // calculated in forward step for "rotate based on": idx and rot R,
    // doing that instruction results in new_idx
    // undoing that instruction means going from new_idx to idx
    // calculate rot L to get back to the original idx from new_idx
    // idx  rot R   new_idx  rot L   idx
    // 0    1       1        1       0
    // 1    2       3        2       1
    // 2    3       5        3       2
    // 3    4       7        4       3
    // 4    6       2        6       4
    // 5    7       4        7       5
    // 6    8       6        8       6
    // 7    9       0        9       7
    // find equivalent rotations to undo the ones for the "rotate based on position of letter X"-rule
    // the rots vector stores the amount of times to rotate to the LEFT
    let mut rots = vec![0; password.len()];
    for idx in 0..password.len() {
        let amount = 1 + idx + usize::from(idx >= 4);
        let amount = amount % password.len();
        let new_idx = (idx + amount) % password.len();
        rots[new_idx] = amount;
    }

    for ins in instructions.iter().rev() {
        match ins {
            // same action to undo as to do
            Instruction::SwapPos(left, right) => {
                password.swap(*right, *left);
            }
            // same action to undo as to do
            Instruction::SwapLetter(left, right) => {
                let left_idx = password
                    .iter()
                    .position(|c| c == left)
                    .unwrap();
                let right_idx = password
                    .iter()
                    .position(|c| c == right)
                    .unwrap();
                password.swap(right_idx, left_idx);
            }
            // reverse direction to undo
            Instruction::RotateDir(dir, amount) => match dir {
                Direction::Left => {
                    password.rotate_right(*amount);
                }
                Direction::Right => {
                    password.rotate_left(*amount);
                }
            },
            // the tricky one to reverse
            Instruction::RotatePos(x) => {
                let idx = password
                    .iter()
                    .position(|c| c == x)
                    .unwrap();
                let amount = rots[idx];
                password.rotate_left(amount);
            }
            // same action to undo as to do
            Instruction::Reverse(left, right) => {
                let count = right - left + 1;
                for i in 0..count / 2 {
                    password.swap(left + i, right - i);
                }
            }
            // remove right, insert left to undo
            Instruction::Move(left, right) => {
                let c = password.remove(*right);
                password.insert(*left, c);
            }
        }
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut instructions = Vec::new();
        for line in input.trim().lines() {
            let (kind, rest) = line
                .split_once(' ')
                .ok_or(AoCError::Parsing)?;
            let instruction = match kind {
                "swap" => {
                    let (kind, rest) = rest
                        .split_once(' ')
                        .ok_or(AoCError::Parsing)?;
                    match kind {
                        "position" => {
                            let (left, right) = rest
                                .split_once(" with position ")
                                .ok_or(AoCError::Parsing)?;
                            let left = left.parse()?;
                            let right = right.parse()?;
                            Instruction::SwapPos(left, right)
                        }
                        "letter" => {
                            let (left, right) = rest
                                .split_once(" with letter ")
                                .ok_or(AoCError::Parsing)?;
                            let left = left
                                .chars()
                                .next()
                                .ok_or(AoCError::Parsing)?;
                            let right = right
                                .chars()
                                .next()
                                .ok_or(AoCError::Parsing)?;
                            Instruction::SwapLetter(left, right)
                        }
                        _ => return Err(AoCError::Parsing),
                    }
                }
                "rotate" => {
                    let (kind, rest) = rest
                        .split_once(' ')
                        .ok_or(AoCError::Parsing)?;
                    match kind {
                        "left" => {
                            let (amount, _) = rest
                                .split_once(' ')
                                .ok_or(AoCError::Parsing)?;
                            let amount = amount.parse()?;
                            Instruction::RotateDir(Direction::Left, amount)
                        }
                        "right" => {
                            let (amount, _) = rest
                                .split_once(' ')
                                .ok_or(AoCError::Parsing)?;
                            let amount = amount.parse()?;
                            Instruction::RotateDir(Direction::Right, amount)
                        }
                        "based" => {
                            let (_, letter) = rest
                                .split_once("on position of letter ")
                                .ok_or(AoCError::Parsing)?;
                            let letter = letter
                                .chars()
                                .next()
                                .ok_or(AoCError::Parsing)?;
                            Instruction::RotatePos(letter)
                        }
                        _ => return Err(AoCError::Parsing),
                    }
                }
                "reverse" => {
                    let rest = rest
                        .strip_prefix("positions ")
                        .ok_or(AoCError::Parsing)?;
                    let (left, right) = rest
                        .split_once(" through ")
                        .ok_or(AoCError::Parsing)?;
                    let left = left.parse()?;
                    let right = right.parse()?;
                    Instruction::Reverse(left, right)
                }
                "move" => {
                    let rest = rest
                        .strip_prefix("position ")
                        .ok_or(AoCError::Parsing)?;
                    let (left, right) = rest
                        .split_once(" to position ")
                        .ok_or(AoCError::Parsing)?;
                    let left = left.parse()?;
                    let right = right.parse()?;
                    Instruction::Move(left, right)
                }
                _ => return Err(AoCError::Parsing),
            };

            instructions.push(instruction);
        }
        Ok(Self(instructions))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut password = "abcdefgh".chars().collect();
        scramble(&self.0, &mut password);
        Ok(password.iter().collect::<String>())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut password = "fbgdceah".chars().collect();
        unscramble(&self.0, &mut password);
        Ok(password.iter().collect::<String>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";
        let data = Data::try_new(input).unwrap();
        let mut password: Vec<char> = "abcde".chars().collect();
        scramble(&data.0, &mut password);
        let password: String = password.iter().collect();
        assert_eq!(password, "decab");
    }

    #[test]
    fn part_2() {
        let input = "rotate right 4 steps
swap letter b with letter e
swap position 1 with position 3
reverse positions 0 through 4
rotate left 5 steps
swap position 6 with position 5
move position 3 to position 2
move position 6 to position 5
reverse positions 1 through 4
rotate based on position of letter e
reverse positions 3 through 7
reverse positions 4 through 7
rotate left 1 step
reverse positions 2 through 6
swap position 7 with position 5
swap letter e with letter c
swap letter f with letter d
swap letter a with letter e
swap position 2 with position 7
swap position 1 with position 7
swap position 6 with position 3
swap letter g with letter h
reverse positions 2 through 5
rotate based on position of letter f
rotate left 1 step
rotate right 2 steps
reverse positions 2 through 7
reverse positions 5 through 6
rotate left 6 steps
move position 2 to position 6
rotate based on position of letter a
rotate based on position of letter a
swap letter f with letter a
rotate right 5 steps
reverse positions 0 through 4
swap letter d with letter c
swap position 4 with position 7
swap letter f with letter h
swap letter h with letter a
rotate left 0 steps
rotate based on position of letter e
swap position 5 with position 4
swap letter e with letter h
swap letter h with letter d
rotate right 2 steps
rotate right 3 steps
swap position 1 with position 7
swap letter b with letter e
swap letter b with letter e
rotate based on position of letter e
rotate based on position of letter h
swap letter a with letter h
move position 7 to position 2
rotate left 2 steps
move position 3 to position 2
swap position 4 with position 6
rotate right 7 steps
reverse positions 1 through 4
move position 7 to position 0
move position 2 to position 0
reverse positions 4 through 6
rotate left 3 steps
rotate left 7 steps
move position 2 to position 3
rotate left 6 steps
swap letter a with letter h
rotate based on position of letter f
swap letter f with letter c
swap position 3 with position 0
reverse positions 1 through 3
swap letter h with letter a
swap letter b with letter a
reverse positions 2 through 3
rotate left 5 steps
swap position 7 with position 5
rotate based on position of letter g
rotate based on position of letter h
rotate right 6 steps
swap letter a with letter e
swap letter b with letter g
move position 4 to position 6
move position 6 to position 5
rotate based on position of letter e
reverse positions 2 through 6
swap letter c with letter f
swap letter h with letter g
move position 7 to position 2
reverse positions 1 through 7
reverse positions 1 through 2
rotate right 0 steps
move position 5 to position 6
swap letter f with letter a
move position 3 to position 1
move position 2 to position 4
reverse positions 1 through 2
swap letter g with letter c
rotate based on position of letter f
rotate left 7 steps
rotate based on position of letter e
swap position 6 with position 1
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "aghfcdeb");
    }
}
