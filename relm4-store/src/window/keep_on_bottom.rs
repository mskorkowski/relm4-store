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
    fn insert(r: &Range, p: &Point) -> WindowTransition {
        if p < r.start() {
            WindowTransition::Identity
        }
        else if p >= r.end() {
            //p is not visible already, then slide by 1 to the right
            WindowTransition::SlideRight(1)
        }
        else {
            WindowTransition::InsertRight{
                pos: p.value(),
                by: 1,
            }
        }
    }

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