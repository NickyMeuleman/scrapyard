use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Data {
    map: HashMap<Point, u8>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn neighbours(&self, map: &HashMap<Point, u8>) -> Vec<Point> {
        // offsets for UP, RIGHT, DOWN, LEFT
        // in form (row, col)
        let offsets: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        let mut neighbours = Vec::new();

        for (row_offset, col_offset) in offsets {
            let row = self.row as isize + row_offset;
            let col = self.col as isize + col_offset;
            if row >= 0 && col >= 0 {
                let point = Point {
                    row: row as usize,
                    col: col as usize,
                };
                if map.contains_key(&point) {
                    neighbours.push(point);
                }
            }
        }
        neighbours
    }

    fn get_basin_size(&self, map: &HashMap<Point, u8>, seen: &mut HashSet<Point>) -> usize {
        if seen.contains(self) {
            return 0;
        }
        if map.get(self) == Some(&9) {
            return 0;
        }

        seen.insert(self.clone());

        let mut total = 1;

        for neighbour in self.neighbours(map) {
            // RECURSIOOOOOOOOOOOOOOOOOOOOOON
            total += neighbour.get_basin_size(map, seen);
        }
        // attempt to write this for loop as an iterator chain: fails, returns 1
        // self.neighbours(map).iter().fold(1, |mut acc, neighbour| {
        //     acc += neighbour.get_basin_size(map, seen);
        //     acc
        // })
        total
    }
}

impl Data {
    fn part_one(self) -> usize {
        let mut risks = Vec::new();

        for (point, depth) in &self.map {
            // compare current depth to all neighbour depths
            let is_lowest = point
                .neighbours(&self.map)
                .iter()
                .filter_map(|point| self.map.get(point))
                .all(|neighbour_depth| depth < neighbour_depth);
            // if it's smaller than them all, add to risks vector
            if is_lowest {
                risks.push(1 + *depth as usize);
            }
        }

        risks.iter().sum()
    }

    fn part_two(&self) -> usize {
        let mut sizes = Vec::new();
        let mut seen: HashSet<Point> = HashSet::new();

        for (point, depth) in &self.map {
            // compare current depth to all neighbour depths
            let is_lowest = point
                .neighbours(&self.map)
                .iter()
                .filter_map(|point| self.map.get(point))
                .all(|neighbour_depth| depth < neighbour_depth);

            // if it's smaller than them all, calculate size of basin by expanding from the lowest point
            if is_lowest {
                let size = point.get_basin_size(&self.map, &mut seen);
                sizes.push(size);
            }
        }

        sizes.sort();
        sizes.iter().rev().take(3).product()
    }
}

impl FromStr for Data {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map = input
            .lines()
            .enumerate()
            .flat_map(|(row_idx, line)| parse_line(line, row_idx).into_iter())
            .collect();

        fn parse_line(input: &str, row_idx: usize) -> Vec<(Point, u8)> {
            input
                .chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    // can cast as u8 because every digit is 0 <= digit <= 9
                    let depth = c.to_digit(10).unwrap() as u8;
                    let point = Point {
                        row: row_idx,
                        col: col_idx,
                    };
                    (point, depth)
                })
                .collect()
        }

        Ok(Self { map })
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = input.parse().unwrap();
    println!("Part one answer: {}", data.clone().part_one());
    println!("Part two answer: {}", data.part_two());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let data: Data = input.parse().unwrap();
        dbg!(&data);
        assert_eq!(data.part_one(), 15);
    }

    #[test]

    fn part_two_example() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 61229);
    }
}
