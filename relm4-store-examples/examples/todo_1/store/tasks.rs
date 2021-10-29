use backend_inmemory::InMemoryBackend;
use backend_inmemory::InMemoryBackendBuilder;
use crate::model::Task;

pub type Tasks = InMemoryBackend<TasksBuilder>;

pub struct TasksBuilder {}

impl InMemoryBackendBuilder for TasksBuilder
{
    type DataModel = Task;

    fn initial_data() -> Vec<Self::DataModel> {
        Vec::new()
    }
}