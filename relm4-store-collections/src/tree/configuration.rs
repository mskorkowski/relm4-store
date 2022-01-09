#[derive(Copy, Clone)]

/// Configuration of the [Tree]
/// 
/// Configuration allows you to set order of the tree. Depending on your use
/// case it might be a smart move to set big values as the order.
/// 
/// Default configuration is creating a BTree of order `12`. 
#[derive(Debug)]
pub struct TreeConfiguration {
    split_at: usize,
}

impl TreeConfiguration {
    /// Creates new instance of this structure
    /// 
    /// [Tree] using this configuration will be of the given `order`
    pub fn new(order: usize) -> Self {
        let half = if order % 2 == 0 {
            order / 2
        }
        else {
            (order + 1) / 2
        };

        Self{
            split_at: half,
        }
    }
}

impl Default for TreeConfiguration {
    fn default() -> Self {
        TreeConfiguration::new(12)
    }
}

impl TreeConfiguration{
    /// How many values node can keep
    pub fn capacity(&self) -> usize { 2*self.split_at -1 }
    /// Minimum length of node after splitting
    pub fn min_len_after_split(&self) -> usize { self.split_at - 1 }
    /// center index
    pub fn kv_idx_center(&self) -> usize { self.split_at - 1 }
    /// last index of the left part of split
    pub fn edge_idx_left_of_center(&self) -> usize { self.split_at - 1 }
    /// first index of the right part of split
    pub fn edge_idx_right_of_center(&self) -> usize { self.split_at }
}