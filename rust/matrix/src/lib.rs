use std::fmt::Debug;
use std::str::FromStr;

pub type Matrix = M<u32>;

/// `Matrix::new` O(n) space complexity
/// `Matrix::column` O(n) time complexity
#[cfg(all(feature = "approach1", not(feature = "approach2")))]
pub struct M<T> {
    v: Vec<Vec<T>>,
}

/// `Matrix::new` O(2*n) space complexity
/// `Matrix::column` O(1) time complexity
#[cfg(feature = "approach2")]
pub struct M<T> {
    r: Vec<Vec<T>>,
    c: Vec<Vec<T>>,
}

pub trait Ops<T>: Common<T> {
    fn new(input: &str) -> Self;
    fn row(&self, r: usize) -> Option<Vec<T>>;
    fn column(&self, c: usize) -> Option<Vec<T>>;
}

pub trait Common<T> {
    fn row_from_line(l: &str) -> Vec<T>;
    fn column_from_row(row: &[Vec<T>], c: usize) -> Option<Vec<T>>;
}

impl<T> Common<T> for M<T>
where
    T: Copy + FromStr,
    <T as FromStr>::Err: Debug,
{
    fn row_from_line(l: &str) -> Vec<T> {
        l.split_whitespace().map(|x| x.parse().unwrap()).collect()
    }
    fn column_from_row(row: &[Vec<T>], c: usize) -> Option<Vec<T>> {
        if row.first().is_some() && row[0].get(c).is_some() {
            Some(row.iter().map(|row| row.get(c).copied().unwrap()).collect())
        } else {
            None
        }
    }
}

#[cfg(all(feature = "approach1", not(feature = "approach2")))]
impl<T> Ops<T> for M<T>
where
    T: Copy + FromStr,
    <T as FromStr>::Err: Debug,
{
    fn new(input: &str) -> Self {
        Self {
            v: input.lines().map(|l| Self::row_from_line(l)).collect(),
        }
    }
    fn row(&self, r: usize) -> Option<Vec<T>> {
        self.v.get(r - 1).cloned()
    }
    fn column(&self, c: usize) -> Option<Vec<T>> {
        Self::column_from_row(&self.v, c - 1)
    }
}

#[cfg(feature = "approach2")]
impl<T> Ops<T> for M<T>
where
    T: Copy + FromStr,
    <T as FromStr>::Err: Debug,
{
    fn new(input: &str) -> Self {
        let r: Vec<Vec<T>> = input.lines().map(|l| Self::row_from_line(l)).collect();
        Self {
            c: (0usize..)
                .into_iter()
                .take_while(|i| Self::column_from_row(&r, *i).is_some())
                .map(|i| Self::column_from_row(&r, i).unwrap())
                .collect(),
            r,
        }
    }
    fn row(&self, r: usize) -> Option<Vec<T>> {
        self.r.get(r - 1).cloned()
    }
    fn column(&self, c: usize) -> Option<Vec<T>> {
        self.c.get(c - 1).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generic() {
        let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9");
        assert_eq!(matrix.column(1), Some(vec![1, 4, 7]));
        let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9");
        assert_eq!(matrix.column(4), None);
        let matrix: M<i32> = M::new("-1 -2 -3\n-4 -5 -6\n-7 -8 -9");
        assert_eq!(matrix.row(1), Some(vec![-1, -2, -3]));
    }
}
