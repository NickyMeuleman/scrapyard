use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Data {
    points: HashSet<Point>,
    instructions: VecDeque<Instruction>,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.points.iter().map(|p| p.x).max().unwrap();
        let height = self.points.iter().map(|p| p.y).max().unwrap();
        let mut result = String::new();
        for y in 0..=height {
            let mut line = String::new();
            for x in 0..=width {
                if self.points.contains(&Point { x, y }) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            line.push_str("\n");
            result.push_str(&line);
        }
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn mirror(&self, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::X(x) => Point {
                x: self.x - (2 * (self.x - x)),
                y: self.y,
            },
            Instruction::Y(y) => Point {
                x: self.x,
                y: self.y - (2 * (self.y - y)),
            },
        }
    }

    fn should_mirror(&self, f: &Instruction) -> bool {
        match f {
            Instruction::Y(y) => self.y > *y,
            Instruction::X(x) => self.x > *x,
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    X(u16),
    Y(u16),
}

impl Data {
    pub fn part_one(&self) -> usize {
        let mut data = self.clone();
        data.fold();
        data.points.len()
    }

    pub fn part_two(&self) -> String {
        let mut data = self.clone();
        while !data.instructions.is_empty() {
            data.fold();
        }

        let width = data.points.iter().map(|p| p.x).max().unwrap();
        let height = data.points.iter().map(|p| p.y).max().unwrap();

        let mut result = String::new();
        // character is 5 wide and 6 high, there are 8 of them
        let mut letters: [[[char; 5]; 6]; 8] = [[['.'; 5]; 6]; 8];
        for y in 0..=height {
            for x in 0..=width {
                let letter = if x == 0 { 0 } else { x as usize / 5 };
                let col = (x % 5) as usize;
                let row = y as usize;
                // push the correct thing into the correct letter
                letters[letter][row][col] = match data.points.contains(&Point { x, y }) {
                    true => '#',
                    false => '.',
                };
            }
        }

        for letter in letters {
            // recognize letter and push it into result
            // I'm so sorry for this, LONG LINES, YIHAAAAAAAAAAAAAA
            // only recognizing the letters in my solution at the moment, will have to extend to recognize ALL letters
            let recognized = match letter {
                [['.', '#', '#', '.', '.'], ['#', '.', '.', '#', '.'], ['#', '.', '.', '.', '.'], ['#', '.', '.', '.', '.'], ['#', '.', '.', '#', '.'], ['.', '#', '#', '.', '.']] => {
                    'C'
                }
                [['.', '#', '#', '.', '.'], ['#', '.', '.', '#', '.'], ['#', '.', '.', '#', '.'], ['#', '#', '#', '#', '.'], ['#', '.', '.', '#', '.'], ['#', '.', '.', '#', '.']] => {
                    'A'
                }
                [['#', '#', '#', '#', '.'], ['#', '.', '.', '.', '.'], ['#', '#', '#', '.', '.'], ['#', '.', '.', '.', '.'], ['#', '.', '.', '.', '.'], ['#', '.', '.', '.', '.']] => {
                    'F'
                }
                [['.', '.', '#', '#', '.'], ['.', '.', '.', '#', '.'], ['.', '.', '.', '#', '.'], ['.', '.', '.', '#', '.'], ['#', '.', '.', '#', '.'], ['.', '#', '#', '.', '.']] => {
                    'J'
                }
                [['#', '.', '.', '#', '.'], ['#', '.', '.', '#', '.'], ['#', '#', '#', '#', '.'], ['#', '.', '.', '#', '.'], ['#', '.', '.', '#', '.'], ['#', '.', '.', '#', '.']] => {
                    'H'
                }
                [['#', '#', '#', '#', '.'], ['.', '.', '.', '#', '.'], ['.', '.', '#', '.', '.'], ['.', '#', '.', '.', '.'], ['#', '.', '.', '.', '.'], ['#', '#', '#', '#', '.']] => {
                    'Z'
                }
                [['#', '.', '.', '#', '.'], ['#', '.', '#', '.', '.'], ['#', '#', '.', '.', '.'], ['#', '.', '#', '.', '.'], ['#', '.', '#', '.', '.'], ['#', '.', '.', '#', '.']] => {
                    'K'
                }
                _ => '?',
            };
            result.push(recognized);
        }

        result
    }

    fn fold(&mut self) {
        // can only fold if there is an instruction left
        if let Some(instruction) = self.instructions.pop_front() {
            self.points = self
                .points
                .iter()
                .map(|point| {
                    if point.should_mirror(&instruction) {
                        point.mirror(&instruction)
                    } else {
                        point.clone()
                    }
                })
                .collect();
        }
    }
}

impl FromStr for Data {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (points, instructions) = input.trim().split_once("\n\n").unwrap();
        let points = points
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();
                Point { x, y }
            })
            .collect();
        let instructions = instructions
            .lines()
            .map(|line| {
                let (_, instruction) = line.rsplit_once(' ').unwrap();
                match instruction.split_once('=').unwrap() {
                    ("x", num) => Instruction::X(num.parse().unwrap()),
                    ("y", num) => Instruction::Y(num.parse().unwrap()),
                    _ => panic!("Invalid input"),
                }
            })
            .collect();

        Ok(Self {
            points,
            instructions,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_tiny() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 17);
    }
}
