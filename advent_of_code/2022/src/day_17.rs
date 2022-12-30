use std::collections::HashMap;

use crate::AoCData;

const WIDTH: usize = 7;
const PIECES: [&[Coord]; 5] = [
    // horizontal line
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 2, y: 0 },
        Coord { x: 3, y: 0 },
    ],
    // plus
    &[
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 0 },
        Coord { x: 1, y: 1 },
        Coord { x: 1, y: 2 },
        Coord { x: 2, y: 1 },
    ],
    // J (or backwards L)
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 2, y: 0 },
        Coord { x: 2, y: 1 },
        Coord { x: 2, y: 2 },
    ],
    // vertical line
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 0, y: 1 },
        Coord { x: 0, y: 2 },
        Coord { x: 0, y: 3 },
    ],
    // square
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 1 },
    ],
];

enum Jet {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Default)]
struct Coord {
    x: usize,
    // positive y goes up.
    // happy mathematicians, sad game programmers
    y: usize,
}

#[derive(Default)]
struct State {
    jet_count: usize,
    piece_count: usize,
    added_by_repeats: usize,
    top: usize,
    map: Vec<[bool; WIDTH]>,
    curr: Coord,
    seen: HashMap<(usize, usize), (usize, usize, usize)>,
}

impl State {
    fn is_valid(&mut self, new_curr: &Coord, piece: &[Coord]) -> bool {
        piece.iter().all(|offset| {
            let x = new_curr.x + offset.x;
            let y = new_curr.y + offset.y;
            while self.map.len() <= y {
                self.map.push([false; WIDTH]);
            }
            x < WIDTH && !self.map[y][x]
        })
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = PIECES[self.piece_count % PIECES.len()];
        let mut print: Vec<Vec<_>> = self
            .map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|rock| if *rock { '#' } else { '.' })
                    .collect()
            })
            .collect();
        let mut local_top = self.top;
        for offset in piece {
            let x = self.curr.x + offset.x;
            let y= self.curr.y + offset.y;
            while print.len() <= y {
                print.push(vec!['.'; WIDTH]);
            }
            print[y][x] = '@';
            local_top = local_top.max(y);
        }
        for row in (0..=local_top).rev() {
            let mut row_str = String::from('|');
            for col in 0..7 {
                row_str.push(print[row][col]);
            }
            row_str.push('|');
            row_str.push('\n');
            write!(f, "{}", row_str)?;
        }
        writeln!(f, "+{}+", "-".repeat(WIDTH))
    }
}

pub struct Data(Vec<Jet>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> Option<Self> {
        let jets = input
            .trim()
            .chars()
            .map(|c| match c {
                '<' => Some(Jet::Left),
                '>' => Some(Jet::Right),
                _ => None,
            })
            .collect::<Option<_>>()?;

        Some(Self(jets))
    }

    fn part_1(&self) -> String {
        let target = 2;
        let jets = &self.0;
        let mut state: State = Default::default();

        while state.piece_count != target {
            // new piece starts falling
            let piece = PIECES[state.piece_count % PIECES.len()];
            state.curr.x = 2;
            state.curr.y = state.top + 3;

            // println!("== Piece {} begins falling ==", state.piece_count + 1);
            // println!("{}", state);

            loop {
                // jet
                let jet = &jets[state.jet_count % jets.len()];
                let new_curr = match jet {
                    Jet::Left => Coord {
                        x: state.curr.x.saturating_sub(1),
                        y: state.curr.y,
                    },
                    Jet::Right => Coord {
                        x: state.curr.x + 1,
                        y: state.curr.y,
                    },
                };
                if state.is_valid(&new_curr, piece) {
                    state.curr = new_curr;
                }
                state.jet_count += 1;

                // println!(
                //     "Jet of gas pushes piece {}:",
                //     match jet {
                //         Jet::Left => "left",
                //         Jet::Right => "right",
                //     }
                // );
                // println!("{}", state);

                // fall
                let new_curr = Coord {
                    x: state.curr.x,
                    y: state.curr.y.saturating_sub(1),
                };
                if state.curr.y == 0 || !state.is_valid(&new_curr, piece) {
                    break;
                }
                state.curr = new_curr;

                // println!("Piece falls 1 unit:");
                // println!("{}", state);
            }

            // settle
            for offset in piece {
                let x = state.curr.x + offset.x;
                let y = state.curr.y + offset.y;
                state.map[y][x] = true;
                // y is 0 indexed.
                state.top = state.top.max(y + 1);
            }

            // prepare for next iteration of while loop
            state.piece_count += 1;
        }

        state.top.to_string()
    }

    fn part_2(&self) -> String {
        let target = 1_000_000_000_000;
        let jets = &self.0;
        let mut state: State = Default::default();
    
        while state.piece_count != target {
            // new piece starts falling
            let piece = PIECES[state.piece_count % PIECES.len()];
            state.curr.x = 2;
            state.curr.y = state.top + 3;
    
            loop {
                // jet
                let jet = &jets[state.jet_count % jets.len()];
                let new_curr = match jet {
                    Jet::Left => Coord {
                        x: state.curr.x.saturating_sub(1),
                        y: state.curr.y,
                    },
                    Jet::Right => Coord {
                        x: state.curr.x + 1,
                        y: state.curr.y,
                    },
                };
                if state.is_valid(&new_curr, piece) {
                    state.curr = new_curr;
                }
                state.jet_count += 1;
    
                // fall
                let new_curr = Coord {
                    x: state.curr.x,
                    y: state.curr.y.saturating_sub(1),
                };
                if state.curr.y == 0 || !state.is_valid(&new_curr, piece) {
                    break;
                }
                state.curr = new_curr;
            }
    
            // settle
            for offset in piece {
                let x = state.curr.x + offset.x;
                let y = state.curr.y + offset.y;
                while state.map.len() <= y {
                    state.map.push([false; WIDTH]);
                }
                state.map[y][x] = true;
                // y is 0 indexed
                state.top = state.top.max(y + 1);
            }
    
            // look for cycle
            if state.added_by_repeats == 0 {
                let key = (
                    state.piece_count % PIECES.len(),
                    state.jet_count % jets.len(),
                );
                // at third occurrence of key, the values in the seen map repeat
                // add as many of them as possible without hitting the goal piece_count
                if let Some((2, old_piece_count, old_top)) = state.seen.get(&key) {
                    let delta_top = state.top - old_top;
                    let delta_piece_count = state.piece_count - old_piece_count;
                    let repeats = (target - state.piece_count) / delta_piece_count;
                    state.added_by_repeats += repeats * delta_top;
                    state.piece_count += repeats * delta_piece_count;
                }
                // update seen map
                // key: (piece_count % PIECES.len(), jet_count % jets.len())
                // value: (amount_of_times_key_was_seen, piece_count, top)
                state
                    .seen
                    .entry(key)
                    .and_modify(|(amnt, old_piece_count, old_top)| {
                        *amnt += 1;
                        *old_piece_count = state.piece_count;
                        *old_top = state.top;
                    })
                    .or_insert((1, state.piece_count, state.top));
            }
    
            // prepare for next iteration of while loop
            state.piece_count += 1;
        }
    
        let top = state.top + state.added_by_repeats;
        top.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(17);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "3068");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(17);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_2(), "1514285714288");
    }
}
