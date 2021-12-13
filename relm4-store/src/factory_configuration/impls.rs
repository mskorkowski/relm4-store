use reexport::relm4;

use relm4::Model as ViewModel;

use crate::FactoryConfiguration;
use crate::FactoryContainerWidgets;

use super::StoreViewInnerComponent;

impl<ParentModel: ViewModel> StoreViewInnerComponent<ParentModel> for () {
    fn on_store_update(&mut self) {}
}

impl<Configuration> FactoryContainerWidgets<Configuration> for () 
where
    Configuration: ?Sized + FactoryConfiguration<View=()>,
    Configuration::ViewModel: ViewModel<Widgets=()>,
{
    fn container_widget(&self) -> &<Configuration as FactoryConfiguration>::View {
        &()
    }
}