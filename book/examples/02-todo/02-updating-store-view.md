# Updating store view

## Preface

Now our application needs to deal with quite a few records to display. In the real world there are three basic strategies to deal with big number of records and make ui reasonable to users.

1. Pagination
2. Filtering
3. Grouping

### Pagination

When you use pagination you reduce visible amount of data to value which are known to the store view. This way you we don't need to render all of the widgets from which most of them are not visible. Proper size of page, makes it human understandable and makes the application fast. You should use pagination when the amount of data in the store is much more then you can show on the screen (in most of the cases it means always). In this chapter we will focus on pagination.

### Filtering

Filtering on the other hand reduces amount of data which store can provide to the store view. For humans it gives an option to see only elements in the store which share some property. How to add filtering will be discussed in chapter 4. You should use filtering whenever your store may contain more the 10 elements (again almost always). This allows user to find the data he's interested into.

### Grouping

Grouping provides better conceptual view for data in the store by splitting them into smaller subsets with common property. You might use it when there are natural groups defined.

For example you have a store of files in transactions. Single file belongs to one transaction but transaction can have multiple files. This gives natural grouping of files by transactions to which they belong to.

Other example could be accounting system. Let's assume you have a store with invoices. Now you might group them by the accounting month to which they belong to.

## Let's implement pagination

In the `view/main_window.rs` we change `StoreSize::Unlimited` into `StoreSize::Items(50)`

```rust
impl Source for MainWindowComponents {
    type ParentViewModel = MainWindowViewModel;
    type SV = StoreViewInterface<TaskFactoryBuilder>;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV {
        StoreViewInterface::new(parent_model.tasks.clone(), StoreSize::Items(50))
    }
}
```

If you start an application now you will find that only first 50 records were shown and there is no way for you to go past that. New tasks added are not shown either.

You can't see more then 50 records since we've limited our view to 50 records. New tasks are added at the end of the store so at the positions 10000+. This is definitely way above range [0, 50) which is being shown.

