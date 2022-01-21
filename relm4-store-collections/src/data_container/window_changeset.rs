use std::collections::HashSet;

use record::Id;

/// WindowChangeset describes how the store view window has changed in response to the changes in the store
/// 
/// Altho fields are public you should use methods as much as possible
#[derive(Debug)]
pub struct WindowChangeset<Record>
where
    Record: 'static + record::Record,
{
    /// Set of record id's of widgets which needs to be removed from the view
    pub ids_to_remove: HashSet<Id<Record>>,
    /// Set of record id's of widgets which needs to be added to the view
    pub ids_to_add: HashSet<Id<Record>>,
    /// Set of record id's of widgets which needs to be updated
    pub ids_to_update: HashSet<Id<Record>>,
    /// Marks changeset as reload
    pub reload: bool,
}

impl<Record> WindowChangeset<Record> 
where
    Record: 'static + record::Record,
{
    /// marks id as freshly added record
    #[inline]
    pub fn add(&mut self, id: Id<Record>) {
        self.ids_to_add.insert(id);
    }

    /// marks id as updated record
    #[inline]
    pub fn update(&mut self, id: Id<Record>) {
        self.ids_to_update.insert(id);
    }

    /// marks id as removed record
    #[inline]
    pub fn remove(&mut self, id: Id<Record>) {
        self.ids_to_remove.insert(id);
    }

    /// returns `true` if id is marked as added
    pub fn add_contains(&self, id: &Id<Record>) -> bool {
        self.ids_to_add.contains(id)
    }

    /// returns `true` if `id` is marked as updated
    pub fn update_contains(&self, id: &Id<Record>) -> bool {
        self.ids_to_update.contains(id)
    }

    /// returns `true` if `id` is marked as removed
    pub fn remove_contains(&self, id: &Id<Record>) -> bool {
        self.ids_to_remove.contains(id)
    }
}

impl<Record> Default for WindowChangeset<Record>
where
    Record: 'static + record::Record,
{
    fn default() -> Self {
        WindowChangeset{
            ids_to_remove: HashSet::new(),
            ids_to_add: HashSet::new(),
            ids_to_update: HashSet::new(),
            reload: false,
        }
    }
}