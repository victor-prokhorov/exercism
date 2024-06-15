use std::collections::VecDeque;

pub struct CircularBuffer<T> {
    buffer: VecDeque<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: VecDeque::with_capacity(capacity),
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        (!self.is_full())
            .then(|| self.buffer.push_back(_element))
            .ok_or(Error::FullBuffer)
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.buffer.pop_front().ok_or(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn is_full(&self) -> bool {
        self.buffer.len() == self.buffer.capacity()
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.is_full() {
            self.buffer.pop_front();
        }
        self.buffer.push_back(_element);
    }
}
