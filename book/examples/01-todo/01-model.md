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

## Implementation of `record::Record` for `Task`

Here is how we implemented the Record structure.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/model/task.rs:22:35}}
```

There are two methods and one type defined there. Method `get_id` is rather self explanatory. It returns current value of record identifier which we discussed in `Task structure` section. Method `set_permanent_id` overrides current value of id with new stable final version of identifier.

It's responsibility of the data store and backend to track down this information and propagate it. Why would you need something like that? The scenario I was solving for myself is "how application should behave in presence of slow backend".

Without this feature when I commit a record to the slow backend I need to wait for the backend to respond with information about saving the record and I under which this record was saved before I can safely show it to user. Other method would be to track down which records are committed and which not. This might involve things like remembering that record without id at the 5th position of some list is the one which should be updated when 2nd http request is successful. It sound painful. Implementing it definitely is. Even if we assume you can do it bug free. Write it down 3 times for slightly different scenarios. So how did we solve it then? By making it data definition problem.

1. Id must be unique. Two instances of the `Task` are considered to represent same record at maybe different point of time if their id is equal
2. In scope of the application, you must be able to return new unique id's during whole application lifetime. This allows you to provide temporary id's which are unique in currently running application. It doesn't matter if two running applications provide same unique id because when records from other application will be visible to this application only after being committed to the backend (for example database) which in turn would make them contain permanent id instead of temporary one. So this application will never see temporary id of other application
3. You are not allowed to keep copy of records with non permanent id. In most cases it's not an issue. Since after you create a record anc commit it to the database you don't really have a reason to keep copy of it. If for some reason you must keep the record around you are responsible for tracking this information
4. Only backend is allowed to call `set_permanent_id`

I'm going to elaborate little bit more about 4th point. To make it clear how bad it's to call it outside of the backend.

Let's assume you have a record like that

```rust,noplaypen
struct User {
    id: Id<User>,
    pub name: String,
}

impl Record for User {
    type Allocator = UserIdAllocator;

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
```

Somewhere in the code you've created a code like this:

```rust,noplaypen
fn new_user_from(mut u: User) -> User {
    let id = UserIdAllocator::getId();
    u.set_permanent_id()
}
```

You compile and test your application. Everything works.

Now few request later you change the user definition:

```rust,noplaypen
struct User {
    id: Id<User>,
    pub name: String,
    // must be unique across all users
    pub unique_email: String,
}
```

Your code still compiles. But now anytime you invoke the `new_user_from` you produce a value which breaks your business model a little bit. If you are not really careful with your tests this error might live in your code base for very long time.

I've seen bugs similar to this one living in the production systems for years. Fixing them afterwards is at least problematic and more often impossible.
