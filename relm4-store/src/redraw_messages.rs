//! Defines messages sent when redraw must be handled

/// Messages used to notify interrelated components that redraw is needed
/// 
/// They might be interesting for you if you are implementing custom store-view
#[derive(Debug)]
pub enum RedrawMessages {
    /// Message sent to redraw parts of ui
    Redraw,
}