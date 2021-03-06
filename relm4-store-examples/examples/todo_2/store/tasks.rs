use backend_inmemory::InMemoryBackend;
use backend_inmemory::InMemoryBackendConfiguration;
use store::Store;

use crate::model::Task;

pub type Tasks = Store<InMemoryBackend<TasksBuilder>>;

pub struct TasksBuilder {}

impl TasksBuilder{
    pub fn build() -> Tasks {
        Tasks::new(
            InMemoryBackend::new()
        )
    }
}

impl InMemoryBackendConfiguration for TasksBuilder
{
    type Record = Task;

    fn initial_data() -> Vec<Self::Record> {
        let mut initial_tasks = Vec::new();

        for i in 0..20015 {
            initial_tasks.push(
                Task::new(format!("Sample task {}", i), false)
            );
        }

        initial_tasks
    }
}