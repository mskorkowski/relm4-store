use crate::Range;
use crate::math::Point;

use super::WindowBehavior;
use super::WindowTransition;

/// Locks the view to the top (first page) of the store view
/// 
/// If you implement reporting tool and would like to show
/// top 10 records in the store and nothing else.
pub struct KeepOnTop {}

impl WindowBehavior for KeepOnTop {
    /// If insert is out of range it's ignored. Otherwise
    /// insert right is returned since it's only direction in
    /// which there are data
    fn insert(r: &Range, p: &Point) -> WindowTransition {
        if p >= r.end() {
            WindowTransition::Identity
        }
        else {
            WindowTransition::InsertRight{
                pos: p.value(),
                by: 1,
            }
        }
    }

    /// If removal is out of range, it's ignored. Otherwise
    /// remove right is returned since all possible data can
    /// only come from right side
    fn remove(r: &Range, p: &Point) -> WindowTransition {
        if p >= r.end() {
            WindowTransition::Identity
        }
        else {
            WindowTransition::RemoveRight{
                pos: p.value(),
                by: 1,
            }
        }
    }

    /// Does nothing. You can't slide away from the top of the window while
    /// use this view
    fn slide(_r: &Range, _moved: &Range) -> WindowTransition {
        WindowTransition::Identity
    }
}