use std::fmt::{Debug, Display, Formatter, Result};
use model::{Id, Identifiable, Model};

#[derive(Clone)]
pub struct Task {
    id: Id<Task>,
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

impl Identifiable for Task {
    type Id = Id<Task>;

    fn get_id(&self) -> Id<Task> {
        self.id
    }
}


impl Model for Task {}

impl Debug for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Task")
            .field("id", &self.id)
            .field("description", &self.description)
            .field("completed", &self.completed)
            .finish()
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let completed = if self.completed {'x'} else {' '};
        f.write_str(&format!("[{}] {}", completed, self.description))
    }
}