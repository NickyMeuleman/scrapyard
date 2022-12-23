use crate::AoCData;

pub struct Data {
    map: Vec<Vec<Tile>>,
    instructions: Vec<Instruction>,
}

#[derive(Clone)]
struct Coord {
    row: i32,
    col: i32,
}

enum Direction {
    L,
    R,
    U,
    D,
}

enum Turn {
    L,
    R,
}

#[derive(PartialEq)]
enum Tile {
    Open,
    Solid,
    None,
}

enum Instruction {
    Rotate(Turn),
    Forward(u8),
}

impl Direction {
    fn score(&self) -> usize {
        use Direction::*;
        match self {
            R => 0,
            D => 1,
            L => 2,
            U => 3,
        }
    }

    fn turn(self, turn: &Turn) -> Direction {
        use Direction::*;
        match (self, turn) {
            (L, Turn::L) => D,
            (L, Turn::R) => U,
            (R, Turn::L) => U,
            (R, Turn::R) => D,
            (U, Turn::L) => L,
            (U, Turn::R) => R,
            (D, Turn::L) => R,
            (D, Turn::R) => L,
        }
    }

    fn offset(&self) -> Coord {
        use Direction::*;
        match &self {
            L => Coord { row: 0, col: -1 },
            R => Coord { row: 0, col: 1 },
            U => Coord { row: -1, col: 0 },
            D => Coord { row: 1, col: 0 },
        }
    }
}

fn wrap1(map: &[Vec<Tile>], pos: &Coord, dir: &Direction) -> Coord {
    let Coord { row: dr, col: dc } = dir.offset();
    let mut curr = pos.clone();
    // while an open or solid tile exists in the map when walking in the opposite direction, update pos
    while let Some(tile) = map
        .get((curr.row - dr) as usize)
        .and_then(|row| row.get((curr.col - dc) as usize))
    {
        if *tile == Tile::None {
            break;
        }
        curr = Coord {
            row: curr.row - dr,
            col: curr.col - dc,
        };
    }

    curr
}

