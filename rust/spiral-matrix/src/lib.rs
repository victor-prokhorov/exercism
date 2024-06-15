#[must_use]
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return Vec::new();
    }
    let size: usize = size as usize;
    let mut v = vec![vec![0; size]; size];
    let mut n: u32 = 1;
    let limit = (size).pow(2) as u32;
    let mut width: usize = size;
    let mut height: usize = size;
    let mut offset = (
        0, // left-to-right
        0, // top-to-bottom
        0, // right-to-left
        0, // bottom-to-top
    );
    'outer: loop {
        for i in 0..width {
            v[offset.0][offset.2 + i] = n;
            if n == limit {
                break 'outer;
            }
            n += 1;
        }
        offset.0 += 1;
        height -= 1;
        for i in 0..height {
            v[offset.0 + i][size - 1 - offset.1] = n;
            if n == limit {
                break 'outer;
            }
            n += 1;
        }
        offset.1 += 1;
        width -= 1;
        for i in (0..width).rev() {
            v[size - 1 - offset.2][i + offset.2] = n;
            if n == limit {
                break 'outer;
            }
            n += 1;
        }
        offset.2 += 1;
        height -= 1;
        for i in (0..height).rev() {
            v[offset.0 + i][offset.3] = n;
            if n == limit {
                break 'outer;
            }
            n += 1;
        }
        offset.3 += 1;
        width -= 1;
    }
    v
}
