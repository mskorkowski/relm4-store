# Updating task list

All changes should happen in the `view/task_list.rs`

In `relm4-store` components there is pagination component ready for you to use. Let's import it

```rust,noplaypen
use relm4::Components;
use relm4::RelmComponent;

use components::pagination::PaginationMsg;
use components::pagination::PaginationConfiguration;
use components::pagination::PaginationViewModel;

use store::StoreViewInnerComponent;
```

Now we need to add a component for the tasks list

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_2/view/task_list.rs:195:229}}
```

TaskListComponent and pagination are normal relm4 components. For `TaskListComponent` we need to implement two extra traits. First is `PaginationConfiguration`. As name implies it provides configuration for pagination component. Second one and more interesting is `StoreViewInnerComponent`. This one provides a way to notify components when there is a change in the store. This allows to solve chicken and the egg problem of what's first store view or the pagination. Without view there is no point in pagination but pagination must own the view since it manages it.

Since we've created component for task list, now we need to add it to the `relm4::Model`

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_2/view/task_list.rs:73:79}}
```

What's left is to add bunch of `'static` lifetimes for `Config` generic attribute all around the file (compiler will tell you where). This is required because compiler can't infer the lifetime of some of the types.

Hopefully full list of the places to add `'static` lifetime:

- implementation of the `ViewModel for TaskListViewModel`
- implementation of the `StoreViewPrototype for TaskListViewModel`
- implementation of the `FactoryContainerWidget for TasksListViewWidgets`

Now let's put a cherry on top and add the pagination to the `TaskListViewWidgets`.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_2/view/task_list.rs:231:253}}
```

Last `append` adds pagination to the view.

This ends the story of adding pagination to the task list. Source code can be found in `todo_2` example
