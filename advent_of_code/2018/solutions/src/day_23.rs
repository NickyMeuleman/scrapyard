use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}
impl Pos {
    fn dist(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Bot {
    pos: Pos,
    radius: u32,
}
impl Bot {
    fn in_range(&self, other: &Bot) -> bool {
        self.pos.dist(&other.pos) <= self.radius
    }
}

#[derive(Debug, Clone)]
pub struct Data(Vec<Bot>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut bots = Vec::new();
        for line in input.lines() {
            let (pos, radius) = line.split_once(", ").unwrap();
            let pos = pos.strip_prefix("pos=<").unwrap();
            let pos = pos.strip_suffix(">").unwrap();
            let pos: Vec<i32> = pos
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            let radius = radius.strip_prefix("r=").unwrap();
            let radius = radius.parse().unwrap();
            let bot = Bot {
                pos: Pos {
                    x: pos[0],
                    y: pos[1],
                    z: pos[2],
                },
                radius,
            };
            bots.push(bot);
        }
        Ok(Self(bots))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let strongest = self
            .0
            .iter()
            .max_by_key(|bot| bot.radius)
            .unwrap();

        Ok(self
            .0
            .iter()
            .filter(|bot| strongest.in_range(bot))
            .count())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let origin = Pos { x: 0, y: 0, z: 0 };
        // Each bot defines a range of distances from the origin where you can stand and be in range.
        // Turn those into "events" for a sweep line: +1 when entering range, -1 when leaving
        // Vec<(distance, delta)>
        let mut events: Vec<(u32, i32)> = Vec::new();

        for bot in &self.0 {
            let dist_to_origin = bot.pos.dist(&origin);
            let start = dist_to_origin.saturating_sub(bot.radius);
            let end = dist_to_origin + bot.radius;

            // entering range
            events.push((start, 1));
            // leaving range
            events.push((end + 1, -1));
        }

        // Sort by distance, then by delta so that leaving happens after entering at same point
        events.sort_unstable();

        let mut count = 0;
        let mut best_count = 0;
        let mut best_dist = 0;

        for (dist, delta) in events {
            count += delta;
            if count > best_count {
                best_count = count;
                best_dist = dist;
            }
        }

        Ok(best_dist)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "7");
    }

    #[test]
    fn part_2() {
        let input = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "36");
    }
}
