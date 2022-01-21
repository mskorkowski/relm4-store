# View

We need to add a button with a trash to the right of our task description. When user presses the button we will remove the task from the store.

So in the `view/task_list.rs` we need to add imports

```rust,noplaypen
use gtk::Button;
use gtk::prelude::ButtonExt;
```

Then we need to update the `TaskWidgets` so they will hold reference to the delete button

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_4/view/task_list.rs:53:60}}
```

We need to add `Delete` event to the `TaskMsg` so we can track which record should be deleted.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_4/view/task_list.rs:45:52}}
```

Now we need to update the `init_view` method in the implementation of the `StoreViewPrototype` for `TaskListViewModel`

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_4/view/task_list.rs:85:86}}
{

    ...

{{#include ../../../relm4-store-examples/examples/todo_4/view/task_list.rs:101:169}}

    ...

}
```

The most important part is that we've built `delete_button` and added it to the `root` widget. When `delete_button` is clicked
it will send `TaskMsg::Delete`. To make ui nice we in the `label` we've added to extra settings

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_4/view/task_list.rs:136:137}}
```

This will expand horizontally and text will be left aligned.

Last thing left to do is to handle the changes in the `view` method in the implementation of `StoreViewPrototype` for `TaskListViewModel`.
Compiler will tell you exactly where.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_4/view/task_list.rs:85:86}}
{

    ...

{{#include ../../../relm4-store-examples/examples/todo_4/view/task_list.rs:194:219}}

    ...

}
```

Sending `StorMsg::Delete` to the task with record id will remove it from the data store.

After this chapter you know how to `create`, `update` and `delete` the records from the store. Full source code can be found in the examples `todo_4`.
