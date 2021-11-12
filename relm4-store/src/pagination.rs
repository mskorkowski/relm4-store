use record::TemporaryIdAllocator;

use crate::FactoryContainerWidgets;
use crate::StoreView;
use crate::factory_configuration::StoreViewInnerComponent;

/// Generic pagination methods which could be carpet implemented for any store-view
pub trait Pagination<Widgets, SV, Components, Allocator> 
where
    Widgets: ?Sized + FactoryContainerWidgets<SV::Configuration, Components, Allocator>, 
    SV: StoreView<Widgets, Components, Allocator>, 
    Allocator: TemporaryIdAllocator,
    Components: StoreViewInnerComponent<SV::Configuration>,
{
    /// Total amount of pages in store view
    fn total_pages(&self) -> usize;
    /// Current page in the view
    fn current_page(&self) -> usize;
}

impl<Widgets, SV, Components, Allocator> Pagination<Widgets, SV, Components, Allocator> for SV 
where
    Widgets: ?Sized + FactoryContainerWidgets<SV::Configuration, Components, Allocator>, 
    SV: StoreView<Widgets, Components, Allocator>, 
    Allocator: TemporaryIdAllocator,
    Components: StoreViewInnerComponent<SV::Configuration>,
{
    fn total_pages(&self) -> usize {
        let len = self.len();
        let size = self.window_size();
        ((len as f64)/(size as f64)).ceil() as usize
    }

    fn current_page(&self) -> usize {
        let window_start = *self.get_window().start();   
        let size = self.window_size();
        1 + ((window_start as f64)/(size as f64)).ceil() as usize
    }
}