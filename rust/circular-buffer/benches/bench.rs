#![feature(test)]
#![allow(unused)]
extern crate test;

use circular_buffer::CircularBuffer;
use circular_buffer::VecDeque;
use test::{black_box, Bencher};

fn init(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..1000 {
            let mut b = CircularBuffer::<i64>::new(1000);
            black_box(b.clear());
        }
    });
}

fn init_write(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..1000 {
            let mut circular_buffer = CircularBuffer::new(1000);
            black_box(circular_buffer.write(i).unwrap());
        }
    });
}

fn write_clear(b: &mut Bencher) {
    let mut circular_buffer = CircularBuffer::new(1000);
    b.iter(|| {
        for i in 0..1000 {
            black_box(circular_buffer.write(i).unwrap());
            circular_buffer.clear();
        }
    });
}

fn write_read_without_init(b: &mut Bencher) {
    let mut circular_buffer = CircularBuffer::new(1000);
    b.iter(|| {
        for i in 0..1000 {
            black_box(circular_buffer.write(i).unwrap());
        }
        for _ in 0..1000 {
            circular_buffer.read().unwrap();
        }
    });
}

fn write_overwrite_clear(b: &mut Bencher) {
    let mut circular_buffer = CircularBuffer::new(1000);
    b.iter(|| {
        for i in 0..1000 {
            circular_buffer.write(i).unwrap();
        }
        for i in 0..1000 {
            circular_buffer.overwrite(i);
        }
        circular_buffer.clear();
    });
}

#[bench]
fn scenario_manual(b: &mut Bencher) {
    let mut circular_buffer = CircularBuffer::new(1000);

    b.iter(|| {
        for i in 0..1000 {
            circular_buffer.write(i).unwrap();
        }

        for i in 0..1000 {
            circular_buffer.overwrite(i);
        }

        for _ in 0..500 {
            black_box(circular_buffer.read().unwrap());
        }

        for i in 0..500 {
            circular_buffer.write(i).unwrap();
        }

        circular_buffer.clear();
    });
}

#[bench]
fn scenario_vec_deque(b: &mut Bencher) {
    let mut circular_buffer = VecDeque::new(1000);

    b.iter(|| {
        for i in 0..1000 {
            circular_buffer.write(i).unwrap();
        }

        for i in 0..1000 {
            circular_buffer.overwrite(i);
        }

        for _ in 0..500 {
            black_box(circular_buffer.read().unwrap());
        }

        for i in 0..500 {
            circular_buffer.write(i).unwrap();
        }

        circular_buffer.clear();
    });
}
