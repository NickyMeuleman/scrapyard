use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    bytes: usize,
    ops: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> Self {
        Self {
            wrapped,
            bytes: 0,
            ops: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.ops
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.wrapped.read(buf)?;
        self.bytes += bytes;
        self.ops += 1;
        Ok(bytes)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    bytes: usize,
    ops: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> Self {
        Self {
            wrapped,
            bytes: 0,
            ops: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.ops
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.wrapped.write(buf)?;
        self.bytes += bytes;
        self.ops += 1;
        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}