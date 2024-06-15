use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut buf = VecDeque::new();
    for value in values.iter() {
        let mut value = *value;
        while value > 0 {
            buf.push_front(u8::try_from(value & 0b1111_1111 | 0b1000_0000).unwrap());
            value >>= 7;
        }
        match buf.back_mut() {
            Some(byte) => *byte &= 0b0111_1111,
            None => buf.push_back(0),
        }
        bytes.extend(&buf);
        buf.clear();
    }
    bytes
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut buf = Vec::new();
    let mut value: u32 = 0;
    for (i, byte) in bytes.iter().enumerate() {
        let is_msb_set = byte & 0b1000_0000 != 0;
        if is_msb_set && i == bytes.len() - 1 {
            return Err(Error::IncompleteNumber);
        }
        let byte: u32 = u32::from(byte & 0b0111_1111);
        if value.leading_zeros() < 7 {
            return Err(Error::Overflow);
        }
        value <<= 7;
        value |= byte;
        if !is_msb_set {
            buf.push(value);
            value = 0;
        }
    }
    if value > 0 {
        buf.push(value);
    }
    Ok(buf)
}
