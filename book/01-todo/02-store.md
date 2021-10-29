# Store

We need to keep todo data somewhere. So let's crate a first store in file `store/tasks.rs`.

```rust
use backend_inmemory::InMemoryBackend;

use crate::model::Task;

pub type Tasks = InMemoryBackend<Task>;

pub struct TasksBuilder {}

impl TasksBuilder {
    pub fn build() -> Tasks {
        let initial_tasks = Vec::new();
        InMemoryBackend::new(initial_tasks)
    }
}
```

That's it folks. Done. We've just implemented first store.

Ok, let's see what happens there

```rust
use backend_inmemory::InMemoryBackend;
```

In this line we import `InMemoryBackend`. It's an implementation of the store which keeps all data in the memory. The easiest store to use but you will loose all the data on application restart.

```rust
pub type Tasks = InMemoryBackend<Task>;
```

Here we define the convenient label for our store type. If we decide to change a backend, that's a place where we should do the update. You should never use backend types directly outside your store implementation. This will reduce amount of changes you must do in the code if you switch them.

```rust
pub struct TasksBuilder {}

impl TasksBuilder {
    pub fn build() -> Tasks {
        let initial_tasks = Vec::new();
        InMemoryBackend::new(initial_tasks)
    }
}
```

In here we define the helper structure which will provide us the new instance of the store with tasks!

[Now let's implement the view](./03-view.md)
