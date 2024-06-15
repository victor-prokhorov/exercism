#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    #[must_use]
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..=7).contains(&rank) && (0..=7).contains(&file) {
            return Some(Self(rank, file));
        }

        None
    }
}

impl Queen {
    #[must_use]
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    #[must_use]
    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.0 .0 == other.0 .0
            || self.0 .1 == other.0 .1
            || (self.0 .0 - other.0 .0).abs() == (self.0 .1 - other.0 .1).abs()
        {
            return true;
        }

        false
    }
}
