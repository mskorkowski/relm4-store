use std::ops::Add;
use std::ops::Sub;
use std::ops::Deref;

use super::math::Point;

/// Position in the store
#[derive(Clone,Copy,Debug)]
pub struct Position(pub usize);

impl Position {
    /// Converts position into Point
    pub fn to_point(self) -> Point {
        Point::new(self.0)
    }

    
    pub fn get(&self) -> usize {
        self.0
    }
}

impl Deref for Position {
    type Target = usize;

    fn deref(&self) -> &usize {
        &self.0
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position(self.0 + rhs.0)
    }
}

impl Add<usize> for Position {
    type Output = Position;

    fn add(self, rhs: usize) -> Self::Output {
        Position(self.0 + rhs)
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position(self.0 - rhs.0)
    }
}

impl Sub<usize> for Position {
    type Output = Position;

    fn sub(self, rhs: usize) -> Self::Output {
        Position(self.0 - rhs)
    }
}