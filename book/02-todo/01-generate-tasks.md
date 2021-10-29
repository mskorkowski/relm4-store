# Generating tasks

Now we will generate 10000 tasks to be kept in our store. Why 10015?

- Because it's enough to see that it takes a while to start our application. The reason for it is amount of work needed to generate view and it's still short enough so we can still start an application for testing. If you feel adventures you can even go for higher values. On my computer 100 000 took about a minute to start.
- Because it will leave last page just partially filled. We will use page size of 50 tasks.

In `store/tasks.rs` we modify the `TaskBuilder`.

```rust
impl TasksBuilder {
    pub fn build() -> Tasks {
        let mut initial_tasks = Vec::new();

        for i in 0..10015 {
            initial_tasks.push(
                Task::new(format!("Sample task {}", i), false)
            );
        }

        InMemoryBackend::new(initial_tasks)
    }
}
```

If you start an application it will take a bit of the time before interface will even show up. Now if your application would need to show lists like that, it would feel unusable. So now we need to do something about the view.

[Updating store view](./02-updating-store-view.md)
