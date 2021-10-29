use std::convert::TryFrom;


pub enum StoreSize {
    Unlimited,
    Items(usize),
}

impl StoreSize {
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