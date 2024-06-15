pub fn series(digits: &str, len: usize) -> Vec<&str> {
    let mut substrings = Vec::with_capacity(digits.len() + 1);
    for offset in 0..(digits.len() + 1).saturating_sub(len) {
        substrings.push(&digits[offset..offset + len]);
    }
    substrings
}
