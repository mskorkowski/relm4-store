use std::fmt::Debug;
use std::ops::Index;

use record::DefaultIdAllocator;
use record::TemporaryIdAllocator;
use store::StoreMsg;

#[derive(Debug, Clone)]
pub struct Step<Record: record::Record<Allocator> + Debug + Clone, Allocator: TemporaryIdAllocator> {
    pub data: Vec<Record>,
    pub events: Vec<StoreMsg<Record, Allocator>>,
}

/// Configuration of the dummy data store
#[derive(Clone)]
pub struct DummyBackendConfiguration<Record, Allocator> 
where
    Record: record::Record<Allocator> + Debug + Clone, 
    Allocator: TemporaryIdAllocator, 
{
    /// List of states for dummy backend configuration
    pub steps: Vec<Step<Record, Allocator>>,
    /// Data in the store at the beginning of the test
    pub initial_data: Vec<Record>,
}

impl<Record, Allocator> Index<usize> for DummyBackendConfiguration<Record, Allocator> 
where 
    Record: record::Record<Allocator> + Debug + Clone,
    Allocator: TemporaryIdAllocator,
{
    type Output = Step<Record, Allocator>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.steps[index]
    }
}

impl<Record, Allocator> DummyBackendConfiguration<Record, Allocator> 
where 
    Record: record::Record<Allocator> + Debug + Clone,
    Allocator: TemporaryIdAllocator,
{
    /// Returns count of steps in the configuration
    /// 
    /// `0` means only initial state
    pub fn len(&self) -> usize {
        self.steps.len()
    }
}


impl<Record, Allocator> Debug for DummyBackendConfiguration<Record, Allocator> 
where 
    Record: record::Record<Allocator> + Debug + Clone,
    Allocator: TemporaryIdAllocator + Debug,
{
    #[cfg(not(tarpaulin_include))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DummyBackendConfiguration")
            .field("len", &self.steps.len())
            .field("steps", &self.steps)
            .field("initial_data", &self.initial_data)
            .finish()
    }
}