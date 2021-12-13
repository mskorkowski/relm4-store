use std::collections::HashSet;

use record::Id;

/// WindowChangeset describes how the store view window has changed in response to the changes in the store
#[derive(Debug)]
pub struct WindowChangeset<Record>
where
    Record: 'static + record::Record,
{
    /// Set of record id's of widgets which needs to be removed from the view
    pub widgets_to_remove: HashSet<Id<Record>>,
    /// Set of record id's of widgets which needs to be added to the view
    pub ids_to_add: HashSet<Id<Record>>,
    /// Set of record id's of widgets which needs to be updated
    pub ids_to_update: HashSet<Id<Record>>,
    /// Marks changeset as reload
    pub reload: bool,
}

impl<Record> Default for WindowChangeset<Record>
where
    Record: 'static + record::Record,
{
    fn default() -> Self {
        WindowChangeset{
            widgets_to_remove: HashSet::new(),
            ids_to_add: HashSet::new(),
            ids_to_update: HashSet::new(),
            reload: false,
        }
    }   
}