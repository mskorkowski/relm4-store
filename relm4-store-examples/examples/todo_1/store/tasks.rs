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
        Vec::new()
    }
}