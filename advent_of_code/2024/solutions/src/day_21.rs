// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2024-day21/

use crate::{AoCData, AoCResult};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

#[derive(Debug, Clone)]
pub struct Data<'a>(Vec<&'a str>);

fn make_numpad() -> HashMap<Point, char> {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    HashMap::from([
        (Point::new(0, 0), '7'),
        (Point::new(0, 1), '8'),
        (Point::new(0, 2), '9'),
        (Point::new(1, 0), '4'),
        (Point::new(1, 1), '5'),
        (Point::new(1, 2), '6'),
        (Point::new(2, 0), '1'),
        (Point::new(2, 1), '2'),
        (Point::new(2, 2), '3'),
        (Point::new(3, 1), '0'),
        (Point::new(3, 2), 'A'),
    ])
}

fn make_dirpad() -> HashMap<Point, char> {
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    HashMap::from([
        (Point::new(0, 1), '^'),
        (Point::new(0, 2), 'A'),
        (Point::new(1, 0), '<'),
        (Point::new(1, 1), 'v'),
        (Point::new(1, 2), '>'),
    ])
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn execute(&self, inst: char, pad: &HashMap<Point, char>) -> (Point, Option<char>) {
        match inst {
            '^' => (Self::new(self.row - 1, self.col), None),
            '>' => (Self::new(self.row, self.col + 1), None),
            'v' => (Self::new(self.row + 1, self.col), None),
            '<' => (Self::new(self.row, self.col - 1), None),
            'A' => (Self::new(self.row, self.col), Some(pad[self])),
            _ => panic!("at the disco"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Node {
    cost: u64,
    pos: Point,
    instr: char,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calc_cost(
    goal: char,
    prev_instr: char,
    depth: u8,
    cache: &mut HashMap<(char, char, u8), u64>,
) -> u64 {
    if let Some(&cost) = cache.get(&(goal, prev_instr, depth)) {
        return cost;
    }
    if depth == 0 {
        return 1;
    }
    let point_to_key = make_dirpad();
    let key_to_point: HashMap<_, _> = point_to_key
        .iter()
        .map(|(pos, c)| (c, pos))
        .collect();

    let mut pq = BinaryHeap::new();

    // start pointed at prev instr
    pq.push((
        Node {
            cost: 0,
            pos: *key_to_point[&prev_instr],
            instr: 'A', // more accurate would be a "nothing" value
        },
        None,
    ));

    while let Some((node, output)) = pq.pop() {
        if output.is_some_and(|key| key == goal) {
            cache.insert((goal, prev_instr, depth), node.cost);
            return node.cost;
        }

        for new_instr in "A^<v>".chars() {
            let (new_pos, new_output) = node
                .pos
                .execute(new_instr, &point_to_key);
            // if new pos ends up in the empty position of the pad, continue
            if !point_to_key.contains_key(&new_pos) {
                continue;
            }
            // if output isn't equal to goal, continue
            if new_output.is_some_and(|instr| instr != goal) {
                continue;
            }

            // calculate total cost of executing new_instr
            let new_cost = node.cost + calc_cost(new_instr, node.instr, depth - 1, cache);

            pq.push((
                Node {
                    cost: new_cost,
                    pos: new_pos,
                    instr: new_instr,
                },
                new_output,
            ));
        }
    }

    panic!("at the disco");
}

fn solve(code: Vec<char>, depth: u8) -> u64 {
    let numpad = make_numpad();

    let mut pq = BinaryHeap::new();
    let mut costmap = HashMap::new();
    let mut cache = HashMap::new();

    // start pointed at the A on the numpad
    pq.push((
        Node {
            cost: 0,
            pos: Point::new(3, 2),
            instr: 'A',
        },
        0,
    ));

    while let Some((node, len)) = pq.pop() {
        if len == code.len() {
            return node.cost;
        }

        // insert (pos, instr, len) combo, if it was already seen, continue
        if costmap
            .insert((node.pos, node.instr, len), node.cost)
            .is_some()
        {
            continue;
        }

        for new_instr in "A^<v>".chars() {
            let (new_pos, output) = node.pos.execute(new_instr, &numpad);
            // if new pos ends up in the empty position of the pad, skip
            if !numpad.contains_key(&new_pos) {
                continue;
            }
            let mut new_len = len;
            if let Some(instr) = output {
                if instr != code[new_len] {
                    continue;
                }
                new_len += 1;
            }
            let new_cost = node.cost + calc_cost(new_instr, node.instr, depth, &mut cache);
            pq.push((
                Node {
                    cost: new_cost,
                    pos: new_pos,
                    instr: new_instr,
                },
                new_len,
            ));
        }
    }

    panic!("at the disco");
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input.lines().collect()))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut sum = 0;
        for code in &self.0 {
            let num: u64 = code
                .strip_suffix('A')
                .unwrap()
                .parse()
                .unwrap();
            let len = solve(code.chars().collect(), 2);
            sum += num * len
        }
        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut sum = 0;
        for code in &self.0 {
            let num: u64 = code
                .strip_suffix('A')
                .unwrap()
                .parse()
                .unwrap();
            let len = solve(code.chars().collect(), 25);
            sum += num * len
        }
        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "029A
980A
179A
456A
379A";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "126384");
    }
}
