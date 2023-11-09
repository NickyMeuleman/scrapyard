use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<[u8; 2]>);

struct Node {
    score: u32,
    free_connection: u8,
    remaining: Vec<[u8; 2]>,
}

struct Node2 {
    score: u32,
    len: u8,
    free_connection: u8,
    remaining: Vec<[u8; 2]>,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .lines()
                .map(|line| {
                    let (first, second) = line.split_once('/').unwrap();
                    [first.parse().unwrap(), second.parse().unwrap()]
                })
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let pieces = &self.0;
        let starting_nodes = pieces
            .iter()
            .filter(|piece| piece.contains(&0))
            .map(|&piece| {
                let remaining = pieces
                    .clone()
                    .into_iter()
                    .filter(|remain| *remain != piece)
                    .collect();
                let free_connection = if piece[0] == 0 { piece[1] } else { piece[0] };
                Node {
                    remaining,
                    score: free_connection.into(),
                    free_connection,
                }
            });
        let mut stack = Vec::from_iter(starting_nodes);
        let mut max = 0;
        while let Some(node) = stack.pop() {
            let candidates = node
                .remaining
                .iter()
                .filter_map(|&[first, second]| {
                    if first == node.free_connection {
                        Some(([first, second], second))
                    } else if second == node.free_connection {
                        Some(([first, second], first))
                    } else {
                        None
                    }
                });
            for (candidate, new_free) in candidates {
                let new_score = node.score + candidate[0] as u32 + candidate[1] as u32;
                let new_remaining = node
                    .remaining
                    .clone()
                    .into_iter()
                    .filter(|piece| *piece != candidate)
                    .collect();
                let new_node = Node {
                    free_connection: new_free,
                    score: new_score,
                    remaining: new_remaining,
                };

                max = max.max(new_score);
                stack.push(new_node);
            }
        }
        Ok(max)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let pieces = &self.0;
        let starting_nodes = pieces
            .iter()
            .filter(|piece| piece.contains(&0))
            .map(|&piece| {
                let remaining = pieces
                    .clone()
                    .into_iter()
                    .filter(|remain| *remain != piece)
                    .collect();
                let free_connection = if piece[0] == 0 { piece[1] } else { piece[0] };
                Node2 {
                    remaining,
                    score: free_connection.into(),
                    free_connection,
                    len: 1,
                }
            });
        let mut stack = Vec::from_iter(starting_nodes);
        // (len, score)
        let mut max = (0, 0);
        while let Some(node) = stack.pop() {
            let candidates = node
                .remaining
                .iter()
                .filter_map(|&[first, second]| {
                    if first == node.free_connection {
                        Some(([first, second], second))
                    } else if second == node.free_connection {
                        Some(([first, second], first))
                    } else {
                        None
                    }
                });
            for (candidate, new_free) in candidates {
                let new_score = node.score + candidate[0] as u32 + candidate[1] as u32;
                let new_remaining = node
                    .remaining
                    .clone()
                    .into_iter()
                    .filter(|piece| *piece != candidate)
                    .collect();
                let new_len = node.len + 1;
                if new_len == max.0 {
                    max = (max.0, max.1.max(new_score));
                } else if new_len > max.0 {
                    max = (new_len, new_score);
                }
                let new_node = Node2 {
                    free_connection: new_free,
                    score: new_score,
                    remaining: new_remaining,
                    len: new_len,
                };

                stack.push(new_node);
            }
        }
        Ok(max.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "31");
    }

    #[test]
    fn part_2() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "19");
    }
}
