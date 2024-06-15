use num::{BigInt,Zero};
use std::cmp::Ordering;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

// sign bit, the exponent, and the significand
//
/// <https://ccnmtl.columbia.edu/projects/mmt/frontiers/web/chapter_5/6665.html>
// 4500000 is 4.5 x 10^6, where significand is 4.5 and exponent is 6
// from 4.5 shift right 6 times
// 0.0000456 is 4.56 x 10^-5, where significand is 4.56 and exponent is -5
// from 4.56 shift left 5 times
// "-0.001" is -1 x 10^-3
// but in our decimal we can't use decimal notation itself and since it's a decimal we always will
//
// -1x10^-2 > -1x10^-1
// -0.1x10^-1 > -1x10^-1
// -0.01 > -0.1         -1x10^-2 > -10.x10^-2
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Decimal {
    // we skeep `Sign` num lib will do this we will rely on this impl
    significand: BigInt,
    exponent: isize, // we will only have negative values, and will stop at 0,
                     // 123_000 will remain as is as significand
                     // since we will rely on `BigInt`
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (l, r) = Self::normalize_significand_scale(self, other);
        Some(l.cmp(&r))
    }
}

impl Add for Decimal {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let (l, r) = Self::normalize_significand_scale(&self, &other);
        Self::remove_trailing_zeros(l + r, self.exponent.min(other.exponent))
    }
}
impl Sub for Decimal {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self + Self::remove_trailing_zeros(-other.significand, other.exponent)
    }
}
impl Mul for Decimal {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::remove_trailing_zeros(
            self.significand * other.significand,
            self.exponent + other.exponent,
        )
    }
}
impl Decimal {
    pub fn try_from(input: &str) -> Option<Self> {
        // let input = "45000.000";
        // 0.0000456
        // 456.
        // `BigInt` will deal with leading zeros
        let significand = BigInt::from_str(input.replace('.', "").as_str()).unwrap();
        // 9 - 1 - 1
        // we can immediately change the sign since
        // we try to count number of digits _post_ '.'
        let exponent = if let Some(index) = input.find('.') {
            -((input.len() - 1 - index) as isize)
        } else {
            0
        };
        Some(Decimal::remove_trailing_zeros(significand, exponent))
        // remove trailing zeros from fractional part and ajdust exponent
        // while &significand % 10 == Zero::zero() && exponent != Zero::zero() {
        // dbg!(&significand,exponent);
        // let mut e = BigInt::from(exponent);
        // Some(Self {
        //     significand,
        //     exponent, // we can't have positive exponent
        //               // 45_000 will remain 45_000 x 10^0
        //               // in our notation
        // })
    }
    // strip trailing zero we use it when we craete a new value
    pub fn remove_trailing_zeros(mut significand: BigInt, mut exponent: isize) -> Self {
        while &significand % 10 == Zero::zero() && exponent != 0 {
            significand /= 10;
            exponent += 1;
        }
        Self {
            significand,
            exponent,
        }
    }
    // align we need to exponennt be the same to reason easily
    // in scientific notaion we would align 4.987 but since we are doing decimal ouself we will
    // rather align to the bigest, so 4987.
    fn normalize_significand_scale(l: &Decimal, r: &Decimal) -> (BigInt, BigInt) {
        // todo: remove `clone()` everywhere
        let (l, r) = match l.exponent.cmp(&r.exponent) {
            Ordering::Equal => (l.clone(), r.clone()),
            Ordering::Greater => {
                let mut l = l.clone();
                debug_assert!(l.exponent > r.exponent);
                while l.exponent != r.exponent {
                    l.significand *= 10;
                    l.exponent -= 1;
                }
                (l, r.clone())
            }
            Ordering::Less => {
                let mut r = r.clone();
                debug_assert!(l.exponent < r.exponent);
                while l.exponent != r.exponent {
                    r.significand *= 10;
                    r.exponent -= 1;
                }
                (l.clone(), r)
            }
        };
        (l.significand, r.significand)
    }
}
