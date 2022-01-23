use crate::Range;
use crate::math::Point;

use super::StoreState;
use super::WindowBehavior;
use super::WindowTransition;

/// Locks the view to the bottom (last page) of the store view
/// 
/// If you implement kind of logging tool it might be a thing.
/// For example if you implement git client list of git command
/// issued might use this window behavior.
#[derive(Debug)]
pub struct KeepOnBottom {}

impl WindowBehavior for KeepOnBottom {
    fn insert(state: &StoreState<'_>, p: &Point) -> WindowTransition {
        if p < state.page.start() {
            WindowTransition::TransitionRight(1)
        }
        else if p >= state.page.end() {
            //p is not visible already, then slide by 1 to the right
            WindowTransition::SlideRight(1)
        }
        else {
            WindowTransition::InsertLeft{
                pos: p.value(),
                by: 1,
            }
        }
    }

    fn remove(state: &StoreState<'_>, p: &Point) -> WindowTransition {
        if p < state.page.start() {
            WindowTransition::TransitionLeft(1)
        }
        else if p >= state.page.end() {
            WindowTransition::Identity
        }
        else {
            WindowTransition::RemoveLeft{
                pos: p.value() +1,
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