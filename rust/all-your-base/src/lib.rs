use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

/// # Number Conversion Algorithms
///
/// <http://www.cs.trincoll.edu/~ram/cpsc110/inclass/conversions.html>
///
/// ## Algorithm to Convert From Any Base to Base 10 Decimal
/// 1.  Let n be the number of digits in the number. For example, 104 has 3 digits, so n=3.
/// 2.  Let b be the base of the number. For example, 104 is decimal so b = 10.
/// 3.  Let s be a running total, initially 0.
/// 4.  For each digit in the number, working left to right do:  
///         Subtract 1 from n.  
///         Multiply the digit times bn and add it to s.
/// 5.  When your done with all the digits in the number, its decimal value will be s
///
/// ## Algorithm to Convert From Decimal To Another Base
///
/// 1.  Let n be the decimal number.
/// 2.  Let m be the number, initially empty, that we are converting to. We'll be composing it right to left.
/// 3.  Let b be the base of the number we are converting to.
/// 4.  Repeat until n becomes 0  
///         Divide n by b, letting the result be d and the remainder be r.  
///         Write the remainder, r, as the leftmost digit of b.  
///         Let d be the new value of n.  
pub fn convert(digits: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut n = digits.len() as u32;
    let mut s = 0;

    for i in digits.iter() {
        if *i > from_base - 1 {
            return Err(Error::InvalidDigit(2));
        }

        n -= 1;
        s += i * from_base.pow(n);
    }

    let mut m = VecDeque::new();

    while s != 0 {
        let d = s / to_base;
        let r = s % to_base;
        m.push_front(r);
        s = d;
    }

    if m.is_empty() {
        m.push_front(0);
    }

    Ok(m.into())
}
