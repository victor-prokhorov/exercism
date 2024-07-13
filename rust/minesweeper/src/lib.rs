#![feature(test)]
extern crate test;

// kudos to the author for this offsets idea
// https://exercism.org/tracks/rust/exercises/minesweeper/solutions/kstep

// i want to store and reuse offsets for subsequent rows
const HI_OFFSETS: &[(isize, isize)] = &[(-1, -1), (-1, 0), (-1, 1)];
const MI_OFFSETS: &[(isize, isize)] = &[(0, -1), (0, 0), (0, 1)];
const LO_OFFSETS: &[(isize, isize)] = &[(1, -1), (1, 0), (1, 1)];

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

#[derive(Default)]
struct Cell {
    hi: u8,
    mi: u8,
    lo: u8,
}

impl Cell {
    fn total(&self) -> u8 {
        self.hi + self.mi + self.lo
    }
}

pub fn annotate(field: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(field.len());
    let mut prev_cells: Vec<Cell> = Vec::new();
    for (i, r) in field.iter().enumerate() {
        let mut s = String::with_capacity(field.first().unwrap().len());
        let r = r.as_bytes();
        for (j, x) in r.iter().enumerate() {
            let total = if let Some(cell) = prev_cells.get_mut(j) {
                cell.hi = cell.mi;
                cell.mi = cell.lo;
                cell.lo = 0;
                for offset in LO_OFFSETS {
                    cell.lo += proc(field, (i, j), *offset);
                }
                cell.total()
            } else {
                let mut cell = Cell::default();
                for offset in HI_OFFSETS {
                    cell.hi += proc(field, (i, j), *offset);
                }
                for offset in MI_OFFSETS {
                    cell.mi += proc(field, (i, j), *offset);
                }
                for offset in LO_OFFSETS {
                    cell.lo += proc(field, (i, j), *offset);
                }
                let total = cell.total();
                prev_cells.push(cell);
                total
            };
            s.push(match x {
                b' ' if total == 0 => ' ',
                b' ' => char::from(b'0' + total),
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
        // iteration    4          3,272.93 ns/iter (+/- 72.51)
        b.iter(|| black_box(annotate(black_box(&SPARSE_MINEFIELD))));
    }

    #[bench]
    fn bench_dense(b: &mut Bencher) {
        // iteration    2          8,359.85 ns/iter (+/- 131.41)
        // iteration    3          4,666.15 ns/iter (+/- 80.93)
        // iteration    4          3,514.33 ns/iter (+/- 180.53)
        b.iter(|| black_box(annotate(black_box(&DENSE_MINEFIELD))));
    }
}
