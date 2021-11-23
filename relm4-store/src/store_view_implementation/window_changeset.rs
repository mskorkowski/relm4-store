use std::collections::HashSet;

use record::Id;
use record::TemporaryIdAllocator;

use crate::DataStore;
use crate::FactoryConfiguration;

/// WindowChangeset describes how the store view window has changed in response to the changes in the store
#[derive(Debug)]
pub struct WindowChangeset<Configuration, Allocator>
where
    Configuration: ?Sized + FactoryConfiguration<Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
{
    /// Set of record id's of widgets which needs to be removed from the view
    pub widgets_to_remove: HashSet<Id<<Configuration::Store as DataStore<Allocator>>::Record>>,
    /// Set of record id's of widgets which needs to be added to the view
    pub ids_to_add: HashSet<Id<<Configuration::Store as DataStore<Allocator>>::Record>>,
    /// Set of record id's of widgets which needs to be updated
    pub ids_to_update: HashSet<Id<<Configuration::Store as DataStore<Allocator>>::Record>>,
    /// Marks changeset as reload
    pub reload: bool,
}