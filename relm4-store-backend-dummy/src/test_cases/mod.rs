//! Contains various configurations for the dummy data store ready for use in your tests
mod add;
mod add_multistep;
mod remove;
mod test_case_builder;

#[cfg(test)]
mod tests;

pub use test_case_builder::TestCaseBuilder;

use record::DefaultIdAllocator;
use record::TemporaryIdAllocator;
use reexport::uuid;


use uuid::Uuid;

use record::Id;
use record::Record;
use super::DummyBackendConfiguration;

/// Sample record for test cases
#[derive(Debug, Clone)]
pub struct TestRecord {
    id: Id<Self>,
    /// Human understandable description of this record
    /// 
    /// Setting proper description for the test record describing what, when, why, where
    /// makes debugging tests easier by a lot
    pub label: String,
    added: Option<usize>,
    removed: Option<usize>,
}

impl TestRecord {
    /// Creates instance of the test record which is expected to live through out whole test
    pub fn constant(label: &str) -> Self {
        TestRecord{
            id: Id::new(),
            label: String::from(label),
            added: None,
            removed: None,
        }
    }

    /// Creates instance of the test record which is expected to be added at given step
    /// 
    /// This record is expected to be present in the data till the end of the test
    pub fn since(label: &str, step: usize) -> Self {
        TestRecord{
            id: Id::new(),
            label: String::from(label),
            added: Some(step),
            removed: None,
        }
    }

    /// Create instance of the test record which is expected to be present until given step (exclusive)
    /// 
    /// This record is expected to be present since initial data to given step
    pub fn until(label: &str, step: usize) -> Self {
        TestRecord{
            id: Id::new(),
            label: String::from(label),
            added: None,
            removed: Some(step),
        }
    }

    /// Creates instance of the test record which is expected to be in data in steps belonging to a range `[added, removed)`
    pub fn between(label: &str, added: usize, removed: usize) -> Self {
        TestRecord{
            id: Id::new(),
            label: String::from(label),
            added: Some(added),
            removed: Some(removed),
        }
    }

    /// Make record permanent
    pub fn permanent(mut self) -> Self {
        self.set_permanent_id(DefaultIdAllocator::new_id()).unwrap();
        self
    }
}

impl Record for TestRecord {
    type Allocator = DefaultIdAllocator;

    fn get_id(&self) -> record::Id<Self> {
        self.id
    }

    fn set_permanent_id(&mut self, value: Uuid) -> Result<(), record::IdentityError> {
        if !self.id.is_new() {
            Err(record::IdentityError("TestRecord already has permanent id"))
        }
        else {
            self.id = Id::from(value);
            Ok( () )
        }
    }
}

impl PartialEq for TestRecord {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
            self.label == other.label &&
            self.added == other.added &&
            self.removed == other.removed
    }
}

impl Eq for TestRecord {}

type C = DummyBackendConfiguration<TestRecord>;

/// Describes test case
#[derive(Debug)]
pub struct TestCase {
    /// Configuration for the [DummyBackend]
    pub configuration: C,
    /// Data used by the test case
    pub data: Vec<TestRecord>,
}

/// Contains various test case configurations for the DummyStore
#[derive(Debug)]
pub struct TestCases {}

