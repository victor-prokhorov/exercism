#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

struct BucketState {
    bucket: Bucket,
    level: u8,
    capacity: u8,
}

impl BucketState {
    fn new(bucket: Bucket, capacity: u8) -> Self {
        Self {
            bucket,
            capacity,
            level: 0,
        }
    }

    fn is_full(&self) -> bool {
        self.level == self.capacity
    }

    fn fill(&mut self) {
        self.level = self.capacity;
    }

    fn is_empty(&self) -> bool {
        self.level == 0
    }

    fn empty(&mut self) {
        self.level = 0;
    }

    fn remaining(&self) -> u8 {
        self.capacity - self.level
    }

    fn pour_into(&mut self, other: &mut Self) {
        let amount = std::cmp::min(self.level, other.remaining());
        self.level -= amount;
        other.level += amount;
    }
}

#[must_use]
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal % gcd(capacity_1, capacity_2) != 0 {
        return None;
    }

    let mut moves = 0;

    let (mut src, mut dest) = match start_bucket {
        Bucket::One => (
            BucketState::new(Bucket::One, capacity_1),
            BucketState::new(Bucket::Two, capacity_2),
        ),
        Bucket::Two => (
            BucketState::new(Bucket::Two, capacity_2),
            BucketState::new(Bucket::One, capacity_1),
        ),
    };

    loop {
        moves += 1;

        if src.is_empty() || src.capacity == goal {
            src.fill();
        } else if dest.capacity == goal {
            dest.fill();
        } else if dest.is_full() {
            dest.empty();
        } else {
            src.pour_into(&mut dest);
        }

        if dest.level == goal {
            break Some(BucketStats {
                goal_bucket: dest.bucket,
                other_bucket: src.level,
                moves,
            });
        }

        if src.level == goal {
            break Some(BucketStats {
                goal_bucket: src.bucket,
                other_bucket: dest.level,
                moves,
            });
        }
    }
}

#[must_use]
fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
