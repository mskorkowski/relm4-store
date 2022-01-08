# View - Main window

view/main_window.rs

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/view/main_window.rs}}
```

There are few things to take a note.

First `MainWindowModel` holds tasks store.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/view/main_window.rs:14:16}}
```

Second is that instead of `RelmComponent` we use `StoreViewComponent`.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/view/main_window.rs:35:37}}
```

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/view/main_window.rs:40:50}}
```
