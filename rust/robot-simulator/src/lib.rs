#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    #[must_use]
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            position: (x, y),
            direction: d,
        }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.direction {
            Direction::North => self.position = (self.position.0, self.position.1 + 1),
            Direction::East => self.position = (self.position.0 + 1, self.position.1),
            Direction::South => self.position = (self.position.0, self.position.1 - 1),
            Direction::West => self.position = (self.position.0 - 1, self.position.1),
        }
        self
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            match c {
                'A' => self = self.advance(),
                'R' => self = self.turn_right(),
                'L' => self = self.turn_left(),
                _ => (),
            }
        }
        self
    }

    #[must_use]
    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    #[must_use]
    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
