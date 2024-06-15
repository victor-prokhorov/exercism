pub fn egg_count(mut x: u32) -> usize {
    let mut cnt = 0;
    let mut y = 0;
    while x > 0 {
        cnt += 1;
        y = x - 1;
        x = x & y;
    }
    cnt
}
