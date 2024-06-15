pub struct Triangle([u64; 3]);

// todo: there for sure a more elegant method with combination of `cycle` and `windows`
impl Triangle {
    #[must_use]
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|x| *x < 1) {
            return None;
        }
        let mut v = sides.to_vec();
        v.push(v[0]);
        v.push(v[1]);
        // 1, 2, 3, 1, 2
        for i in 0..=2 {
            if v[i] + v[i + 1] < v[i + 2] {
                return None;
            }
        }

        Some(Triangle(sides))
    }

    #[must_use]
    pub fn is_equilateral(&self) -> bool {
        // if the user `build` `Triangle` we already checked that it's not 0
        self.0
            .iter()
            .skip(1)
            .all(|x| x == self.0.first().unwrap_or(&0))
    }

    #[must_use]
    pub fn is_scalene(&self) -> bool {
        let mut v = self.0.to_vec();
        v.push(self.0[0]);
        v.windows(2).all(|s| s[0] != s[1])
    }

    #[must_use]
    pub fn is_isosceles(&self) -> bool {
        let mut v = self.0.to_vec();
        v.push(self.0[0]);
        v.windows(2).any(|s| s[0] == s[1])
    }
}
