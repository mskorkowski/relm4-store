use std::io::stdout;
use std::io::Write;

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


        println!("Generating 10_000_000 records. Each `.` is 10000 records\n");
        stdout().flush().unwrap();

        for i in 0..10_000_000 {
            initial_tasks.push(
                Task::new(format!("Sample task {}", i), false)
            );
            if i % 10_000 == 0 {
                stdout().write(".".as_bytes()).unwrap();
                stdout().flush().unwrap();
            }
        }

        println!("\nInitializing the store");
        stdout().flush().unwrap();

        initial_tasks
    }
}