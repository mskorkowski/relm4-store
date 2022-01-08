# Custom sorting

In this example we will be working with natural order. This means we need to update the `Tasks` store. All changes will take place in the `store/tasks.rs`.

First we need to change our use statements so we can change backend to the `SortedInMemoryBackend` from

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_2/store/tasks.rs:1:5}}
```

to

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_3/store/tasks.rs:1:6}}
```

`SortedInMemoryBackend` gives an ability to change natural order of the elements in the store. `SortedInMemoryBackendConfiguration` is the trait describing
configuration of the sorted backend. It's expanded version of the `InMemoryBackendConfiguration` to also provide information about how elements should be ordered.
`Sorter` is the trait which implementation will decide what orders are available.

## Implementing sorter

This section is important. We will talk a lot about Rust and how to go around some of design decisions done by Rust developers.

In Rust you can implement `std::cmp::Ord` for your structure. This will define natural order of all instances of your structure. Now there is a problem what if there
is more then one natural order? For example sales transactions can be sorted by the date of creations, or they can be sorted by the amount in the month, or they can
be sorted by the customer and date. There is many many more ways to provide good and meaningful order to more complex values. Unfortunately standard library of Rust
doesn't allow such scenarios. So how we can solve it? There are two ways and both are in the terrible territory

1. Break requirements of `std::cmp::Ord` implementation
2. Repeat the code

Let's talk about first point, breaking `Ord`. To properly implement `Ord` you must define total order. But your implementation must be coffined to values provided by
the structure so either you need a way to update all the instances of your structure simultaneously so every two instances will be compared using same comparator and
constitute that way a total order or you end up with instances which can't be compared with each other in the safe way. Returning broken result from sorting is the
smallest issue here since the second choice can make some sorting algorithms fail into infinite loop. First option on the other hand is going to mess up your results
if you have more then one collection of your structures which needs to be in order. If you need to change order of the first collection and change the natural order
of your structure when you will add new element to your second collection you are screwed. So breaking `Ord` is bad.

Second choice is to use lambdas and write your own sorting strategy every time again and again. Repeating the code is bad that's truism. If your code needs to guarantee
the correct order of elements now you either end up writing your own wrapper around standard collections or repeatably write your sorting functions. If your sorting
is just a tiny bit more complex then comparing single field you will not like this idea. Searching for errors where by accident fields in order where swapped is no fun.

`SortedInMemoryStore` is specialized collection which keeps ordering internal to the collection so it's not related to any data inside the sorted structure. So as long
as instance of `Sorter` constitutes the total order we can update order of elements just this in this store and it won't affect any other store of elements of the same
kind. Since it's relative to store we can implement as many sorting strategies as we like and swap them in the runtime.

### Total order

Until now I was total order this and total order that. So let's remind ourselves what it is. In simple words it means that for any two elements in the set you can
definitely say which one is smaller or wherever they are equal. In case of our `todo` application it means that for any two possible tasks (so not only ones we created,
but literally any possible tasks) we must provide an answer what is relationship between them.

If we would be talking about integers, or rational numbers, or real numbers, or texts that order is really natural. `1` is before `2`, `a` is before `b`, etc... Problem
arises when we start talking about more complex structures. For example complex numbers don't have this kind of natural order. It's possible to order them but it
requires providing a definition of how you would like to see them ordered. Examples can be:

- real part, imaginary part
- imaginary part, real part
- length, angle
- angle, length

And there is infinitely many more different orderings which are equally good. When you write your application the structures are often way more complex. So defining
good total order becomes harder and harder.

Now let's talk about math

1. It's reflexive: `a <= a`
2. It's transitive: `If a <= b and b <= c then a <= c`
3. It's antisymmetric: `If a <= b and b <= a then a == b`
4. It's total: For any `a` and `b` `a <= b or b <= a`

**Warning** If you don't like the idea of being sad java programmer take this relationships seriously.

### Implementation

Let's stop talking and get to work

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_3/store/tasks.rs:9:30}}
```

As you see we've implemented `Sorter` for enumeration and not a structure. This way you can add the new ways of sorting as you see fit. Just remember to make sure that
for every case `cmp` method is total order.

Where is the issue in our implementation? Equality relationship. Two records are the same if they have a same id. But our implementation doesn't use that. If we would
be using it the total order would be broken. So as you can see inside of the store we are using two different definitions of what `equal` means.

The `SortedInMemoryBackend` is using `record id equality` to perform CRUD actions on record level. When we talk about order so which element is after which we use sorter
equality. That in itself can lead to the issues. What if I have a two instances of the record in the store one with the old value and second with new one?
`SortedInMemoryBackend` doesn't. So you are safe.

If we would implement some kind of store which keeps historic values of the record so you can do record based rollback it would add another level of thinking how to
handle all of that.

Any store backend implementation is a convenience tool but you must think and understand about how it works. Unfortunately at the current level of implementation
I don't see any way to make it one stop shop for you.

## Backend

Wherever we used `InMemoryBackend` or `InMemoryBackendConfiguration` earlier now we must use `SortedInMemoryBackend` and `SortedInMemoryBackendConfiguration`.

```rust,noplaypen
pub type Tasks = Store<InMemoryBackend<TasksBuilder>>;
```

becomes

```rust,noplaypen
pub type Tasks = Store<SortedInMemoryBackend<TasksBuilder>>;
```

and

```rust,noplaypen
impl InMemoryBackendConfiguration for TasksBuilder
```

becomes

```rust,noplaypen
impl SortedInMemoryBackendConfiguration for TasksBuilder
```

What's left is to update the implementation of the `SortedInMemoryBackendConfiguration` for `TaskBuilder`

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_3/store/tasks.rs:40:61}}
```

First we've defined `OrderBy` type to be the implementation of the `Sorter`. This way our backend will know which sorter to use.

As you see I've reduced amount of records by creating them by hand as unordered bunch of tasks. When you open your application they will show up in ascending order.
Ease proof that we didn't make a mistake.

Last update is the `initial_order` method. We didn't use `Default` for the `OrderTaskBy` because there could be more then one instance of the `Tasks` store with
different configuration and different needs. Hard coding that into `Default` would be exactly what Rust designers did to make our life hard while implementing our own
version of the data store.

Whole code after this step can be found in examples `todo_3_sorted_store`.
