pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 {
            return PascalsTriangle(vec![]);
        }
        let mut vec = vec![vec![1]];
        for _ in 0..row_count - 1 {
            let mut last = vec.last().unwrap().clone();
            let len = last.len();
            last.insert(0, 0);
            last.push(0);
            let mut new = vec![];
            for j in 0..len + 1 {
                // dbg!(last[j], last[j + 1]);
                new.push(last[j] + last[j + 1]);
            }
            // dbg!(&new);
            vec.push(new);
        }
        PascalsTriangle(vec)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}

// [
// [ 0, 1, ]
// [ 0, 1, 1 ]
// [ 1, 2, 1 ]
// ]
