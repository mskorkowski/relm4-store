# View - Task list

Our view will have a two parts. If you know `relm4` you will see lots of similarities here.

1. Task widget and tasks list
2. Main window

I've decided to split the view this way so each part of implementation is easier to understand.

## Store view

In this chapter we will implement our first store view. Store view is responsible for two things

- Selecting elements which should be visible
- Rendering this elements as fast as possible
- Making sure that visible elements are the one in store

In database analogy it would be a `SELECT` statement over the store which holds the data. Simplified interaction between the store and the view can be describe in pseudocode like this

```text
while true {
  await data_store.changed();
  view = SELECT * FROM data_store LIMIT view.start,view.page_size;
  for record in view {
    if record.has_changed() {
       update_ui(record);
    }
  }
}
```

From `relm4` point of view `StoreView` is a kind of factory.

## List of tasks

All snippets in this section should go to `view/task_list.rs`

### List of imports

There will be a lots of them here. I'm providing them here so they won't obstruct the examples later. We will cover all important parts later in this chapter.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/view/task_list.rs:1:33}}
```

### Task widget and task list

Firstly we need to define structures which will keep our widgets around.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/view/task_list.rs:35:62}}
```

Let's discuss it one by one.

The first thing is definition of the `StoreMsg` type. In your code you will interact with store. Main goal of this type alias is to reduce amount of typing. All stores and store views are using `store::StoreMsg` to communicate between each other and using it is only way to affect state of the data store. `store::StoreMsg` is parametrized by the type of Record so you won't be able to send a message of the wrong type to the store.

The second one is `TaskWidgets`. Exactly same structure you would be defining if you would use `relm4` factories. Checkbox to mark task as complete, label to keep task description and a box (root) to keep it together.

The third one is `TasksListConfiguration`. It's part of the component pattern to allow more then one instance of the component to be shown at the same time. It contains a method `get_tasks` which will return instance of the `store::Store`.

Finally `TasksListViewModel`. First really interesting things happens here. First attribute is `tasks` it's the data store which keeps all the data. We will need it to notify the store about status changes of the tasks. Second attribute is store_view. It will provide view into your store.

### `relm4::Model` implementation for `TasksListViewModel`

This part is obvious

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/view/task_list.rs:64:68}}
```

### `StoreViewPrototype`

In here we will implement `store::StoreViewPrototype` to provide a view for items in the store. You would use `StoreViewPrototype` trait in all the places where using pure `relm4` you would use `relm4::factory::FactoryPrototype`.

Differences between the `relm4::factory::FactoryPrototype` and `store::StoreViewPrototype`

|What|FactoryPrototype | StoreViewPrototype |
|:---|:----------------|:---------------|
| **Target of implementation** | You implement it for the ViewModel. | You implement it for whatever you like. This makes this interface behave more like configuration. |
| **Data container** | `type Factory` which points to data container in the ViewModel | `type Store` which points to data container type. There is no requirement of it being inside of `ViewModel`. |
| **Data visibility** | All data in the factory are visible. | Only part of data in the Store is visible. `type Window` defines how the view window behaves (more in chapter 2 and 3). |
| Method signature | Since you implemented the factory for the ViewModel, it takes `self` as an argument. You create a widgets to display `self`. Second is key under which factory is going to find it. Key is unstable and managed by the factory. | It's not bound to `self`. First is record for which widgets should be created. Second is position in the store. Position in the dataset at the time of widget generation. There is no guarantee to get the same widget in the future when asking store for record at the given position. Record is required to hold stable id by implementing `model::Identifialble`. |

Let's create a file `view/task.rs`

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/view/task_list.rs:70:193}}
```

Let's look at the first part of `StoreViewPrototype` implementation

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/view/task_list.rs:73:80}}
```

| type name | value | meaning |
|:----------|:------|:--------|
| Store     | `Tasks` | This type provides information about which store type will be used. This itself also provides information abut the model which (`DataStoreBase::Model`) which is used by the related store and as the consequence this view. In relm4's `FactoryPrototype` you would provide factory type where your data would be stored. |
| StoreView | `View<TasksListViewModel>` | This type provides information about which store view type will be used. In `relm4` this would be part of `FactoryPrototype`. It's responsible for providing view into the store |
| RecordWidgets | `TaskWidgets` | The same as in relm4's `FactoryPrototype::Widgets`. Type of structure holding all widgets. |
| Root    | `gtk::Box` | Type of widget which is a root for all widgets kept in the `RecordWidgets`. Same as in `FactoryPrototype::Root`. |
| View    | `gtk::Box` | Type of widgets which will keep the list of widgets. (The widget to which factory should add widgets to). Same as in `FactoryPrototype::View`. There must exist implementations of `relm4::factory::FactoryView` and `relm4::factory::FactoryListView` for `View`. |
| Window  | `PositionTrackingWindow` | Describes how the view window will behave in case of new data. For now use `PositionTrackingWindow` with annotation that if you don't know what to use, this one is probably the one. |
| ViewModel | `TasksListViewModel` | Provides information about type of view model used by implementation of the `StoreViewPrototype` |
| ParentViewModel | `Config::ParentViewModel` | Provides information about the parent view model. Used during initialization of the view model |

#### `init_store_view`

This method is responsible for creating instance of the store view. In here you connect your view with store and make sure your view has all required properties.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/view/task_list.rs:82:88}}
```

#### `relm4::factory::FactoryPrototype`

Next we implemented `init_view`, `view`, `position` and `root_widget` methods. All four methods are equivalents of the methods with the same name in `FactoryPrototype`.

#### `relm4::ComponentUpdate`

This method is equivalent of `update` for `ComponentUpdate`.  `init_view_model` is `init_model` from `ComponentUpdate`.

### TaskListViewWidgets

Now we can create our widgets for showing whole list

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/view/task_list.rs:195:219}}
```

There are only two interesting things here. First `StoreView` is kind of relm4 factory.

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/view/task_list.rs:214:214}}
```

Second is

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/view/task_list.rs:212:215}}
```

In here we've named container handling our list of tasks. It's important so the component knows which element to provide to relm4's `Factory::init_view` method.

Rest is classic relm4.

#### FactoryContainerWidgets

Now we need to implement extra trait `store::FactoryContainerWidgets`

```rust,noplaypen
{{#include ../../../relm4-store-examples/examples/todo_1/view/task_list.rs:221:230}}
```

In here we return reference to the widget used to keep whole list of our tasks
