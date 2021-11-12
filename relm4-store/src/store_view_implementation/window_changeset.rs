use std::collections::HashSet;

use record::Id;
use record::TemporaryIdAllocator;

use crate::DataStore;
use crate::FactoryConfiguration;
use crate::FactoryContainerWidgets;
use crate::factory_configuration::StoreViewInnerComponent;

/// WindowChangeset describes how the store view window has changed in response to the changes in the store
#[derive(Debug)]
pub struct WindowChangeset<Widgets, Builder, Components, Allocator>
where
    Widgets: ?Sized + FactoryContainerWidgets<Builder, Components, Allocator>,
    Builder: FactoryConfiguration<Widgets, Components, Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
    Components: StoreViewInnerComponent<Builder>,
{
    /// Set of record id's of widgets which needs to be removed from the view
    pub widgets_to_remove: HashSet<Id<<Builder::Store as DataStore<Allocator>>::Record>>,
    /// Set of record id's of widgets which needs to be added to the view
    pub ids_to_add: HashSet<Id<<Builder::Store as DataStore<Allocator>>::Record>>,
    /// Set of record id's of widgets which needs to be updated
    pub ids_to_update: HashSet<Id<<Builder::Store as DataStore<Allocator>>::Record>>,
}