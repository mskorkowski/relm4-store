use crate::Range;
use crate::math::Point;

use super::StoreState;
use super::WindowBehavior;
use super::WindowTransition;

/// Locks the view to the top (first page) of the store view
/// 
/// If you implement reporting tool and would like to show
/// top 10 records in the store and nothing else.
#[derive(Debug)]
pub struct KeepOnTop {}

impl WindowBehavior for KeepOnTop {
    fn insert(state: &StoreState<'_>, p: &Point) -> WindowTransition {
        if p >= state.page.end() {
            WindowTransition::Identity
        } 
        else if p < state.page.start() {
            WindowTransition::InsertRight {
                pos: *state.page.start(),
                by: 1,
            }
        } 
        else {
            WindowTransition::InsertRight {
                pos: p.value(),
                by: 1,
            }
        }
    }

    /// If removal is out of range, it's ignored. Otherwise
    /// remove right is returned since all possible data can
    /// only come from right side
    fn remove(state: &StoreState<'_>, p: &Point) -> WindowTransition {
        if p >= state.page.end() {
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
    fn slide(_state: &StoreState<'_>, _moved: &Range) -> WindowTransition {
        WindowTransition::Identity
    }
}