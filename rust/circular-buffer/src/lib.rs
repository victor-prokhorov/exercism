mod vec_deque;

pub use vec_deque::CircularBuffer as VecDeque;

use std::mem::MaybeUninit;

pub struct CircularBuffer<T> {
    buffer: Box<[MaybeUninit<T>]>,
    // buffer: Vec<MaybeUninit<T>>,
    write_index: usize,
    read_index: usize,
    len: usize,
    capacity: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> Drop for CircularBuffer<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        unsafe {
            buffer.set_len(capacity);
        }
        Self {
            buffer: buffer.into_boxed_slice(),
            write_index: 0,
            read_index: 0,
            len: 0,
            capacity,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.len == self.capacity {
            Err(Error::FullBuffer)
        } else {
            self.buffer[self.write_index].write(element);
            self.len += 1;
            self.write_index = (self.write_index + 1) % self.capacity;
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.len == 0 {
            return Err(Error::EmptyBuffer);
        }
        let removed = unsafe { self.buffer[self.read_index].assume_init_read() };
        self.len -= 1;
        self.read_index = (self.read_index + 1) % self.capacity;
        Ok(removed)
    }

    pub fn clear(&mut self) {
        while let Ok(_) = self.read() {}
    }

    pub fn overwrite(&mut self, element: T) {
        if self.len < self.capacity {
            self.write(element).unwrap();
        } else {
            self.read().unwrap();
            self.write(element).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn owned() {
        let mut buffer = CircularBuffer::new(1);
        buffer.write("1".to_owned()).unwrap();
        buffer.read().unwrap();
    }
    #[test]
    #[ignore]
    fn read_then_clear_full_buffer() {
        let mut buffer = CircularBuffer::new(2);
        buffer.write("1".to_owned()).unwrap();
        buffer.write("2".to_owned()).unwrap();
        buffer.read().unwrap();
        buffer.clear();
    }

    #[test]
    #[ignore]
    fn write() {
        let mut buffer = CircularBuffer::new(1);
        buffer.write("1".to_owned()).unwrap();
    }

    #[test]
    #[ignore]
    fn write_non_full_buffer() {
        let mut buffer = CircularBuffer::new(2);
        buffer.write("1".to_owned()).unwrap();
    }

    #[test]
    #[ignore]
    fn overwrite() {
        let mut buffer = CircularBuffer::new(1);
        buffer.write("1".to_owned()).unwrap();
        buffer.overwrite("1".to_owned());
    }
}
