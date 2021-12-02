use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data = parse(&input);
    println!("Part one answer: {}", part_one(&data));
    println!("Part two answer: {}", part_two(&data));
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: u8,
}

impl Instruction {
    fn new(input: &str) -> Self {
        let instruction: Vec<_> = input.split_whitespace().collect();
        let amount = instruction[1].parse().unwrap();
        let direction = match instruction[0] {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => panic!("invalid input data"),
        };
        Self { direction, amount }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| Instruction::new(line)).collect()
}

fn part_one(data: &Vec<Instruction>) -> u32 {
    let (horizontal, depth): (u32, u32) =
        data.iter()
            .fold((0, 0), |(horizontal, depth), instruction| {
                let direction = &instruction.direction;
                let amount = instruction.amount as u32;
                match direction {
                    Direction::Up => (horizontal, depth - amount),
                    Direction::Down => (horizontal, depth + amount),
                    Direction::Forward => (horizontal + amount, depth),
                }
            });
    horizontal * depth
}

fn part_two(data: &Vec<Instruction>) -> u32 {
    let (_, horizontal, depth): (u32, u32, u32) =
        data.iter()
            .fold((0, 0, 0), |(aim, horizontal, depth), instruction| {
                let direction = &instruction.direction;
                let amount = instruction.amount as u32;
                match direction {
                    Direction::Up => (aim - amount, horizontal, depth),
                    Direction::Down => (aim + amount, horizontal, depth),
                    Direction::Forward => (aim, horizontal + amount, depth + (aim * amount)),
                }
            });
    horizontal * depth
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        let data = parse(input);
        assert_eq!(part_one(&data), 150);
    }

    #[test]
    fn part_two_example() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        let data = parse(input);
        assert_eq!(part_two(&data), 900);
    }
}
