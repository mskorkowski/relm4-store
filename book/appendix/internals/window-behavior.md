# WindowBehavior

`WindowBehavior` is defined in the `relm4-store/src/window/mod.rs` file.

```rust
pub trait WindowBehavior {
    fn insert(r: &Range, p: &Point) -> WindowTransition;
    fn remove(r: &Range, p: &Point) -> WindowTransition;
    fn slide(r: &Range, moved: &Range) -> WindowTransition;
}
```

Implementation of this trait define how store view would behave in case of
new data being present (methods `insert`, `remove`) and in case of moving
the view across the data store content `slide`.

`WindowBehavior::insert` or `WindowBehavior::remove` are triggered when order
of elements might change due to operations on the store.

`WindowBehavior::slide` is called when order of elements was not changed and
user action would change the set of records visible by changing the view range.

## Why implement that?

When you insert record into the data store view must decide, wherever inserting
this record affects the view or not. Inserting record might propagate far away
from insertion place.

For example if you use `PositionTrackingWindow` adding any records before the visible
range will cause the showing up a new record on the left/top and dropping record from
the right/bottom. On the other hand you would ignore any record added after the
visible range.

The same logic applies for the removal of the record in the store.

Sliding is slightly more abstract from data change point of view. Basically you change
the data visible to the user. But you also have a decision to make how much past
the data user can scroll. You can see it in some text editors (for example VScode)
allows you to scroll the text in such a way that only last line of text is visible.
On the other hand gnomes gedit would only allow you to scroll up to the
last line (so you would always see last page of text). Which is better? It depends
on the application and your users that's why you get the option to set it up.
(and also because I hate some software which did it wrong in my opinion ;) ).

## When the WindowBehavior methods are called

It's decided by the method `StoreViewImpl::convert_to_transition`. Which is called for
every message sent to a view. At the time of writing it was like this:

```rust
fn convert_to_transition(&self, range: &Range, message: &StoreMsg<<Builder::Store as DataStoreBase>::Model>) -> WindowTransition {
    match message {
        StoreMsg::New(_record) => {
            Builder::Window::insert(range, &Point::new(self.view_data.borrow().len()))
        },
        StoreMsg::NewAt(p) => {
            Builder::Window::insert(range, &p.to_point())
        },
        StoreMsg::Move{from, to} => {
            Builder::Window::slide(range, &Range::new(from.0, to.0))
        },
        StoreMsg::Reorder{from, to} => {
            Builder::Window::slide(range, &Range::new(from.0, to.0))
        },
        StoreMsg::Remove(at) => {
            Builder::Window::remove(range, &at.to_point())
        },
        StoreMsg::Commit(_) => {
            WindowTransition::Identity
        },
        StoreMsg::Update(_) => {
            WindowTransition::Identity
        },
        StoreMsg::Reload => {
            WindowTransition::Identity
        },
    }
}
```
