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

fn proc(mf: &[&str], p: (usize, usize), offset: (isize, isize)) -> usize {
    let i: usize = match (p.0 as isize + offset.0).try_into() {
        Ok(i) => i,
        Err(_) => return 0,
    };
    let r = match mf.get(i) {
        Some(r) => r.as_bytes(),
        None => return 0,
    };
    let j: usize = match (p.1 as isize + offset.1).try_into() {
        Ok(j) => j,
        Err(_) => return 0,
    };
    match r.get(j).as_ref() {
        Some(b' ') | None => 0,
        Some(b'*') => 1,
        _ => panic!("{:?}", r[j]),
    }
}

#[must_use]
pub fn annotate(mf: &[&str]) -> Vec<String> {
    let mut out: Vec<Vec<char>> = Vec::new();
    for (i, r) in mf.iter().enumerate() {
        out.push(Vec::new());
        let r = r.as_bytes();
        for (j, _) in r.iter().enumerate() {
            let mut total = 0;
            for offset in OFFSETS {
                let cell_result = proc(mf, (i, j), *offset) as u8;
                total += cell_result;
            }
            let c = (b'0' + total) as char;
            out.last_mut()
                .unwrap()
                .push(if mf[i].as_bytes()[j] == b' ' {
                    if total > 0 {
                        c
                    } else {
                        ' '
                    }
                } else {
                    '*'
                });
        }
    }
    out.into_iter().map(|r| r.into_iter().collect()).collect()
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
        b.iter(|| black_box(annotate(black_box(&SPARSE_MINEFIELD))));
    }

    #[bench]
    fn bench_dense(b: &mut Bencher) {
        // iteration    2          8,359.85 ns/iter (+/- 131.41)
        b.iter(|| black_box(annotate(black_box(&DENSE_MINEFIELD))));
    }
}
