

// use std::fmt::Debug;

/// Nice little struct, stolen from relm4::factory
pub struct Widgets<Widgets, Root> {
    pub widgets: Widgets,
    pub root: Root,
}

// impl<WidgetsType: Debug, Root: Debug> Debug for Widgets<WidgetsType, Root> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Widgets")
//             .field("widgets", &self.widgets)
//             .field("root", &self.root)
//             .finish()
//     }
// }