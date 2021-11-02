use std::cmp::min;

use crate::Range;
use crate::math::Point;

use super::WindowBehavior;
use super::WindowTransition;

/// Implements window in such a way that window keeps it's position
/// in dataset
///
/// This window will be stable in terms of pagination. If your store
/// is at third page of data it will stay there as much as possible
pub struct PositionTrackingWindow{}

impl WindowBehavior for PositionTrackingWindow  {
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
        if p >= r.end() { // Case 2
            WindowTransition::Identity
        } 
        else if p < r.start() { // Case 1
            WindowTransition::InsertRight {
                pos: *r.start(),
                by: 1,
            }
        } 
        else { // Case 3
            WindowTransition::InsertRight {
                pos: p.value(),
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
        if p >= r.end() { // Case 2
            WindowTransition::Identity
        }
        else if p < r.start() { // Case 1
            WindowTransition::RemoveRight {
                pos: *r.start(),
                by: 1,
            }
        }
        else { // Case 3
            WindowTransition::RemoveRight {
                pos: p.value() - r.start(),
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
        if r.end() <= moved.start() { // Case 2
            WindowTransition::Identity
        }
        else if moved.start() <= r.start() && r.end() <= moved.end() { // Case 3
            WindowTransition::InsertRight{
                pos: 0,
                by: min(moved.len(), r.len())
            }
        } else if moved.start() < r.start() { // Case 1 and 4
            WindowTransition::InsertRight{
                pos: 0,
                by: min(
                    min(r.start(), moved.end()) - moved.start(),
                    r.len()
                )
            }
        } else { // Case 5 and 6
            WindowTransition::InsertRight{
                pos: moved.start() - r.start(),
                by: min(
                    min(r.end(), moved.end()) - moved.start(),
                    r.len()
                )
            }
        }
    }
}