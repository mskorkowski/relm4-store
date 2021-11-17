use backend_inmemory::SortedInMemoryBackend;
use backend_inmemory::SortedInMemoryBackendConfiguration;
use crate::model::Task;

pub enum OrderTasksBy {
    Name,
}

pub type Tasks = SortedInMemoryBackend<TasksBuilder>;



pub struct TasksBuilder {}

impl SortedInMemoryBackendConfiguration for TasksBuilder
{
    type Record = Task;
    type OrderBy = OrderTasksBy;

    fn initial_data() -> Vec<Self::Record> {
        Vec::new()
    }

    fn initial_order() -> Self::OrderBy {
        OrderTasksBy::Name
    }
}


