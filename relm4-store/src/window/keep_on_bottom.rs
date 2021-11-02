use crate::Range;
use crate::math::Point;

use super::WindowBehavior;
use super::WindowTransition;

/// Locks the view to the bottom (last page) of the store view
/// 
/// If you implement kind of logging tool it might be a thing.
/// For example if you implement git client list of git command
/// issued might use this window behavior.
pub struct KeepOnBottom {}

impl WindowBehavior for KeepOnBottom {
    /// If insert is out of range it's ignored. Otherwise
    /// insert left is returned since it's only direction in
    /// which there are data
    fn insert(r: &Range, p: &Point) -> WindowTransition {
        if p < r.start() {
            WindowTransition::Identity
        }
        else {
            WindowTransition::InsertLeft{
                pos: p.value(),
                by: 1,
            }
        }
    }

    /// If removal is out of range, it's ignored. Otherwise
    /// remove left is returned since all possible data can
    /// only come from left side
    fn remove(r: &Range, p: &Point) -> WindowTransition {
        if p < r.start() {
            WindowTransition::Identity
        }
        else {
            WindowTransition::RemoveLeft{
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