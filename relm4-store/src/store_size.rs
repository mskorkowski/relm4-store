use std::convert::TryFrom;

/// Enum used to define the size of the store/store-view
/// 
/// Store/store-view decides how to respect those limits. It's up to the
/// implementation to decide which limits are hard and which are soft
/// and wherever it's acceptable to go beyond what's set.
#[derive(Debug)]
pub enum StoreSize {
    /// Keep unlimited amounts of data
    /// 
    /// It's not really unlimited. It just makes it absurdly high ([usize::Max])
    Unlimited,
    /// Keep only up to given amount of data
    Items(usize),
}

impl StoreSize {
    /// How many items can be stored
    pub fn items(&self) -> usize {
        match self {
            StoreSize::Unlimited => usize::MAX,
            StoreSize::Items(items) => *items
        }
    }
}

impl From<usize> for StoreSize {
    fn from(value: usize) -> StoreSize {
        StoreSize::Items(value)
    }
}

impl TryFrom<i64> for StoreSize {
    type Error = &'static str;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 {
            Err("Size can't be negative")
        }
        else {
            Ok(StoreSize::Items(value as usize))
        }
    }
}