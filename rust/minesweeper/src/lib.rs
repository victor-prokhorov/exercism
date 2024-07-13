#![feature(test)]
extern crate test;

// kudos to the author for this offsets idea
// https://exercism.org/tracks/rust/exercises/minesweeper/solutions/kstep
#[rustfmt::skip]
static OFFSETS: &[(isize, isize)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

fn proc(field: &[&str], cord: (usize, usize), offset: (isize, isize)) -> u8 {
    let Ok::<usize, _>(i) = (cord.0 as isize + offset.0).try_into() else {
        return 0;
    };
    let Some(row) = field.get(i) else {
        return 0;
    };
    let Ok::<usize, _>(j) = (cord.1 as isize + offset.1).try_into() else {
        return 0;
    };
    match row.as_bytes().get(j) {
        None => 0,
        Some(&b' ') => 0,
        Some(&b'*') => 1,
        _ => panic!(),
    }
}

#[must_use]
pub fn annotate(field: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(field.len());
    for (i, r) in field.iter().enumerate() {
        let mut s = String::with_capacity(field.first().unwrap().len());
        let r = r.as_bytes();
        for (j, x) in r.iter().enumerate() {
            let mut cell_result = 0;
            for offset in OFFSETS {
                cell_result += proc(field, (i, j), *offset);
            }
            s.push(match x {
                b' ' if cell_result == 0 => ' ',
                b' ' => char::from(b'0' + cell_result),
                _ => '*',
            });
        }
        out.push(s);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::black_box;
    use test::Bencher;

    const SPARSE_MINEFIELD: [&str; 25] = [
        "                         ",
        "    *                    ",
        "                         ",
        "           *             ",
        "                         ",
        "    *                    ",
        "                         ",
        "                  *      ",
        "                         ",
        "                         ",
        "                         ",
        "   *                     ",
        "                         ",
        "                         ",
        "                *        ",
        "                         ",
        "     *                   ",
        "                         ",
        "                         ",
        "                      *  ",
        "                         ",
        "                         ",
        "               *         ",
        "                         ",
        "                         ",
    ];

    const DENSE_MINEFIELD: [&str; 25] = [
        "*    *  *    *  *  *   *  *",
        " *  *    *  *  *    *  *  *",
        "  *  *  *  *    *  *  *    ",
        "*    *  *  *  *  *    *   *",
        "  *  *  *    *   *  *  *  *",
        "*  *    *  *  *    *   *  *",
        "  *    *  *  *    *  *  *  ",
        "*  *  *  *    *  *  *    * ",
        "  *  *  *  *  *    *  *  * ",
        "*    *    *  *  *  *  *    ",
        "  *  *  *    *   *  *  *  *",
        "*  *    *  *  *  *  *    * ",
        "  *  *  *  *  *    *  *  * ",
        "*    *  *  *    *  *  *    ",
        "  *  *  *    *  *  *    * *",
        "*  *    *  *  *    *  *  * ",
        "  *  *  *  *  *  *  *    * ",
        "*    *    *  *  *    *  *  ",
        "  *  *  *    *  *  *  *  * ",
        "*  *  *  *  *  *    *    * ",
        "  *    *  *  *    *  *  * *",
        "*  *    *  *  *  *  *  *   ",
        "  *  *  *  *  *    *    * *",
        "*  *    *  *  *  *  *  *  *",
        "  *  *  *    *   *  *    * ",
    ];

    #[bench]
    fn bench_sparse(b: &mut Bencher) {
        // iteration    2          7,704.31 ns/iter (+/- 164.31)
        // iteration    3          3,963.78 ns/iter (+/- 133.49)
        b.iter(|| black_box(annotate(black_box(&SPARSE_MINEFIELD))));
    }

    #[bench]
    fn bench_dense(b: &mut Bencher) {
        // iteration    2          8,359.85 ns/iter (+/- 131.41)
        // iteration    3          4,666.15 ns/iter (+/- 80.93)
        b.iter(|| black_box(annotate(black_box(&DENSE_MINEFIELD))));
    }
}
