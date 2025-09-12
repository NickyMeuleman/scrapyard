use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    depth: u32,
    target: (u32, u32),
}

struct Cave {
    depth: u32,
    target: (u32, u32),
    cache: HashMap<(u32, u32), u32>,
}
impl Cave {
    fn geological_index(&mut self, x: u32, y: u32) -> u32 {
        if let Some(&idx) = self.cache.get(&(x, y)) {
            return idx;
        }
        let idx = match (x, y) {
            // If the region's Y coordinate is 0, the geologic index is its X coordinate times 16807.
            (x, 0) => x * 16807,
            // If the region's X coordinate is 0, the geologic index is its Y coordinate times 48271.
            (0, y) => y * 48271,
            // The region at the coordinates of the target has a geologic index of 0.
            p if p == self.target => 0,
            // Otherwise, the region's geologic index is the result of multiplying the erosion levels of the regions at X-1,Y and X,Y-1.
            (x, y) => self.erosion_level(x - 1, y) * self.erosion_level(x, y - 1),
        };
        self.cache.insert((x, y), idx);
        idx
    }

    fn erosion_level(&mut self, x: u32, y: u32) -> u32 {
        // A region's erosion level is its geologic index plus the cave system's depth, all modulo 20183
        (self.geological_index(x, y) + self.depth) % 20183
    }

    fn risk_level(&mut self, x: u32, y: u32) -> u32 {
        self.erosion_level(x, y) % 3
    }

    fn region_type(&mut self, x: u32, y: u32) -> Type {
        match self.risk_level(x, y) {
            0 => Type::Rocky,
            1 => Type::Wet,
            2 => Type::Narrow,
            _ => unreachable!(),
        }
    }

    fn heuristic(&self, pos: (u32, u32), tool: Option<Tool>) -> u32 {
        let manhattan = self.target.0.abs_diff(pos.0) + self.target.1.abs_diff(pos.1);
        let tool_cost = if tool == Some(Tool::Torch) { 0 } else { 7 };
        manhattan + tool_cost
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Type {
    Rocky,
    Narrow,
    Wet,
}
impl Type {
    fn possible(&self, tool: &Option<Tool>) -> bool {
        matches!(
            (self, tool),
            (Type::Rocky, Some(Tool::ClimbingGear) | Some(Tool::Torch))
                | (Type::Wet, Some(Tool::ClimbingGear) | None)
                | (Type::Narrow, Some(Tool::Torch) | None)
        )
    }
    fn other_tool(&self, tool: &Option<Tool>) -> Option<Tool> {
        assert!(self.possible(tool), "incompatible tool");
        match (self, tool) {
            (Type::Rocky, Some(Tool::ClimbingGear)) => Some(Tool::Torch),
            (Type::Rocky, Some(Tool::Torch)) => Some(Tool::ClimbingGear),
            (Type::Wet, Some(Tool::ClimbingGear)) => None,
            (Type::Wet, None) => Some(Tool::ClimbingGear),
            (Type::Narrow, Some(Tool::Torch)) => None,
            (Type::Narrow, None) => Some(Tool::Torch),
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Tool {
    Torch,
    ClimbingGear,
}

#[derive(PartialEq, Eq)]
struct Node {
    cost: u32,
    heuristic: u32,
    pos: (u32, u32),
    tool: Option<Tool>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let other_total = other.cost + other.heuristic;
        let self_total = self.cost + self.heuristic;
        other_total.cmp(&self_total)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut lines = input.lines();
        let (_, depth) = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap();
        let (_, target) = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap();
        let (row, col) = target.split_once(",").unwrap();
        Ok(Self {
            depth: depth.parse().unwrap(),
            target: (row.parse().unwrap(), col.parse().unwrap()),
        })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut cave = Cave {
            depth: self.depth,
            target: self.target,
            cache: HashMap::new(),
        };
        let mut sum = 0;
        for y in 0..=cave.target.1 {
            for x in 0..=cave.target.0 {
                sum += cave.risk_level(x, y);
            }
        }
        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut cave = Cave {
            depth: self.depth,
            target: self.target,
            cache: HashMap::new(),
        };

        let mut pq = BinaryHeap::new();
        let mut min_costs: HashMap<((u32, u32), Option<Tool>), u32> = HashMap::new();

        let start_pos = (0, 0);
        let start_tool = Some(Tool::Torch);
        let start_cost = 0;

        pq.push(Node {
            cost: start_cost,
            heuristic: cave.heuristic(start_pos, start_tool),
            pos: start_pos,
            tool: start_tool,
        });
        min_costs.insert((start_pos, start_tool), start_cost);

        while let Some(Node {
            cost, pos, tool, ..
        }) = pq.pop()
        {
            if pos == self.target && tool == Some(Tool::Torch) {
                return Ok(cost);
            }
            // Check if we've found a shorter path already
            if cost
                > *min_costs
                    .get(&(pos, tool))
                    .unwrap_or(&u32::MAX)
            {
                continue;
            }
            // swap tools
            let new_tool = cave
                .region_type(pos.0, pos.1)
                .other_tool(&tool);
            let new_cost = cost + 7;
            if new_cost
                < *min_costs
                    .get(&(pos, new_tool))
                    .unwrap_or(&u32::MAX)
            {
                min_costs.insert((pos, new_tool), new_cost);
                pq.push(Node {
                    cost: new_cost,
                    heuristic: cave.heuristic(pos, new_tool),
                    pos,
                    tool: new_tool,
                });
            }
            // move to neighbour
            for (dx, dy) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                // my favourite, dealing with unsigned integers possibly underflowing
                if (pos.0 == 0 && dx == -1) || (pos.1 == 0 && dy == -1) {
                    continue;
                }
                let new_pos = ((pos.0 as i32 + dx) as u32, (pos.1 as i32 + dy) as u32);
                if !cave
                    .region_type(new_pos.0, new_pos.1)
                    .possible(&tool)
                {
                    continue;
                }
                let new_cost = cost + 1;
                if new_cost
                    >= *min_costs
                        .get(&(new_pos, tool))
                        .unwrap_or(&u32::MAX)
                {
                    continue;
                }
                min_costs.insert((new_pos, tool), new_cost);
                pq.push(Node {
                    cost: new_cost,
                    heuristic: cave.heuristic(new_pos, tool),
                    pos: new_pos,
                    tool,
                });
            }
        }
        Ok(u32::MAX)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "depth: 510
target: 10,10
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "114");
    }

    #[test]
    fn part_2() {
        let input = "depth: 510
target: 10,10
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "45");
    }
}
