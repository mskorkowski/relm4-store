use std::collections::HashSet;

use model::Id;

use super::DataStoreBase;
use super::FactoryBuilder;

pub struct WindowChangeset<Builder> 
where
    Builder: FactoryBuilder + 'static
{
    pub widgets_to_remove: HashSet<Id<<Builder::Store as DataStoreBase>::Model>>,
    pub ids_to_add: HashSet<Id<<Builder::Store as DataStoreBase>::Model>>,
    pub ids_to_update: HashSet<Id<<Builder::Store as DataStoreBase>::Model>>,
}