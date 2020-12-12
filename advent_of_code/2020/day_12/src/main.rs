use std::fs;

// TODO: remove duplication

fn main() {
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
    // tup.0 is N-S position
    // N is positive, S is negative
    // tup.1 is E-W position
    // E is positive, W is negative
    let mut ship = Ship2 {
        position: (0, 0),
        waypoint: (1, 10),
    };
    for &instruction in instructions {
        ship.action(instruction);
    }
    manhattan_distance(
        0,
        0,
        ship.position.0.abs() as usize,
        ship.position.1.abs() as usize,
    )
}

fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let x_diff = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let y_diff = if y1 < y2 { y2 - y1 } else { y1 - y2 };
    x_diff + y_diff
}

#[derive(Debug)]
struct Ship2 {
    position: (isize, isize),
    waypoint: (isize, isize),
}

impl Ship2 {
    fn action(&mut self, instruction: (char, usize)) {
        match instruction {
            ('N', _) => self.move_waypoint(instruction),
            ('S', _) => self.move_waypoint(instruction),
            ('E', _) => self.move_waypoint(instruction),
            ('W', _) => self.move_waypoint(instruction),
            ('L', _) => self.rotate_waypoint(instruction),
            ('R', _) => self.rotate_waypoint(instruction),
            ('F', num) => self.travel(num),
            _ => panic!("Invalid instruction given to ship"),
        }
    }
    fn travel(&mut self, amount: usize) {
        // move to waypoint amount of times
        self.position.0 += amount as isize * self.waypoint.0;
        self.position.1 += amount as isize * self.waypoint.1;
    }
    fn move_waypoint(&mut self, instruction: (char, usize)) {
        match instruction.0 {
            'N' => self.waypoint.0 += instruction.1 as isize,
            'S' => self.waypoint.0 -= instruction.1 as isize,
            'E' => self.waypoint.1 += instruction.1 as isize,
            'W' => self.waypoint.1 -= instruction.1 as isize,
            _ => panic!("tried to move waypoint in invalid direction"),
        }
    }
    fn rotate_waypoint(&mut self, instruction: (char, usize)) {
        match instruction.0 {
            'L' => {
                match instruction.1 {
                    90 => {
                        self.waypoint = (self.waypoint.1, -self.waypoint.0);
                    }
                    270 => {
                        self.waypoint = (-self.waypoint.1, self.waypoint.0);
                    }
                    180 => {
                        self.waypoint = (-self.waypoint.0, -self.waypoint.1);
                    }
                    _ => panic!("tried to rotate waypoint invalid amount of degrees"),
                };
            }
            'R' => {
                match instruction.1 {
                    90 => {
                        self.waypoint = (-self.waypoint.1, self.waypoint.0);
                    }
                    270 => {
                        self.waypoint = (self.waypoint.1, -self.waypoint.0);
                    }
                    180 => {
                        self.waypoint = (-self.waypoint.0, -self.waypoint.1);
                    }
                    _ => panic!("tried to rotate waypoint invalid amount of degrees"),
                };
            }
            _ => panic!("tried to rotate waypoint in invalid direction"),
        }
    }
}
