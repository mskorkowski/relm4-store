//! Contains mathematics required to operate data store
//! 
//! This module contains structures and traits which allow to compute which part of store view
//! should be modified so the amount of changes is minimal. 

use std::cmp::min;
use std::cmp::max;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Result;
use std::fmt::Formatter;

/// One dimensional range [start, end)
#[derive(Clone,Copy)]
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
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns smallest value not in range
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns new range which starts at `start - l`
    /// and has a length equal to this
    ///
    /// If move to right would cause the range to move towards negative values, 
    /// returned range will start at 0
    ///
    pub fn to_left(&self, l: usize) -> Range {
        let to_left = min(self.start, l);
        self.slide(self.start() - to_left)
    }

    pub fn to_right(&self, r: usize) -> Range {
        self.slide(self.start() + r)
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
    pub fn new(p: usize) -> Self {
        Self(p)
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

/// Describe transitions in relation to range of items displayed by window
pub enum WindowTransition {
    /// Nothing to do
    Identity,
    /// Insert elements and change indexes to right of new items
    ///
    /// Starting from `pos`, insert `by` elements, change indexes of items from `pos` by `idx + by`, drop last `by` elements
    InsertRight{
        pos: usize,
        by:  usize,
    },

    /// Insert elements and change indexes to left of new items
    ///
    /// Drop first `by` elements, change indexes of items from first to `pos` by `idx - by`, insert `by` elements
    InsertLeft{
        pos: usize,
        by: usize,
    },

    /// Remove elements and change indexes to right of new items
    ///
    /// Starting from `pos`, remove `by` elements, change indexes of items from `pos` by `idx - by`, insert last `by` elements
    RemoveRight{
        pos: usize,
        by:  usize,
    },

    /// Remove elements and change indexes to left of new items
    ///
    /// Change indexes of items from first to `pos` by `idx - by`. Insert at the beginning `by` elements.
    /// From pos remove `by` elements
    RemoveLeft{
        pos: usize,
        by: usize,
    },

    /// Move window to the right. No data changes
    SlideRight(usize),

    /// Move window to the left. No data changes
    SlideLeft(usize),

}

pub trait Window {
    /// Computes change in order of indexes of elements on the line due to insert
    fn insert(r: &Range, p: &Point) -> WindowTransition;

    /// Computes change in order of indexes of element on the line due to removal
    fn remove(r: &Range, p: &Point) -> WindowTransition;

    /// Computes change in order of indexes of elements due to moving the range
    fn slide(r: &Range, moved: &Range) -> WindowTransition;
}

/// Implements window in such a way that window keeps it's position
/// in dataset
///
/// This window will be stable in terms of pagination. If your store
/// is at third page of data it will stay there as much as possible
pub struct PositionTrackingWindow{}

/// Implements window in such a way that window keeps data in the
/// view as much as possible
///
/// This window tries to keep given set of data in view. So if
/// you add 100 of elements before the view range it will keep
/// current dataset in view
pub struct ValueTrackingWindow{}

impl Window for PositionTrackingWindow  {
    /// Computes change in order of indexed elements on the line due to addition of element
    /// 
    /// There are 3 cases to take care of
    ///
    /// ## Case 1: Value has been inserted before range
    /// 
    /// Take one item from left so it can become first, first item in range 
    /// becomes second, second becomes third, ..., N-1 item becomes N, Nth item is dropped 
    /// and value is dropped
    ///
    /// ## Case 2: Value has been inserted after range
    ///
    /// Nothing changes and value is dropped
    ///
    /// ## Case 3: Value is inserted in the range
    ///
    /// Position of value in range `P = p.pos - range.start`. 
    /// Indexes from `[range.start, P)` are kept. Value is inserted at P.
    /// Indexes from `[P+1, range.end)` are increased
    fn insert(r: &Range, p: &Point) -> WindowTransition {
        if p.0 >= r.end { // Case 2
            WindowTransition::Identity
        } 
        else if p.0 < r.start { // Case 1
            WindowTransition::InsertRight {
                pos: r.start,
                by: 1,
            }
        } 
        else { // Case 3
            WindowTransition::InsertRight {
                pos: p.0,
                by: 1,
            }
        }
    }

    /// Computes change in order of indexed elements on the line due to removal of element
    ///
    /// There are 3 cases to take care of
    ///
    /// ## Case 1: Remove element before range
    ///
    /// Remove first element, change index of all elements `idx - by`. Insert `by` elements at the end
    ///
    /// ## Case 2: Remove element after range
    ///
    /// Nothing to do
    ///
    /// ## Case 3: Remove element in the range
    ///
    /// Position of value in range is `P = p.pos - range.start`.
    /// Indexes from `[range.start, P) are kept. Value is removed at P.
    /// Indexes from `[P+1, range.end) are decreased by 1.
    fn remove(r: &Range, p: &Point) -> WindowTransition {
        if p.0 >= r.end { // Case 2
            WindowTransition::Identity
        }
        else if p.0 < r.start { // Case 1
            WindowTransition::RemoveRight {
                pos: r.start,
                by: 1,
            }
        }
        else { // Case 3
            WindowTransition::RemoveRight {
                pos: p.0 - r.start,
                by: 1,
            }
        }
    }

    /// Computes change in order of indexes of elements due to moving the range
    ///
    /// There are 6 cases to take care of
    ///
    /// ## Case 1: moved range is fully before the reference range
    ///
    /// It's equivalent to adding `moved.len()` elements before the `r`
    ///
    /// ## Case 2: moved range is fully after the reference range
    ///
    /// Nothing to do
    ///
    /// ## Case 3: moved range contains the r
    ///
    /// It's equivalent of inserting `r.len()` elements at `r.start()`
    ///
    /// ## Case 4: moved range contains start but not end
    ///
    /// It's equivalent of inserting `moved.end - r.start` elements at `r.start()`
    ///
    /// ## Case 5: moved range contains end but not start
    ///
    /// It's equivalent of inserting `r.end - moved.start` elements at `moved.start()`
    ///
    /// ## Case 6: moved range is subset of `r`
    ///
    /// It's equivalent of inserting `moved.len()` at `moved.start()`
    fn slide(r: &Range, moved: &Range) -> WindowTransition {
        if r.end <= moved.start { // Case 2
            WindowTransition::Identity
        }
        else if moved.start <= r.start && r.end <= moved.end { // Case 3
            WindowTransition::InsertRight{
                pos: 0,
                by: min(moved.len(), r.len())
            }
        } else if moved.start < r.start { // Case 1 and 4
            WindowTransition::InsertRight{
                pos: 0,
                by: min(
                    min(r.start, moved.end) - moved.start,
                    r.len()
                )
            }
        } else { // Case 5 and 6
            WindowTransition::InsertRight{
                pos: moved.start - r.start,
                by: min(
                    min(r.end, moved.end) - moved.start,
                    r.len()
                )
            }
        }
    }
}

impl Window for ValueTrackingWindow {
    /// Computes change in order of indexes of elements on the line
    ///
    /// There are 3 cases to handle
    ///
    /// ## Case 1: Element inserted before `range.start`
    ///
    /// Slide by 
    ///
    /// ## Case 2: Element inserted after range
    ///
    /// Nothing to do
    ///
    /// ## Case 3: Element inserted inside the range
    ///
    /// ```
    /// half = (r.start + r.end)/2
    /// If p.pos < half then remove first element.
    ///   Reduce index of elements from start to p.pos by 1, insert p 
    /// Else
    ///   Increase index of elements from `p.pos` by 1. Insert p at `p.pos`
    /// ```
    fn insert(r: &Range, p: &Point) -> WindowTransition {
        if p.0 >= r.end {
            WindowTransition::Identity
        }
        else if p.0 < r.start {
            WindowTransition::SlideRight(1)
        }
        else {
            let half: usize = (r.start + r.end)/2;
            if p.0 < half {
                WindowTransition::InsertLeft{
                    pos: p.0 - r.start,
                    by: 1,
                }
            }
            else {
                WindowTransition::InsertRight{
                    pos: p.0 - r.start,
                    by: 1,
                }
            }

        }
    }

    /// Computes change in order of indexes of element on the line
    ///
    /// There are 3 cases to handle
    ///
    /// ## Case 1: Element removed before `range.start`
    ///
    /// Nothing to do
    ///
    /// ## Case 2: Element removed after range
    ///
    /// Nothing to do
    ///
    /// ## Case 3: Element removed inside the range
    ///
    /// ```
    /// half = (r.start + r.end)/2
    /// If p.pos < half then remove first element.
    ///   Reduce index of elements from start to p.pos by 1, insert p 
    /// Else
    ///   Increase index of elements from `p.pos` by 1. Insert p at `p.pos`
    /// ```
    fn remove(r: &Range, p: &Point) -> WindowTransition {
        if p.0 < r.start || p.0 >= r.end {
            WindowTransition::Identity
        }
        else {
            let half: usize = (r.start + r.end)/2;
            if p.0 < half {
                WindowTransition::RemoveLeft{
                    pos: p.0 - r.start,
                    by: 1,
                }
            }
            else {
                WindowTransition::RemoveRight{
                    pos: p.0 - r.start,
                    by: 1,
                }
            }
        }
    }

    /// Computes change in order of indexes of elements due to moving the range
    ///
    /// There are 3 cases to consider
    ///
    /// ## Case 1: Moved range start before current range starts
    ///
    /// No data change, just slide the window to the right by the moved range size
    ///
    /// ## Case 2: Moved range starts after current range ends
    ///
    /// Nothing to do
    ///
    /// ## Case 3: Moved range starts between current range start and end
    ///
    /// Insert at moved range start, at most current range end - moved range start items
    ///
    /// Nothing to do, cos we can move the window around and keep the data visible
    fn slide(r: &Range, moved: &Range) -> WindowTransition {
        if r.end <= moved.start {
            WindowTransition::Identity
        }
        else if moved.start < r.start {
            WindowTransition::SlideRight(moved.len())
        }
        else {
            WindowTransition::InsertRight{
                pos: moved.start,
                by: min(
                    moved.len(),
                    r.end - moved.start
                )
            }
        }
    }
}