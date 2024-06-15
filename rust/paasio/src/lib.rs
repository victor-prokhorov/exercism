use std::io::{Read, Result, Write};
// use std::marker::PhantomData;

pub struct ReadStats<R> {
    wrapped: R,
    bytes_read: usize,
    read_count: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            bytes_read: 0,
            read_count: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.read_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.read_count += 1;

        let bytes_read = self.wrapped.read(buf)?;
        self.bytes_read += bytes_read;
        Ok(bytes_read)
    }
}

pub struct WriteStats<W>(W, usize, usize);

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats(wrapped, 0, 0)
    }

    pub fn get_ref(&self) -> &W {
        &self.0
    }

    pub fn bytes_through(&self) -> usize {
        self.1
    }

    pub fn writes(&self) -> usize {
        self.2
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.2 += 1;

        let bytes = self.0.write(buf)?;
        self.1 += bytes;
        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}
