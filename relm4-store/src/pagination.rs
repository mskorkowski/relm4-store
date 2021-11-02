use crate::StoreView;

pub trait Pagination<SV: StoreView> {
    fn total_pages(&self) -> usize;
    fn current_page(&self) -> usize;
}

impl<SV: StoreView> Pagination<SV> for SV {
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