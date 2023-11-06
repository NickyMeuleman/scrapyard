use std::{collections::HashMap, fmt::Display};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(HashMap<Coord, char>);

enum Turn {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coord {
    row: i64,
    col: i64,
}

impl Coord {
    fn turn(&mut self, dir: &Turn) {
        match dir {
            Turn::Left => {
                let temp = self.col;
                self.col = self.row;
                self.row = -temp;
            }
            Turn::Right => {
                let temp = self.col;
                self.col = -self.row;
                self.row = temp;
            }
        }
    }
    fn forward(&mut self, offset: &Self) {
        self.row += offset.row;
        self.col += offset.col;
    }
    fn backward(&mut self, offset: &Self) {
        self.row -= offset.row;
        self.col -= offset.col;
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut map = HashMap::new();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let coord = Coord {
                    row: row as i64,
                    col: col as i64,
                };
                if c != ' ' {
                    map.insert(coord, c);
                }
            }
        }

        Ok(Self(map))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let map = &self.0;
        let mut coord = *map
            .iter()
            .find(|(coord, c)| coord.row == 0 && **c == '|')
            .unwrap()
            .0;
        let mut dir = Coord { row: 1, col: 0 };
        let mut seen = String::new();
        loop {
            while let Some(c) = map.get(&coord) {
                if c.is_alphabetic() {
                    seen.push(*c);
                }
                coord.forward(&dir);
            }
            coord.backward(&dir);

            // try left
            let mut left_offset = dir;
            left_offset.turn(&Turn::Left);
            let mut left_coord = coord;
            left_coord.forward(&left_offset);
            if map.contains_key(&left_coord) {
                dir = left_offset;
                continue;
            }
            // try right
            let mut right_offset = dir;
            right_offset.turn(&Turn::Right);
            let mut right_coord = coord;
            right_coord.forward(&right_offset);
            if map.contains_key(&right_coord) {
                dir = right_offset;
                continue;
            }
            break;
        }

        Ok(seen)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let map = &self.0;
        let mut coord = *map
            .iter()
            .find(|(coord, c)| coord.row == 0 && **c == '|')
            .unwrap()
            .0;
        let mut dir = Coord { row: 1, col: 0 };
        let mut steps = 1;
        loop {
            while map.contains_key(&coord) {
                coord.forward(&dir);
                steps += 1;
            }
            coord.backward(&dir);
            steps -= 1;
            // try left
            let mut left_offset = dir;
            left_offset.turn(&Turn::Left);
            let mut left_coord = coord;
            left_coord.forward(&left_offset);
            if map.contains_key(&left_coord) {
                dir = left_offset;
                continue;
            }
            // try right
            let mut right_offset = dir;
            right_offset.turn(&Turn::Right);
            let mut right_coord = coord;
            right_coord.forward(&right_offset);
            if map.contains_key(&right_coord) {
                dir = right_offset;
                continue;
            }
            break;
        }

        Ok(steps)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "ABCDEF");
    }

    #[test]
    fn part_2() {
        let input = "     |          
     |  +--+    
     A  |  C    
 F---|--|-E---+ 
     |  |  |  D 
     +B-+  +--+ 
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "38");
    }
}
