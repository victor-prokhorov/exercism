pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut i = 0;
    let mut n = n;

    while n != 1 {
        match n % 2 {
            0 => n /= 2,
            _ => n = u64::checked_add(u64::checked_mul(3, n)?, 1)?,
        }
        i += 1;
    }

    Some(i)
}
