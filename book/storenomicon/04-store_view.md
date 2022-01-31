# StoreView

## Chapter status

Currently development is focused of store views. That makes this chapter rather loose bunch of thoughts. It might seem chaotic. If you find something hard to understand please rise an issue on gh repository. If you can't find an answer to how store view in given case works also.

## What store view is?

Store view is a glue layer between the store and ui. Store view is responsible for

- rendering records and doing it fast
- knowing which records should be rendered
- updating the view as records are being changed

## Page/Window

Store might contain a lot of data. Sometimes way more than can be loaded into the RAM directly. View keeps track of even smaller subset of the data. The data which view keeps track of is a page of data. Window is the range of data shown by the view.  

## Limitation

1. Page size and windows size are equal. One of the possible optimizations is to allow windows to have a smaller size then a page. This would allow to prepare the widgets before they are necessary and reduce lag to show the next record. You can find more about it in the `optimizing` section
2. Page/window size can't be changed in runtime (yet)

## Outer ordering

> **WARNING:** This part is not implemented yet. Please treat this section as my thoughts how it should work.

Store always have some natural order. It's expected that if you ask the store twice for records in range `[10, 20)` you will get the same 10 records (as long as store was not modified). For description how to deal with natural order of the store you might check example `todo_3` where you can read how to setup and change natural order of the store. There is always just one natural order of the store.

Outer ordering is the order which store view requested while asking for data.

> If your store has records `[(0, b), (1, z), (2, h), (3, a)]` where first value in a tuple is index of record in the store and second one is the record itself.
>
> Now if your view asks for first three records the store will return a series `[(0, b), (1, z), (2, h)]`.
>
> If your store is used by more then one store view you would not be keen on idea of changing the natural order since it will affect all views and as the consequence might
> mess them up. Probably what's more important users probably will not like this "feature". What you can do then is to ask view to be ordered. From this moment any request
> which is sent to the store will request certain order.
>
> If you choose to order records by content alphabetically while asking for first three records then your example store should return a series which looks like this
> `[(0, a), (1, b), (3, h)]`

### Why outer order?

> From maths point of view set is not ordered nor indexed. Indexing is a property of the series
>
> Series where
>
> \\[
> \forall i \in \mathbb{Z_{0+}} \quad e_i < e_{i+1}
> \\]
>
> is met is considered ordered.

In computer science you don't really have data which are really unordered. The order in which you lookup the data is some kind of order. When it's meaningless in your business domain then we consider it "unordered".

When you deal with a store and store view this things starts to matter. If you ask the store for the data without specifying any kind of an order you will get records ordered by the natural order which is implementation/backend specific. Now if you request data ordered then the returned result depends not only on the ordering which you requested but also the order in which records are present in the natural order. From data point of view the order which view is showing is the composition of function `outer_order(natural_order(data))` where `data` is the set containing your data. Since in the composition of functions this ordering is the outmost one so it's "outer order".

## Optimizing the store view

Store view can be optimized in a few ways

1. Rendering
2. Memory access
3. Data acquisition

### Rendering

Goal of rendering optimizations is reducing amount of time spent on the rendering. The biggest gain we can achieve there is to not perform rendering for records which are not in the view or their state didn't change.

1. View only renders a page of records
2. If there is a change in data view computes the difference between states and only applies the difference to the view

Rendering optimizations are done or almost done.

### Memory access

`DataContainer` from `relm4-store-collections` crate keeps copy of records in a `HashMap` indexed by id. Order of records is stored as vector of id's. All operations on datastore except adding/removing records are performed on id vector. This makes the algorithm work much faster then for the cases where big records are moved around in the ordered vector. The downside of this is HashMap which can easily loose the locality of memory access.

There is one thing which could be done to reduce amount of memory allocations. When inserting the records to the HashMap we clone them from the input vector. We could potentially remove them from this vector and assign them to the HashMap and remove the clone. For really big/expensive records that might me a good solution.

### Data acquisition

To show the responsive ui you need to be able to run it around 30 frames/s and responses to the changes must be available as soon as possible. If we assume that data store is backed by remote database server, responses coming back in 100-200 of milliseconds for properly optimized database are nothing strange. That means you can have 5-10 queries to database per second to respond to the user actions like scrolling. Now if you try to scroll fast the long list of records you might end up waiting a pretty long time.

Reducing amount of queries is one of the high priorities for near future.
