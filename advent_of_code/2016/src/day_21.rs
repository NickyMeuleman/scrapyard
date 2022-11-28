use crate::AoCData;

pub struct Data(Vec<Instruction>);

#[derive(Debug)]
enum Instruction {
    SwapPos(usize, usize),
    SwapLetter(char, char),
    RotateDir(Direction, usize),
    RotatePos(char), // always rotates to the right
    Reverse(usize, usize),
    Move(usize, usize),
}

#[derive(Debug)]
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
                let left_idx = password.iter().position(|c| c == left).expect("swap left");
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
                let left_idx = password.iter().position(|c| c == left).unwrap();
                let right_idx = password.iter().position(|c| c == right).unwrap();
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
                let idx = password.iter().position(|c| c == x).unwrap();
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

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut instructions = Vec::new();
        for line in input.trim().lines() {
            let (kind, rest) = line.split_once(' ')?;
            let instruction = match kind {
                "swap" => {
                    let (kind, rest) = rest.split_once(' ')?;
                    match kind {
                        "position" => {
                            let (left, right) = rest.split_once(" with position ")?;
                            let left = left.parse().ok()?;
                            let right = right.parse().ok()?;
                            Instruction::SwapPos(left, right)
                        }
                        "letter" => {
                            let (left, right) = rest.split_once(" with letter ")?;
                            let left = left.chars().next()?;
                            let right = right.chars().next()?;
                            Instruction::SwapLetter(left, right)
                        }
                        _ => return None,
                    }
                }
                "rotate" => {
                    let (kind, rest) = rest.split_once(' ')?;
                    match kind {
                        "left" => {
                            let (amount, _) = rest.split_once(' ')?;
                            let amount = amount.parse().ok()?;
                            Instruction::RotateDir(Direction::Left, amount)
                        }
                        "right" => {
                            let (amount, _) = rest.split_once(' ')?;
                            let amount = amount.parse().ok()?;
                            Instruction::RotateDir(Direction::Right, amount)
                        }
                        "based" => {
                            let (_, letter) = rest.split_once("on position of letter ")?;
                            let letter = letter.chars().next()?;
                            Instruction::RotatePos(letter)
                        }
                        _ => return None,
                    }
                }
                "reverse" => {
                    let rest = rest.strip_prefix("positions ")?;
                    let (left, right) = rest.split_once(" through ")?;
                    let left = left.parse().ok()?;
                    let right = right.parse().ok()?;
                    Instruction::Reverse(left, right)
                }
                "move" => {
                    let rest = rest.strip_prefix("position ")?;
                    let (left, right) = rest.split_once(" to position ")?;
                    let left = left.parse().ok()?;
                    let right = right.parse().ok()?;
                    Instruction::Move(left, right)
                }
                _ => return None,
            };

            instructions.push(instruction);
        }
        Some(Self(instructions))
    }

    fn part_1(&self) -> String {
        let mut password = "abcdefgh".chars().collect();
        scramble(&self.0, &mut password);
        password.iter().collect()
    }

    fn part_2(&self) -> String {
        let mut password = "fbgdceah".chars().collect();
        unscramble(&self.0, &mut password);
        password.iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(21);
        let data = Data::try_new(input).unwrap();
        let mut password: Vec<char> = "abcde".chars().collect();
        scramble(&data.0, &mut password);
        let password: String = password.iter().collect();
        assert_eq!(password, "decab");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(21);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "aghfcdeb");
    }
}
