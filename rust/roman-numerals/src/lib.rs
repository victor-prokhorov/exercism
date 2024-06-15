use std::fmt::{Display, Formatter, Result};

const R: [(u32, char, u32, char); 7] = [
    (1000, 'M', 100, 'C'), // don't need to be hardcoded
    (500, 'D', 100, 'C'),
    (100, 'C', 10, 'X'),
    (50, 'L', 10, 'X'),
    (10, 'X', 1, 'I'),
    (5, 'V', 1, 'I'),
    (1, 'I', 0, 'I'),
];
// In the Roman numeral system, the symbols I, V, X, L,    C,   D,  and M stand respectively
//                                      for 1, 5, 10, 50, 100, 500, and 1,000
//  are written by expressing each digit separately starting with the left most digit and skipping any digit with a value of zero.
// 59, L, 9 rem,
pub struct Roman {
    romans: String,
    hindu_arabic: u32,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut romans = String::new();
        let mut n = self.hindu_arabic;
        // dbg!(n);
        loop {
            // 2k is MM so just loop until met dividor don't nest
            // find index that feed, one at a time
            // let mut x: u32 = 0;
            let mut i = 0;
            // 93 -> 100 - 10 + 1 + 1 + 1
            'w: while i < R.len() && n != 0 {
                // dbg!(R[i].0);
                // 1000, 100, 100
                // 900 - CM - M-C = 1000-100
                // we are at 93 -> 90 (10 ^ 1 ) : 100 (10 ^ 2)
                let rn = R[i].0; // 100

                if n / R[i].0 > 0 {
                    dbg!("second");
                    let x = n / R[i].0;
                    dbg!(x);
                    for _ in 0..x {
                        romans.push(R[i].1);
                    }
                    n -= x * R[i].0;
                    break 'w;
                }
                if n / (rn - R[i].2) > 0 {
                    dbg!(rn - R[i].2);
                    romans.push(R[i].3);
                    romans.push(R[i].1);
                    n -= rn - R[i].2;
                    break 'w;
                }
                i += 1;
            }
            dbg!(&romans, n);
            if n == 0 {
                break;
            }
        }
        write!(f, "{}", romans)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        // unimplemented!("Construct a Roman object from the '{num}' number");
        Self {
            hindu_arabic: num,
            romans: "I".to_owned(),
        }
    }
}
