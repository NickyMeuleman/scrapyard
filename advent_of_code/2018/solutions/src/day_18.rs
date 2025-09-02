use std::{collections::HashMap, fmt::Display};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<Acre>>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Acre {
    Open,
    Tree,
    Lumber,
}

#[derive(Default)]
struct Counts {
    open: u8,
    tree: u8,
    lumber: u8,
}

impl Counts {
    fn increment(&mut self, acre: Acre) {
        match acre {
            Acre::Open => self.open += 1,
            Acre::Tree => self.tree += 1,
            Acre::Lumber => self.lumber += 1,
        }
    }
}

// slower but shorter (and harder to read imo)
// fn neighbours(grid: &[Vec<Acre>], x: usize, y: usize) -> Counts {
//     let width = grid[0].len();
//     let height = grid.len();
//     let mut counts = Counts::default();
//     for dy in y.saturating_sub(1)..=(y + 1).min(height - 1) {
//         for dx in x.saturating_sub(1)..=(x + 1).min(width - 1) {
//             if dx == x && dy == y {
//                 continue;
//             }
//             counts.increment(grid[dy][dx]);
//         }
//     }
//     counts
// }
// addresses clippy lints while making it even harder to read
// fn neighbours(grid: &[Vec<Acre>], x: usize, y: usize) -> Counts {
//     let width = grid[0].len();
//     let height = grid.len();
//     let mut counts = Counts::default();
//     for (dy, row) in grid
//         .iter()
//         .enumerate()
//         .take((y + 1).min(height - 1) + 1)
//         .skip(y.saturating_sub(1))
//     {
//         for (dx, &acre) in row
//             .iter()
//             .enumerate()
//             .take((x + 1).min(width - 1) + 1)
//             .skip(x.saturating_sub(1))
//         {
//             if dx == x && dy == y {
//                 continue;
//             }
//             counts.increment(acre);
//         }
//     }
//     counts
// }
// slower, but shorter still
// fn neighbours(grid: &[Vec<Acre>], x: usize, y: usize) -> Counts {
//     let width = grid[0].len();
//     let height = grid.len();
//     let y_range = y.saturating_sub(1)..=(y + 1).min(height - 1);
//     let x_range = x.saturating_sub(1)..=(x + 1).min(width - 1);
//
//     y_range
//         .flat_map(|dy| x_range.clone().map(move |dx| (dy, dx)))
//         .filter(|&(dy, dx)| dy != y || dx != x)
//         .fold(Counts::default(), |mut counts, (dy, dx)| {
//             counts.increment(grid[dy][dx]);
//             counts
//         })
// }

// very verbose, but clear and fast
fn neighbours(map: &[Vec<Acre>], x: usize, y: usize) -> Counts {
    let width = map[0].len();
    let height = map.len();
    let mut res = Counts::default();
    // up
    if y > 0 {
        res.increment(map[y - 1][x]);
    }
    // up right
    if (y > 0) && (x < width - 1) {
        res.increment(map[y - 1][x + 1]);
    }
    // right
    if x < width - 1 {
        res.increment(map[y][x + 1]);
    }
    // down right
    if (y < height - 1) && (x < width - 1) {
        res.increment(map[y + 1][x + 1]);
    }
    // down
    if y < height - 1 {
        res.increment(map[y + 1][x]);
    }
    // down left
    if (y < height - 1) && (x > 0) {
        res.increment(map[y + 1][x - 1]);
    }
    // left
    if x > 0 {
        res.increment(map[y][x - 1]);
    }
    // up left
    if (y > 0) && (x > 0) {
        res.increment(map[y - 1][x - 1]);
    }
    res
}

fn tick(map: &mut Vec<Vec<Acre>>, new_map: &mut Vec<Vec<Acre>>) {
    for (y, row) in map.iter().enumerate() {
        for (x, acre) in row.iter().enumerate() {
            let Counts { tree, lumber, .. } = neighbours(map, x, y);
            let new = match acre {
                Acre::Open if tree >= 3 => Acre::Tree,
                Acre::Tree if lumber >= 3 => Acre::Lumber,
                Acre::Lumber if lumber == 0 || tree == 0 => Acre::Open,
                _ => *acre,
            };
            new_map[y][x] = new;
        }
    }
    std::mem::swap(map, new_map);
}

fn get_resource_val(map: &[Vec<Acre>]) -> u32 {
    let (wood, lumber) = map
        .iter()
        .flatten()
        .fold((0, 0), |(wood, lumber), acre| match acre {
            Acre::Tree => (wood + 1, lumber),
            Acre::Lumber => (wood, lumber + 1),
            _ => (wood, lumber),
        });
    wood * lumber
}

// fn print(map: &[Vec<Acre>]) {
//     for y in 0..map.len() {
//         for x in 0..map[0].len() {
//             let c = match map[y][x] {
//                 Acre::Open => '.',
//                 Acre::Tree => '|',
//                 Acre::Lumber => '#',
//             };
//             print!("{}", c);
//         }
//         println!();
//     }
// }

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Acre::Open,
                            '|' => Acre::Tree,
                            '#' => Acre::Lumber,
                            _ => panic!("at the disco"),
                        })
                        .collect()
                })
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut map = self.0.clone();
        let mut new_map = self.0.clone();
        // print(&map);
        for _ in 0..10 {
            tick(&mut map, &mut new_map);
            // println!("After {i} minutes: ");
            // print(&map);
            // println!();
        }
        Ok(get_resource_val(&map))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut map = self.0.clone();
        let mut new_map = self.0.clone();
        let mut seen = HashMap::new();
        for i in 0..1_000_000_000 {
            if let Some(seen_at) = seen.get(&map) {
                let period = i - seen_at;
                let remaining = 1_000_000_000 - seen_at;
                for _ in 0..remaining % period {
                    tick(&mut map, &mut new_map);
                }
                break;
            }
            seen.insert(map.clone(), i);
            tick(&mut map, &mut new_map);
        }
        Ok(get_resource_val(&map))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "1147");
    }
}
