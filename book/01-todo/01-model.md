# Model

We are writing simple todo list. So we need to talk about tasks!

Task will have an description and status wherever it's completed or not.

So let's start with it. Create a file `model/task.rs` and write there

```rust
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use model::Id;
use model::Identifiable;
use model::Model;

#[derive(Clone)]
pub struct Task {
    id: Id<Task>,
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String, completed: bool) -> Self {
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
```

This is minimal implementation for model. So let's talk about it.

First we defined structure `Task`. It has a three fields. First is an `id`. This filed is used to identify the record in the store. This `id` must be stable during whole application execution. Later we have a description. It will contain the description of the task. At the end there is boolean flag which will let us know if the task has been completed or not.

Next we defined the method `new` for Task. It will create a record marked as new.

As the next thing we've implemented interface `Identifiable` for Task. This interface makes store capable of understanding the identity of task without talking about it's structure.

Next thing is implementation of Model for Task. One last time it's not `relm4::Model`. It's model like business model.

As the last thing we've implemented `Debug`. It's required by store. In case of issues store will use it to let you know what was wrong.

We can end our business modelling session now, except I like to add some pretty printing abilities to my business model classes. It's alway useful to be able to println them to see what happens.

```rust
impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let completed = if self.completed {'x'} else {' '};
        f.write_str(format!("[{}] {}", completed, self.description))
    }
}
```

[Let's start working on the store!](./02-store.md)
