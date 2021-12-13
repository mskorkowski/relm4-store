use reexport::log;

use std::cmp::min;

use crate::Range;
use crate::math::Point;

use super::StoreState;
use super::WindowBehavior;
use super::WindowTransition;

/// Implements window in such a way that window keeps data in the
/// view as much as possible
///
/// This window tries to keep given set of data in view. So if
/// you add 100 of elements before the view range it will keep
/// current dataset in view
#[derive(Debug)]
pub struct ValueTrackingWindow{}

impl WindowBehavior for ValueTrackingWindow {
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
    /// ```text
    /// half = (r.start + r.end)/2
    /// If p < half then remove first element.
    ///   Reduce index of elements from start to p by 1, insert p 
    /// Else
    ///   Increase index of elements from `p` by 1. Insert p at `p.pos`
    /// ```
    fn insert(state: &StoreState<'_>, p: &Point) -> WindowTransition {
        if p >= state.page.end() {
            log::error!("Insert after");
            WindowTransition::Identity
        }
        else if p <= state.page.start() && state.page.len() == state.view {
            log::error!("Insert before");
            WindowTransition::SlideRight(1)
        }
        else {
            let half: usize = (state.page.start() + state.page.end())/2;
            if p < &half {
                log::error!("Insert first half");
                WindowTransition::InsertLeft{
                    pos: p.value(),
                    by: 1,
                }
            }
            else {
                log::error!("Insert second half");
                WindowTransition::InsertRight{
                    pos: p.value(),
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
    /// ```text
    /// half = (r.start + r.end)/2
    /// If p < half then remove first element.
    ///   Reduce index of elements from start to p by 1, insert p 
    /// Else
    ///   Increase index of elements from `p` by 1. Insert p at `p`
    /// ```
    fn remove(state: &StoreState<'_>, p: &Point) -> WindowTransition {
        if p < state.page.start() || p >= state.page.end() {
            WindowTransition::Identity
        }
        else {
            let half: usize = (state.page.start() + state.page.end())/2;
            if p < &half {
                WindowTransition::RemoveLeft{
                    pos: p.value() - state.page.start(),
                    by: 1,
                }
            }
            else {
                WindowTransition::RemoveRight{
                    pos: p.value() - state.page.start(),
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
    fn slide(state: &StoreState<'_>, moved: &Range) -> WindowTransition {
        if state.page.end() <= moved.start() {
            WindowTransition::Identity
        }
        else if moved.start() < state.page.start() {
            WindowTransition::SlideRight(moved.len())
        }
        else {
            WindowTransition::InsertRight{
                pos: *moved.start(),
                by: min(
                    moved.len(),
                    state.page.end() - moved.start()
                )
            }
        }
    }
}