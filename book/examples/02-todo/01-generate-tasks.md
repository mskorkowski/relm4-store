# Generating tasks

Now we will generate 20015 tasks to be kept in our store. Why 20015?

- Because it will leave last page just partially filled. We will use page size of 50 tasks.
- Because it's value big enough to be meaningful

In `store/tasks.rs` we modify the `TaskBuilder`.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_2/store/tasks.rs:19:}}
```

If you start an application it will take a quite bit of the time before interface will even show up. Now if your application would need to show lists like that, it would feel unusable. So now we need to do something about the view.

Code after this step can be found in example `todo_2_generating_tasks`.
