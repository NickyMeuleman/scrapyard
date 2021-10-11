use std::collections::VecDeque;

pub struct CircularBuffer<T> {
    capacity: usize,
    data: VecDeque<T>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: VecDeque::new(),
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.data.len() == self.capacity {
            Err(Error::FullBuffer)
        } else {
            Ok(self.data.push_back(element))
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.data.pop_front().ok_or(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn overwrite(&mut self, element: T) {
        if self.data.len() == self.capacity {
            self.data.pop_front();
        }
        self.data.push_back(element)
    }
}
