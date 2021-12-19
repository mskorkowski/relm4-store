# Store

We need to keep todo data somewhere. So let's crate a first store in file `store/tasks.rs`.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/store/tasks.rs::35}}
```

That's it folks. Done. We've just implemented first store.

Ok, let's see what happens there

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/store/tasks.rs:1}}
```

In this line we import `InMemoryBackend`. It's a Store backend which keeps all data in the memory. The easiest one to use but you will loose all the data on application restart.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/store/tasks.rs:7}}
```

Here we define the convenient label for our store type. If we decide to change a backend, that's a place where we should do the update. You should never use backend types directly outside your store implementation. This will reduce amount of changes you must do in the code if you switch them. Maybe not often but changing a backend is sometimes necessity and being tight to it is bad. All patterns we will show in this book will try to minimize exposure the backend to the rest of your code.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/store/tasks.rs:9:}}
```

In here we define the builder structure which will provide us the new instance of the store with tasks and provides configuration of your backend. In most of the cases having one structure for both tasks is totally fine. In case you need some fancier builder you will probably split the configuration and keep it private in the module and have builder do just building.

[Now let's implement the view](./03-view.md)
