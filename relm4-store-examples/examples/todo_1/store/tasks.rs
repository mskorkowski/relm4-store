use backend_inmemory::InMemoryBackend;
use backend_inmemory::InMemoryBackendConfiguration;
use record::DefaultIdAllocator;
use crate::model::Task;

pub type Tasks = InMemoryBackend<TasksBuilder>;

pub struct TasksBuilder {}

impl InMemoryBackendConfiguration for TasksBuilder
{
    type Record = Task;
    type Allocator = DefaultIdAllocator;

    fn initial_data() -> Vec<Self::Record> {
        Vec::new()
    }
}