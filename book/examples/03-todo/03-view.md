# View

Now we need an ability to toggle the order of tasks in the view. So we will add two buttons one to sort ascending and other one to sort descending.

Let's start with imports. We need to add

```rust,noplaypen
use gtk::prelude::ButtonExt;
use relm4::send;
use store::OrderedStore;

use crate::store::OrderTasksBy; 
```

Since we need to sort tasks we need to add new messages to the `MainWindowMsg`.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_3/view/main_window.rs:12:15}}
```

Now we need to handle new messages in the `update` method of the `MainWindowViewModel`

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_3/view/main_window.rs:27:49}}
```

`SortedInMemoryStore` implements `OrderedStore` which provides a method `set_order` which you can use to set order of elements in the store. Implementation of the
`set_order` will take care of propagating the knowledge about the changes to the store view and in consequence render it on the screen.

Only thing left it to update our `MainWindowWidgets` so it can send the messages to sort our view.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_3/view/main_window.rs:82:103}}
```

To the titlebar of the main window we've added two buttons which will trigger sorting of the list. If you try adding new element to the list you will find that it's
being inserted accordingly to the state of the data store.

As always full code can be found in examples as `todo_3`.
