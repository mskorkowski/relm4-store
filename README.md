# relm4-store

## State

THIS IS EARLY BETA VERSION. USE AT YOUR OWN RISK

There are two sides to APIs in this library

1. API's facing relm4 code and what you would use on the daily basis. This part should be stable.
2. There is public API related to the data management. This API is used by `data store` and `store view`. It's possible that some earthquakes will happen there but
the scope of changes should be limited to your `data store` implementations.

### Known limitations

1. Store view implementation is mostly complete in terms of features but still there are cases where things are suboptimal in terms of execution. If your logs will show
ERROR message ending with `unimplemented yet` reported from the `relm4-store-view-implementation` crate that means implementation is short circuited to reload whole
view and we are working on fixing this part.
2. Store is notifying store view in asynchronous way but view is using synchronous api to talk to the store. This limits current library usage to the cases when you can
preload data store with user data or get them fast enough. In some cases like fetching data from remote server or big data sets it might be an issue.

## Book

Beta version of the book can be found at [https://mskorkowski.github.io/relm4-store/beta/book/index.html](https://mskorkowski.github.io/relm4-store/beta/book/index.html)

You can also build the book yourself running `mdbook build` in the root of the project. It can be found in the `target\book` afterwards.

## Naming convention of examples

There are two kinds of examples in the `relm4-store-examples` crate. First one are complete applications which and second one are some special cases of the first case. Special cases can be either modifications or snapshots of the state of code at some given point of the book. The first case has names either ending with a number indicating a version or doesn't have a version number at all. Second case after version number has some extra description about it's content. Examples of this can be seen in table below

| Name | What it is |
|:-----|:-----------|
| todo_2 | Final version of the simple todo application from the book from chapter 2 |
| todo_2_single_scroll | Special case of the todo_2 showing how to implement custom scroll bar |
| todo_2_set_pagination | Snapshot of the code state while working towards todo_2 along the book |
| window_behavior | Application showcasing different behavior of store view window in case of presence of the new data |
