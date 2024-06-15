#[must_use]
pub fn is_armstrong_number(n: u32) -> bool {
    if n < 10 {
        return true;
    }

    let count = (n.ilog10() + 1) as u32;
    let mut candidate = n;
    let mut s: u32 = 0;

    while candidate > 0 {
        let Some(result) = s.checked_add((candidate % 10).pow(count)) else {
            return false;
        };
        s = result;
        candidate /= 10;
    }

    s == n
}