fn wrap2(pos: &Coord, dir: &Direction) -> (Coord, Direction) {
    // find idxes of entire cube
    // this huge match statement only covers cases in the real input, but can be expanded to cover everything. It's just tedious
    let (cube_row, cube_col, new_dir) = match (pos.row / 50, pos.col / 50, dir) {
        (0, 1, Direction::U) => (3, 0, Direction::R),
        (0, 1, Direction::L) => (2, 0, Direction::R),
        (0, 2, Direction::U) => (3, 0, Direction::U),
        (0, 2, Direction::R) => (2, 1, Direction::L),
        (0, 2, Direction::D) => (1, 1, Direction::L),
        (1, 1, Direction::R) => (0, 2, Direction::U),
        (1, 1, Direction::L) => (2, 0, Direction::D),
        (2, 0, Direction::U) => (1, 1, Direction::R),
        (2, 0, Direction::L) => (0, 1, Direction::R),
        (2, 1, Direction::R) => (0, 2, Direction::L),
        (2, 1, Direction::D) => (3, 0, Direction::L),
        (3, 0, Direction::R) => (2, 1, Direction::U),
        (3, 0, Direction::D) => (0, 2, Direction::D),
        (3, 0, Direction::L) => (0, 1, Direction::D),
        _ => unreachable!(),
    };
    // find idxes within the cube
    let (row_idx, col_idx) = (pos.row % 50, pos.col % 50);

    let i = match dir {
        Direction::L => 49 - row_idx,
        Direction::R => row_idx,
        Direction::U => col_idx,
        Direction::D => 49 - col_idx,
    };

    // find new idxes within the cube
    let new_row = match new_dir {
        Direction::L => 49 - i,
        Direction::R => i,
        Direction::U => 49,
        Direction::D => 0,
    };
    let new_col = match new_dir {
        Direction::L => 49,
        Direction::R => 0,
        Direction::U => i,
        Direction::D => 49 - i,
    };

    let new_pos = Coord {
        row: cube_row * 50 + new_row,
        col: cube_col * 50 + new_col,
    };

    (new_pos, new_dir)
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> Option<Self> {
        // do NOT remove starting whitespace, it's significant
        let (grid, moves) = input.trim_end().split_once("\n\n")?;
        let mut instructions = Vec::new();
        let mut digits = Vec::new();
        for c in moves.chars() {
            if c.is_numeric() {
                // accumulate digits
                let digit = c.to_digit(10)? as u8;
                digits.push(digit);
            } else {
                // construct number out of digits
                // uses math to concatenate digits instead of turning them into a string first and parsing the string
                let num = digits.iter().fold(0, |num, digit| num * 10 + digit);
                digits.clear();
                instructions.push(Instruction::Forward(num));

                // parse turn
                let turn = match c {
                    'L' => Turn::L,
                    'R' => Turn::R,
                    _ => return None,
                };
                instructions.push(Instruction::Rotate(turn));
            }
        }
        // construct number out of any remaining digits
        // uses math to concatenate digits instead of turning them into a string first and parsing the string
        let num = digits.iter().fold(0, |num, digit| num * 10 + digit);
        instructions.push(Instruction::Forward(num));

        let mut map = Vec::new();
        for line in grid.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let tile = match c {
                    '.' => Tile::Open,
                    '#' => Tile::Solid,
                    ' ' => Tile::None,
                    _ => return None,
                };
                row.push(tile);
            }
            map.push(row);
        }

        Some(Self { map, instructions })
    }

    fn part_1(&self) -> String {
        // go to the first open position on the top row (skip the Nones)
        let start_col = self.map[0]
            .iter()
            .position(|tile| *tile == Tile::Open)
            .unwrap() as i32;

        let mut pos = Coord {
            row: 0,
            col: start_col,
        };
        let mut dir = Direction::R;

        for inst in &self.instructions {
            match inst {
                Instruction::Rotate(turn) => dir = dir.turn(turn),
                Instruction::Forward(amount) => {
                    // take a step "amount" times
                    for _ in 0..*amount {
                        let Coord { row: dr, col: dc } = dir.offset();
                        let new_tile = self
                            .map
                            .get((pos.row + dr) as usize)
                            .and_then(|row| row.get((pos.col + dc) as usize))
                            .unwrap_or(&Tile::None);

                        match new_tile {
                            // if new tile is solid, stop moving
                            Tile::Solid => break,
                            // if new tile is open, move there
                            Tile::Open => {
                                pos = Coord {
                                    row: pos.row + dr,
                                    col: pos.col + dc,
                                };
                            }
                            // if new tile is not found, wrap around
                            Tile::None => {
                                let new_pos = wrap1(&self.map, &pos, &dir);
                                // if the new_pos is solid, stop moving
                                if self.map[new_pos.row as usize][new_pos.col as usize]
                                    == Tile::Solid
                                {
                                    break;
                                }
                                // if the new_pos is open, move there
                                pos = new_pos;
                            }
                        }
                    }
                }
            }
        }

        let password = 1000 * (pos.row + 1) + 4 * (pos.col + 1) + dir.score() as i32;
        password.to_string()
    }

    fn part_2(&self) -> String {
        // go to the first open position on the top row (skip the Nones)
        let start_col = self.map[0]
            .iter()
            .position(|tile| *tile == Tile::Open)
            .unwrap() as i32;

        let mut pos = Coord {
            row: 0,
            col: start_col,
        };
        let mut dir = Direction::R;

        for inst in &self.instructions {
            match inst {
                Instruction::Rotate(turn) => dir = dir.turn(turn),
                Instruction::Forward(amount) => {
                    // take a step "amount" times
                    for _ in 0..*amount {
                        let Coord { row: dr, col: dc } = dir.offset();
                        let new_tile = self
                            .map
                            .get((pos.row + dr) as usize)
                            .and_then(|row| row.get((pos.col + dc) as usize))
                            .unwrap_or(&Tile::None);

                        match new_tile {
                            // if new tile is solid, stop moving
                            Tile::Solid => break,
                            // if new tile is open, move there
                            Tile::Open => {
                                pos = Coord {
                                    row: pos.row + dr,
                                    col: pos.col + dc,
                                };
                            }
                            // if new tile is not found, wrap around
                            Tile::None => {
                                let (new_pos, new_dir) = wrap2(&pos, &dir);
                                // if the new_pos is solid, stop moving
                                if self.map[new_pos.row as usize][new_pos.col as usize]
                                    == Tile::Solid
                                {
                                    break;
                                }
                                // if the new_pos is open, move there
                                pos = new_pos;
                                dir = new_dir
                            }
                        }
                    }
                }
            }
        }

        let password = 1000 * (pos.row + 1) + 4 * (pos.col + 1) + dir.score() as i32;
        password.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(22);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "6032");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(22);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_2(), "144361");
    }
}
