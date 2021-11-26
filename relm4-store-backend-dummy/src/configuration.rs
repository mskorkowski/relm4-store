use std::fmt::Debug;
use std::ops::Index;

use store::StoreMsg;

#[derive(Debug)]
pub struct Step<Record: record::Record + Debug + Clone> {
    pub data: Vec<Record>,
    pub events: Vec<StoreMsg<Record>>,
}

/// Configuration of the dummy data store
pub struct DummyBackendConfiguration<Record: record::Record + Debug + Clone> {
    /// List of states for dummy backend configuration
    pub steps: Vec<Step<Record>>,
    /// Data in the store at the beginning of the test
    pub initial_data: Vec<Record>,
}

impl<Record> Index<usize> for DummyBackendConfiguration<Record> 
where Record: record::Record + Debug + Clone 
{
    type Output = Step<Record>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.steps[index]
    }
}

impl<Record> DummyBackendConfiguration<Record> 
where Record: record::Record + Debug + Clone 
{
    /// Returns count of steps in the configuration
    /// 
    /// `0` means only initial state
    pub fn len(&self) -> usize {
        self.steps.len()
    }
}


impl<Record> Debug for DummyBackendConfiguration<Record> 
where Record: record::Record + Debug + Clone 
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