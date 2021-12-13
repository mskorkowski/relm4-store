use reexport::relm4;

use relm4::Model as ViewModel;

use record::TemporaryIdAllocator;

use crate::FactoryConfiguration;
use crate::FactoryContainerWidgets;

use super::StoreViewInnerComponent;

impl<ParentModel: ViewModel> StoreViewInnerComponent<ParentModel> for () {
    fn on_store_update(&mut self) {}
}

impl<Configuration, StoreIdAllocator> FactoryContainerWidgets<Configuration, StoreIdAllocator> for () 
where
    Configuration: ?Sized + FactoryConfiguration<StoreIdAllocator, View=()>,
    Configuration::ViewModel: ViewModel<Widgets=()>,
    StoreIdAllocator: TemporaryIdAllocator,
{
    fn container_widget(&self) -> &<Configuration as FactoryConfiguration<StoreIdAllocator>>::View {
        &()
    }
}