#![warn(clippy::pedantic)]

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}
impl<'a> HighScores<'a> {
    #[must_use]
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    #[must_use]
    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    #[must_use]
    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    #[must_use]
    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    #[must_use]
    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut s = self.scores.to_vec();
        s.sort_unstable();
        s.iter().copied().rev().take(3).collect()
    }
}
