# Stores for relm4

This book introduces concept of stores and provides samples of usage in relm4 applications.

## What is store and why do I need it?

**Store is shared data collection**. There are few benefits of using stores.

1. It makes much easier to separate application business model and view.
2. It makes state propagation much easier.
3. It reduces the complexity of the applications.

Other way of thinking about Store is as really simple database specialized in making relm4 applications easier to write.

## What is the price to use it?

Now it's pre-alpha state. So you shouldn't use it at all. :)

To tell the truth, the target is to make it possible to use it with minimum costs and integrate it as tightly to the relm4 as possible. Currently it's not trivial to use stores since many things are not completed.

## What this book is not about?

- It's not about `rust`. Examples consists of multiple files and you are expected to add `mod.rs` files with appropriate exports by yourself (or check the directory with example). On the other hand we did our best to make sure that any file which has more then module declarations and reexports will be documented here.
- It's not about relm4. You won't learn how widget macro works or how events are propagated. We might talk a bit about that but only to make code presented here understandable.
- You won't learn how to write your own store/relm4 factory from this book. Writing the store or relm4 factory is complex task. There is a plan to add a chapter about that, but it won't happen anywhere in the near future.
- For the same reason as factories, StoreViews are also out of scope. Good for you we already provide some implementations.

## How to read the book

There are two ways to read this book. First one is to follow the chapters in order. This will gently introduce you to all concepts required to use stores. Second way is to go directly to the part you are interested into. The examples in the book after chapter 1 are just the modifications of the code from previous chapter. Full code of examples you can find in the `examples folder`.

### Window behavior

- [Description of window behavior](./examples/03-todo/01-ordering.md)
