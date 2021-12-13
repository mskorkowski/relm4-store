use record::TemporaryIdAllocator;

use crate::StoreView;

/// Generic pagination methods which could be carpet implemented for any store-view
pub trait Pagination<SV, StoreIdAllocator> 
where
    SV: StoreView<StoreIdAllocator>,
    StoreIdAllocator: TemporaryIdAllocator,
{
    /// Total amount of pages in store view
    fn total_pages(&self) -> usize;
    /// Current page in the view
    fn current_page(&self) -> usize;
}

impl<SV, StoreIdAllocator> Pagination<SV, StoreIdAllocator> for SV 
where
    SV: StoreView<StoreIdAllocator>,
    StoreIdAllocator: TemporaryIdAllocator,
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