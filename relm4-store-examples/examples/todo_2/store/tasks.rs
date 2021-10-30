use backend_inmemory::InMemoryBackend;
use backend_inmemory::InMemoryBackendBuilder;
use crate::model::Task;

pub type Tasks = InMemoryBackend<TasksBuilder>;

pub struct TasksBuilder {}

impl InMemoryBackendBuilder for TasksBuilder
{
    type DataModel = Task;

    fn initial_data() -> Vec<Self::DataModel> {
        let mut initial_tasks = Vec::new();

        for i in 0..1000015 {
            initial_tasks.push(
                Task::new(format!("Sample task {}", i), false)
            );
        }

        initial_tasks
    }
}