use backend_inmemory::InMemoryBackend;
use backend_inmemory::InMemoryBackendConfiguration;
use record::DefaultIdAllocator;
use crate::model::Task;

pub type Tasks = InMemoryBackend<TasksBuilder, DefaultIdAllocator, DefaultIdAllocator>;

pub struct TasksBuilder {}

impl InMemoryBackendConfiguration<DefaultIdAllocator> for TasksBuilder
{
    type Record = Task;

    fn initial_data() -> Vec<Self::Record> {
        Vec::new()
    }
}