use std::borrow::Borrow;
// #[cfg(feature = "io")]
// BufWriter
use std::io::{Read, Result, Write};
use std::iter::Cycle;
use std::slice::Iter;

#[derive(Clone)] // used in tests
pub struct Xorcism<'a> {
    key: Cycle<Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    pub fn new<T>(key: &'a T) -> Xorcism<'a>
    where
        T: AsRef<[u8]> + ?Sized,
    {
        Xorcism {
            // iterator on immutable value
            // when we iterate we mutate the iterator not the value
            key: key.as_ref().iter().cycle(),
        }
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut()
            // that's why we have a mutable iterator here
            .zip(&mut self.key)
            // and immutable value on the right here
            .for_each(|(l, r): (&mut u8, &u8)| {
                *l ^= r;
            });
    }

    // pub fn munge<T>(&mut self, data: T) -> impl Iterator<Item = u8> {
    pub fn munge<'b, Data>(&'b mut self, data: Data) -> CombinedIterator<'a, 'b, Data::IntoIter>
    where
        Data: IntoIterator,
        // Data::IntoIter: 'b,
        Data::Item: Borrow<u8>,
        // <Data as IntoIterator>::IntoIter: 'a + 'b,
    {
        CombinedIterator {
            data: data.into_iter(),
            key: &mut self.key,
        }
    }

    // #[cfg(feature = "io")]
    pub fn reader(self, stream_reader: impl Read + 'a) -> impl Read + 'a {
        XorcismStream {
            xorcism: self,
            io: stream_reader,
        }
    }
    // #[cfg(feature = "io")]
    pub fn writer(self, stream_writer: impl Write + 'a) -> impl Write + 'a {
        XorcismStream {
            xorcism: self,
            // io: BufWriter::new(stream_writer),
            io: stream_writer,
        }
    }
}

// todo: this is the easiest solution which group them outside
// there a way to write this without adapter which merge two iterators
pub struct CombinedIterator<'a, 'b, T> {
    key: &'b mut Cycle<Iter<'a, u8>>,
    data: T,
}

impl<'a, 'b, T, U> Iterator for CombinedIterator<'a, 'b, T>
where
    T: Iterator<Item = U>,
    U: Borrow<u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self
            .data
            .next()
            .map(|b| b.borrow() ^ self.key.next().unwrap());
        r
    }
}

// #[cfg(feature = "io")]
pub struct XorcismStream<'a, T> {
    xorcism: Xorcism<'a>,
    io: T,
}

// #[cfg(feature = "io")]
impl<'a, T: Read> Read for XorcismStream<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.io.read(buf).and_then(|n| {
            self.xorcism.munge_in_place(buf);
            Ok(n)
        })
    }
}

// #[cfg(feature = "io")]
impl<'a, T: Write> Write for XorcismStream<'a, T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut n: usize = 0;
        let mut it = self.xorcism.munge(buf);
        while let Some(byte) = it.next() {
            // n += self.io.write(&[byte])?;
            self.io.write(&[byte])?;
            // we write byte by byte anyway
            // todo: wasnt able to implement BufWriter to write buf of bytes in on go
            // without allocating via Vec to the head
            n += 1;
        }
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.io.flush()
    }
}
