use aoc_core::Solution;

use crate::{AoCData, AoCResult};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Sand,
    Clay,
    Still,
    Flow,
}

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// enum Dir {
//     Down,
//     Left,
//     Right,
// }
//
// impl Dir {
//     fn val(&self) -> i32 {
//         match self {
//             Self::Down => 0,
//             Self::Left => -1,
//             Self::Right => 1,
//         }
//     }
// }
//
// enum FlowResult {
//     Bounded(usize),
//     Unbounded,
// }

// fn flow(grid: &mut Vec<Vec<Tile>>, x: usize, y: usize, dir: Dir) -> FlowResult {
//     if y >= grid.len() - 1 {
//         return FlowResult::Unbounded;
//     }
//     if grid[y][x] == Tile::Clay {
//         return FlowResult::Bounded(x);
//     }
//
//     if grid[y][x] == Tile::Sand {
//         grid[y][x] = Tile::Flow;
//     }
//
//     // flow down
//     if grid[y + 1][x] == Tile::Sand {
//         flow(grid, x, y + 1, Dir::Down);
//     }
//     // flow left and right (if supported by clay or still water)
//     if matches!(grid[y + 1][x], Tile::Still | Tile::Clay) {
//         // continue flowing to sides
//         if dir != Dir::Down {
//             return flow(grid, (x as i32 + dir.val()) as usize, y, dir);
//         }
//         // check if walled in, fill if true
//         let left_x = flow(grid, x - 1, y, Dir::Left);
//         let right_x = flow(grid, x + 1, y, Dir::Right);
//         if let (FlowResult::Bounded(lx), FlowResult::Bounded(rx)) = (left_x, right_x) {
//             if grid[y][lx] == Tile::Clay && grid[y][rx] == Tile::Clay {
//                 grid[y][lx + 1..rx].fill(Tile::Still);
//             }
//         }
//     }
//
//     FlowResult::Bounded(x)
// }

fn solve(input: &str) -> Option<(u32, u32)> {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    // parse input and get bounds to turn the infinite space into a finite one
    let mut ranges = Vec::new();
    for line in input.lines() {
        let (single_part, range_part) = line.split_once(", ").unwrap();
        let is_x = single_part.starts_with('x');
        let single: i32 = single_part[2..].parse().unwrap();
        let range = &range_part[2..];
        let (start, end) = range.split_once("..").unwrap();
        let start: i32 = start.parse().unwrap();
        let end: i32 = end.parse().unwrap();

        min_x = min_x.min(if is_x { single } else { start });
        max_x = max_x.max(if is_x { single } else { end });
        min_y = min_y.min(if is_x { start } else { single });
        max_y = max_y.max(if is_x { end } else { single });
        ranges.push((is_x, single, start, end));
    }

    // populate a 2d grid with sand and clay
    // restrict x start and end, but do not restrict y start
    let width = (max_x - min_x + 2) as usize;
    let height = (max_y + 1) as usize;
    let mut grid = vec![vec![Tile::Sand; width]; height];

    for (is_x, single, start, end) in ranges {
        for num in start..=end {
            let x = if is_x { single } else { num } - min_x + 1;
            let y = if is_x { num } else { single };
            grid[y as usize][x as usize] = Tile::Clay;
        }
    }

    let spring_x = (500 - min_x + 1) as usize;
    let spring_y = 0;

    // recursive solution
    // flow(&mut grid, spring_x, spring_y, Dir::Down);
    // or
    // iterative solution
    let mut stack = vec![(spring_x, spring_y)];
    while let Some((x, y)) = stack.pop() {
        if y >= height {
            continue;
        }

        if grid[y][x] == Tile::Sand {
            grid[y][x] = Tile::Flow;
        }

        // water does not flow into clay or already filled tiles
        if matches!(grid[y][x], Tile::Clay | Tile::Still) {
            continue;
        }

        if y + 1 >= height {
            continue;
        }

        if grid[y + 1][x] == Tile::Sand {
            stack.push((x, y + 1));
            continue;
        }

        // At this point, grid[y+1][x] is Clay, Still, or Flow
        // Check for horizontal spreading and potential filling
        if matches!(grid[y + 1][x], Tile::Clay | Tile::Still) {
            // Find the left and right boundaries of the pool
            let mut left_bound = x;
            while left_bound > 0
                && !matches!(grid[y][left_bound - 1], Tile::Clay)
                && matches!(grid[y + 1][left_bound], Tile::Clay | Tile::Still)
            {
                left_bound -= 1;
            }

            let mut right_bound = x;
            while right_bound + 1 < width
                && !matches!(grid[y][right_bound + 1], Tile::Clay)
                && matches!(grid[y + 1][right_bound], Tile::Clay | Tile::Still)
            {
                right_bound += 1;
            }

            // Check if contained
            let left_wall = left_bound > 0 && grid[y][left_bound - 1] == Tile::Clay;
            let right_wall = right_bound + 1 < width && grid[y][right_bound + 1] == Tile::Clay;

            if left_wall && right_wall {
                // Contained, fill with still water
                grid[y][left_bound..=right_bound].fill(Tile::Still);
                // push tile above to stack to process next layer
                stack.push((x, y - 1));
            } else {
                // Not contained, mark as flowing
                grid[y][left_bound..=right_bound].fill(Tile::Flow);
                // Not contained, water overflows. Add new sources where it falls off the edge.
                if !left_wall && grid[y + 1][left_bound] == Tile::Sand {
                    stack.push((left_bound, y));
                }
                if !right_wall && grid[y + 1][right_bound] == Tile::Sand {
                    stack.push((right_bound, y));
                }
            }
        }
    }

    let (flowing, still) = grid
        .iter()
        .take(max_y as usize + 1)
        .skip(min_y as usize)
        .flatten()
        .fold((0, 0), |(flow, still), &tile| match tile {
            Tile::Flow => (flow + 1, still),
            Tile::Still => (flow, still + 1),
            _ => (flow, still),
        });

    // show(&grid);
    Some((still + flowing, still))
}

// fn show(map: &Vec<Vec<Tile>>) {
//     for y in 0..map.len() {
//         for x in 0..map[0].len() {
//             let c = match map[y][x] {
//                 Tile::Sand => '.',
//                 Tile::Clay => '#',
//                 Tile::Still => '~',
//                 Tile::Flow => '|',
//             };
//             print!("{}", c);
//         }
//         println!()
//     }
// }

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let (res, _) = solve(self.0).unwrap();
        Ok(res)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let (_, res) = solve(self.0).unwrap();
        Ok(res)
    }

    fn solve(self) -> AoCResult<Solution>
    where
        Self: Sized,
    {
        let (part1, part2) = solve(self.0).unwrap();
        Ok(Solution {
            part1: Box::new(part1),
            part2: Box::new(part2),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "57");
    }

    #[test]
    fn part_2() {
        let input = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "29");
    }
}
