
use super::Range;

/// Basic information about store state required to calculate data window change
#[derive(Debug)]
pub struct StoreState<'a> {
    /// Store vide data range
    /// 
    /// Describes content of the current page
    pub page: &'a Range,
    /// How many data are currently loaded into the store view page
    /// 
    /// Page can be partially visible due to various reasons. Not exhaustive list
    /// 
    /// - Not enough data in data store to fill the first page
    /// - Last page of data in data store doesn't fill the whole page size
    /// - Filtering was applied and there is not enough data to fill a page
    /// - Grouping might produce a cases when there is less data to show then
    ///   a page size
    pub view: usize,
}