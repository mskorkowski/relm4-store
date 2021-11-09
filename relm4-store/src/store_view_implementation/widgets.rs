/// Nice little struct, stolen from relm4::factory
pub(crate) struct Widgets<Widgets, Root> {
    /// struct with widgets defined by user
    pub(crate) widgets: Widgets,
    /// root widget, in most cases gtk
    pub(crate) root: Root,
}