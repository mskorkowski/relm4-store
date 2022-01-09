use std::fmt::{self, Debug, Display, Formatter};

use record::{Id, Record, DefaultIdAllocator, TemporaryIdAllocator};

#[derive(Clone, Debug)]
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

impl Record for Task {
    type Allocator = DefaultIdAllocator;
    fn get_id(&self) -> Id<Task> {
        self.id
    }

    fn set_permanent_id(
        &mut self, 
        value: <Self::Allocator as TemporaryIdAllocator>::Type
    ) -> Result<(), record::IdentityError> {
        self.id = Id::from(value);
        Ok( () )
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let completed = if self.completed {'x'} else {' '};
        f.write_str(&format!("[{}] {}", completed, self.description))
    }
}