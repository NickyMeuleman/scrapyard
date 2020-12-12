use std::fs;

fn main() {
    // make ship struct with: location, face degrees, move implementation
    // ship always turns in 90 degree increments
    let input = fs::read_to_string("./input.txt").unwrap();
    let instructions: Vec<(char, usize)> = parse(&input);
    println!("Part one answer: {}", part_one(&instructions));
    println!("Part two answer: {}", part_two(&instructions));
}

fn parse(input: &str) -> Vec<(char, usize)> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(input: &str) -> (char, usize) {
    let letter = input[0..1].parse().unwrap();
    let number: usize = input[1..].parse().unwrap();
    (letter, number)
}

#[derive(Debug)]
struct Ship {
    // The ship starts by facing east
    // direction in degrees, 0 for due east, 90 for due north, 180 for due west, and 270 for due south
    // pos first is N, neg first is S, pos second is E, neg second is W
    position: (isize, isize),
    direction: usize,
}

impl Ship {
    fn action(&mut self, instruction: (char, usize)) {
        match instruction {
            ('N', num) => self.change_pos('N', num),
            ('S', num) => self.change_pos('S', num),
            ('E', num) => self.change_pos('E', num),
            ('W', num) => self.change_pos('W', num),
            ('L', num) => self.change_dir('L', num),
            ('R', num) => self.change_dir('R', num),
            ('F', num) => self.change_pos('F', num),
            _ => panic!("Invalid instruction given to ship"),
        }
    }
    fn change_pos(&mut self, dir: char, amount: usize) {
        match dir {
            'F' => {
                let new_dir = num_to_dir(self.direction);
                self.change_pos(new_dir, amount)
            }
            'N' => self.position.0 = self.position.0 + amount as isize,
            'S' => self.position.0 = self.position.0 - amount as isize,
            'E' => self.position.1 = self.position.1 + amount as isize,
            'W' => self.position.1 = self.position.1 - amount as isize,
            _ => panic!("tried to move ship in invalid direction"),
        }
    }
    fn change_dir(&mut self, dir: char, amount: usize) {
        // TODO: math with modulo to make this more elegant
        match dir {
            'L' => {
                let mut new_dir = self.direction + amount;
                while new_dir >= 360 {
                    new_dir -= 360;
                }
                self.direction = new_dir;
            }
            'R' => {
                let mut new_dir = self.direction as isize - amount as isize;
                while new_dir < 0 {
                    new_dir += 360;
                }
                self.direction = new_dir as usize;
            }

            _ => panic!("tried to turn ship in invalid direction"),
        }
    }
}

fn num_to_dir(num: usize) -> char {
    match num {
        0 => 'E',
        90 => 'N',
        180 => 'W',
        270 => 'S',
        _ => panic!("new direction was not an absolute wind direction"),
    }
}

fn part_one(instructions: &Vec<(char, usize)>) -> usize {
    let mut ship = Ship {
        position: (0, 0),
        direction: 0,
    };
    for &instruction in instructions {
        dbg!(&instruction, &ship);
        ship.action(instruction);
    }
    manhattan_distance(
        0,
        0,
        ship.position.0.abs() as usize,
        ship.position.1.abs() as usize,
    )
}

fn part_two(instructions: &Vec<(char, usize)>) -> usize {
    1
}

fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let x_diff = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let y_diff = if y1 < y2 { y2 - y1 } else { y1 - y2 };
    x_diff + y_diff
}
