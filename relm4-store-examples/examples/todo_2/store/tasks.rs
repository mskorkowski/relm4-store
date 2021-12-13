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
        let mut initial_tasks = Vec::new();

        for i in 0..1000015 {
            initial_tasks.push(
                Task::new(format!("Sample task {}", i), false)
            );
        }

        initial_tasks
    }
}