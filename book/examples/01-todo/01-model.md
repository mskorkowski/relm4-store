# Model

We are writing simple todo list. So we need to talk about tasks!

Task will have the description and status wherever it's completed or not.

So let's start with it. Create a file `model/task.rs` and write there

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/model/task.rs::35}}
```

This is minimal implementation for record. It consist of

1. Definition of `Task` structure
2. Implementation for `Task` which provides method `new`.
3. Implementation of `record::Record` (`relm4_store_record::Record`)

## `Task` structure

First we defined structure `Task`. It has a three fields. First is an `id`. This filed is used to identify the record in the store. This `id` must be stable during whole application execution. Later we have a description. It will contain the description of the task. At the end there is boolean flag which will let us know if the task has been completed or not.

Task derives two traits `Clone` and `Debug`. `Debug` is obvious. `Clone` is consequence of what `Record` is. Since you can save a record in the database it's equivalent of `Clone`. What's more without `Clone` it would be hard to reason about multiple views showing same record. It also allows to escape the lifetime boundary issues. Store is a collection of records. So whatever you will place there should have `'static` lifetime. Now let's think about keeping references to the records with `'static` lifetime which are being removed while application is being run. It sounds like going against what `'static' is. It isn't but requires so many lifetime annotations and makes code way overcomplicated.

## Implementation of `Task`

As good practice I strongly suggest

1. That you implement `new` for your business model structures. `new` should return really "new" instance so identifier should be set to `Id::New`.
2. In case you need to recreate an instance I would suggest using `from` method.

It's related to the expected behavior of implementations of `Record`. Two instances of the business model are expected to represent the same value if their identifier is equal and not when internals are in the same state. You might think about `Record` as photograph of the business state at some point of time. You can have multiple photos of the `Record` but it's up to you to tell which is newer and which is older.

We can end our business modelling session now, except I like to add some pretty printing abilities to my business model classes. It's alway useful to be able to println them to see what happens.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/model/task.rs:37:}}
```

[Let's start working on the store!](./02-store.md)
