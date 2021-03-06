use backend_inmemory::SortedInMemoryBackend;
use backend_inmemory::SortedInMemoryBackendConfiguration;
use store::Sorter;
use store::Store;

use crate::model::Task;

#[derive(Clone, Copy, Debug)]
pub enum OrderTasksBy {
    Name{ascending: bool},
}

impl Sorter<Task> for OrderTasksBy {
    fn cmp(&self, lhs: &Task, rhs: &Task) -> std::cmp::Ordering {
        match self {
            OrderTasksBy::Name{ascending} => {
                if *ascending {
                    lhs.description.cmp(&rhs.description)
                }
                else {
                    lhs.description.cmp(&rhs.description).reverse()
                }
            },
        }
    }
}

pub type Tasks = Store<SortedInMemoryBackend<TasksBuilder>>;

pub struct TasksBuilder {}

impl TasksBuilder{
    pub fn build() -> Tasks {
        Tasks::new(
            SortedInMemoryBackend::new()
        )
    }
}

impl SortedInMemoryBackendConfiguration for TasksBuilder
{
    type Record = Task;
    type OrderBy = OrderTasksBy;

    fn initial_data() -> Vec<Self::Record> {
        vec![
            Task::new(String::from("r"), false),
            Task::new(String::from("f"), false),
            Task::new(String::from("i"), false),
            Task::new(String::from("c"), false),
            Task::new(String::from("o"), false),
            Task::new(String::from("y"), false),
            Task::new(String::from("l"), false),
            Task::new(String::from("u"), false),
        ]
    }

    fn initial_order() -> Self::OrderBy {
        OrderTasksBy::Name{ascending: true}
    }
}