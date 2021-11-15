use backend_inmemory::SortedInMemoryBackend;
use backend_inmemory::SortedInMemoryBackendConfiguration;
use crate::model::Task;

pub type Tasks = SortedInMemoryBackend<TasksBuilder>;

pub struct TasksBuilder {}



impl SortedInMemoryBackendConfiguration for TasksBuilder
{
    type Record = Task;

    fn initial_data() -> Vec<Self::Record> {
        Vec::new()
    }
}


