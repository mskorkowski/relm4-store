use reexport::uuid;

use std::fmt::{self, Debug, Display, Formatter};

use uuid::Uuid;

use record::{Id, Record, DefaultIdAllocator};

#[derive(Clone)]
pub struct Task {
    id: Id<Task, DefaultIdAllocator>,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(description: String, completed: bool) -> Self {
        Self{
            id: Id::new(),
            description,
            completed,
        }
    }
}

impl Record<DefaultIdAllocator> for Task {
    fn get_id(&self) -> Id<Task, DefaultIdAllocator> {
        self.id
    }

    fn set_permanent_id(&mut self, value: Uuid) -> Result<(), record::IdentityError> {
        self.id = Id::from(value);
        Ok( () )
    }
}

impl Debug for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task")
            .field("id", &self.id)
            .field("description", &self.description)
            .field("completed", &self.completed)
            .finish()
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let completed = if self.completed {'x'} else {' '};
        f.write_str(&format!("[{}] {}", completed, self.description))
    }
}