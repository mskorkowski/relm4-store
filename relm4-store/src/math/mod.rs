//! Contains mathematics required to operate data store
//! 
//! This module contains structures and traits which allow to compute which part of store view
//! should be modified so the amount of changes is minimal.

mod tests;

use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Deref;

/// One dimensional range [start, end)
#[derive(Clone,Copy,PartialEq,Eq)]
pub struct Range{
    start: usize,
    end: usize
}

impl Range {
    /// Creates new instance of Range
    #[must_use]
    pub fn new(a: usize, b: usize) -> Self {
        let start = min(a, b);
        let end   = max(a, b);

        Self {
            start,
            end,
        }
    }

    /// Returns the length of the range
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Checks if range contain any values
    /// 
    /// Range is considered empty if distance between start and end is equal to 0
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Returns new range which has the same size as this but
    /// starts at new position
    pub fn slide(&self, start: usize) -> Range {
        Range{
            start,
            end: start + self.len()
        }
    }

    /// Returns smallest value in the range
    pub fn start(&self) -> &usize {
        &self.start
    }

    /// Returns smallest value not in range
    pub fn end(&self) -> &usize {
        &self.end
    }

    /// Returns new range which starts at `start - l`
    /// and has a length equal to this
    ///
    /// If move to right would cause the range to move towards negative values, 
    /// returned range will start at 0
    pub fn to_left(&self, l: usize) -> Range {
        let to_left = min(self.start, l);
        self.slide(self.start() - to_left)
    }

    /// Returns new range with starts at `start + r`
    pub fn to_right(&self, r: usize) -> Range {
        self.slide(self.start() + r)
    }

    /// Returns common part of the two ranges
    /// 
    /// If there is no common part then it returns `None`.
    /// 
    /// List of cases
    /// | # | Case | Result |
    /// |:-:|:-----|:-------|
    /// | 1 | `other` is strictly to the left of the `self` | None |
    /// | 2 | `other` is strictly to the right of the `self` | None |
    /// | 3 | `other` is fully contained in `self` | Some(other) |
    /// | 4 | `self` is fully contained in `other` | Some(self) |
    /// | 5 | `other.start() <= self.start() < other.end() <= self.end()` | Some(Range(self.start(), other.end())) |
    /// | 6 | `self.start() <= other.start() < self.end() <= other.end()` | Some(Range(other.start(), self.end())) |
    /// 
    pub fn common_part(&self, other: &Range) -> Option<Range> {
        if other.start() >= self.end() || other.end() <= self.start() { //cases 1 and 2
            None
        }
        else if self.start() <= other.start() { // cases 3 or 6
            if other.end() <= self.end() { // case 3
                Some(*other)
            }
            else { // case 6
                Some(Range::new(*other.start(), *self.end()))
            }
        }
        else { // cases 4 or 5
            if self.end() <= other.end() { // case 4
                Some(*self)
            }
            else { // case 5
                Some(Range::new(*self.start(), *other.end()))
            }
        }
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Range({}..{})", self.start, self.end)
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Range({}..{})", self.start, self.end)
    }
}

/// One dimensional point
pub struct Point(usize);

impl Point {
    /// returns new instance of the point
    pub fn new(p: usize) -> Self {
        Self(p)
    }

    /// returns value of the point
    pub fn value(&self) -> usize {
        self.0
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Point({})", self.0)
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Point({})", self.0)
    }
}

impl Deref for Point {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<usize> for Point {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<usize> for Point {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        Some(self.0.cmp(other))
    }
}