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
        let mut initial_tasks = Vec::new();

        for i in 0..1000015 {
            initial_tasks.push(
                Task::new(format!("Sample task {}", i), false)
            );
        }

        initial_tasks
    }
}