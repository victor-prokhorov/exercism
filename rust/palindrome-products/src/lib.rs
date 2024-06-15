/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn is_palindrome(value: u64) -> bool {
        // if (0..=9).contains(&value) {
        //     return true;
        // }
        // let mut s = value.to_string().chars().collect::<Vec<_>>();
        // if s.len() % 2 != 0 {
        //     s.remove(s.len() / 2);
        // }
        // let (l, r) = s.split_at(s.len() / 2);
        // return l.iter().zip(r.iter().rev()).all(|(l, r)| l == r);
        let mut rev = 0;
        let mut val = value;
        // 913|319
        //
        while val > 0 {
            //               get last digit
            rev = rev * 10 + val % 10;
            val /= 10; // remove last digit
        }
        rev == value
    }

    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if Self::is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut all_possible = Vec::new();
    for i in min..=max {
        for j in min..=max {
            if Palindrome::is_palindrome(i * j) {
                all_possible.push(i * j);
            }
        }
    }
    all_possible.sort();
    let (l, r) = (*all_possible.first()?, *all_possible.last()?);
    Some((Palindrome(l), Palindrome(r)))
}
