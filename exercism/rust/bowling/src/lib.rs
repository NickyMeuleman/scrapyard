#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    buffer: Vec<u16>,
    frames: Vec<Frame>,
}

#[derive(Debug, PartialEq)]
pub enum Frame {
    Open(u16, u16),
    Spare(u16),
    Strike,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            frames: Vec::new(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_over() {
            return Err(Error::GameComplete);
        }
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        match self.buffer.len() {
            0 => {
                // first roll of a frame or first fillball
                match pins {
                    10 => {
                        if self.frames.len() < 10 {
                            self.frames.push(Frame::Strike);
                        } else {
                            self.buffer.push(pins)
                        }
                        Ok(())
                    }
                    _ => {
                        self.buffer.push(pins);
                        Ok(())
                    }
                }
            }
            1 => {
                // second roll of a frame or second fillball
                let pins_left = match self.buffer[0] {
                    // pins were reset is first fillball was a strike
                    10 => 10,
                    _ => 10 - self.buffer[0],
                };
                if pins <= pins_left {
                    match pins == pins_left {
                        true => {
                            if self.frames.len() < 10 {
                                self.frames.push(Frame::Spare(self.buffer[0]));
                                self.buffer.clear();
                            } else {
                                self.buffer.push(pins)
                            }
                            Ok(())
                        }
                        false => {
                            if self.frames.len() < 10 {
                                self.frames.push(Frame::Open(self.buffer[0], pins));
                                self.buffer.clear();
                            } else {
                                self.buffer.push(pins)
                            }
                            Ok(())
                        }
                    }
                } else {
                    Err(Error::NotEnoughPinsLeft)
                }
            }
            _ => Err(Error::NotEnoughPinsLeft),
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_game_over() {
            let total = self
                .frames
                .iter()
                .enumerate()
                .map(|(i, frame)| self.score_frame(frame, i))
                .sum();
            Some(total)
        } else {
            None
        }
    }

    fn is_game_over(&self) -> bool {
        match self.frames.len() {
            10 => match self.frames.last().unwrap() {
                Frame::Open(_, _) => true,
                Frame::Spare(_) => self.buffer.len() == 1,
                Frame::Strike => self.buffer.len() == 2,
            },
            _ => false,
        }
    }

    fn score_frame(&self, frame: &Frame, frame_idx: usize) -> u16 {
        match frame {
            Frame::Open(first, second) => first + second,
            Frame::Spare(_) => {
                let next_roll = match self.frames.get(frame_idx + 1) {
                    Some(next_frame) => match next_frame {
                        Frame::Open(roll, _) => *roll,
                        Frame::Spare(roll) => *roll,
                        Frame::Strike => 10,
                    },
                    None => {
                        // function arg frame was the last frame, bonus roll is in buffer
                        self.buffer[0]
                    }
                };
                10 + next_roll
            }
            Frame::Strike => {
                let (next_roll, second_next_roll) = match self.frames.get(frame_idx + 1) {
                    Some(next_frame) => match next_frame {
                        Frame::Open(first, second) => (*first, *second),
                        Frame::Spare(first) => (*first, 10 - *first),
                        Frame::Strike => {
                            match self.frames.get(frame_idx + 2) {
                                Some(next_frame) => match next_frame {
                                    Frame::Open(first, _) => (10, *first),
                                    Frame::Spare(first) => (10, *first),
                                    Frame::Strike => (10, 10),
                                },
                                None => {
                                    // next frame was the last frame, bonus roll is in buffer
                                    (10, self.buffer[0])
                                }
                            }
                        }
                    },
                    None => {
                        // function arg frame was the last frame, bonus rolls are in buffer
                        (self.buffer[0], self.buffer[1])
                    }
                };
                10 + next_roll + second_next_roll
            }
        }
    }
}