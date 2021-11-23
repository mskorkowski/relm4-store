//! Holds implementation for basic view window behaviors
//! 
//! When you display the data to the user different kind of behavior of the view is desired
//! in presence of the new data
//! 
//! If you use pagination you should use [PositionTrackingWindow]. Together they will never
//! show data belonging to two "pages" a the same time.
//! 
//! If you use scrolling you should probably use the [ValueTrackingWindow]. It will try to
//! preserve the data in the view as much as possible. 
//! 
//! If you implement some kind of log preview you probably would like to use [KeepOnBottom]
//! which will show just the last page of records from the store.
//! 
//! If you implement some kind of reporting dashboard you might find [KeepOnTop] useful. It
//! will lock the view to the first page of records. 

mod keep_on_bottom;
mod keep_on_top;
mod position_tracking_window;
mod store_state;
mod value_tracking_window;
mod window_transition;

use super::Range;
use super::math::Point;

pub use keep_on_bottom::KeepOnBottom;
pub use keep_on_top::KeepOnTop;
pub use position_tracking_window::PositionTrackingWindow;
pub use store_state::StoreState;
pub use value_tracking_window::ValueTrackingWindow;
pub use window_transition::WindowTransition;

/// Describes how the window view should behave in presence of changes
/// 
/// If you do custom implementation of the trait make sure you return [WindowTransition::Identity]
/// as often as possible. Otherwise you might find your view to work extremely slow since it will try
/// to compute updates for elements which are out of the view.
pub trait WindowBehavior {
    /// Computes change in order of indexes of elements on the line due to insert
    /// 
    /// - `r` is a current store view range
    /// - `p` is an insertion point
    fn insert(r: &StoreState<'_>, p: &Point) -> WindowTransition;

    /// Computes change in order of indexes of element on the line due to removal
    /// 
    /// - `r` is a current store view range
    /// - `p` is an insertion point
    fn remove(r: &StoreState<'_>, p: &Point) -> WindowTransition;

    /// Computes change in order of indexes of elements due to moving the range
    /// 
    /// - `r` is a current store view range
    /// - `moved` is the new range to which the view would be moved to
    fn slide(r: &StoreState<'_>, moved: &Range) -> WindowTransition;
}
