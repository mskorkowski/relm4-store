# Ordering

## Natural order

Natural order is the order in which your store returns data when no sorting is applied. The important thing to remember is if you implement your store
make sure natural order is stable. Otherwise to make your application useful either you would need to make sure you always apply sorting or you might
end up with application where every time interface is refreshed due to data changes being propagated, your whole ui shuffles. This will be terrible
from user point of view.

All examples until now we were using `InMemoryBackend` for which natural order is defined as order in which records where added. In this example we will switch to
`SortedInMemoryBackend` where we can change natural order of elements.

## Sorting behavior

There are two ways in which sorting work while you use data store. First is global sorting or natural order. You apply it to the store and store rebuilds itself so
natural order becomes the one which you had requested. This will cause update in all store views. Second way is to apply sorting to the store view.
This will make store view from now to ask the data to be ordered by the given property. Result seen by the view is equivalent of
`store view ordering(natural order(data))`.

If you define the ordering what was originally append at the end of the store might become insert somewhere in the middle of the data set. This might
lead to undesired side effects on the ui side. To give you power to decide what should happen you can use implementation of `store::window::WindowBehavior`.
By default in `relm4-store` there are implemented four kinds of window behaviors.

1. PositionTrackingWindow
2. ValueTrackingWindow
3. KeepOnTop
4. KeepOnBottom

### `store::window::PositionTrackingWindow`

It keeps position of the view in the data store. So if you are showing records 5-10 you will always show records from 5-10 whatever they are. To understand
it better let's talk about it on examples.

All examples start with data store containing data `[0->a, 1->b, 2->c, 3->d, 4->e, 5->f, 6->g]`. Our view shows the range `[2, 5)` so user can see `[c, d, e]`.

If user inserts data at position before the range shown by the view for example at position `1 -> a'` the data store then looks like
`[0->a, 1->a', 2->b, 3->c, 4->d, 5->e, 6->f, 7->g]`. View is expected to always show records from `[2, 5)` so view will be updated to user can see `[b, c, d]`.

If user inserts data inside the range `[2, 5)`. For example `3->c'` then ui will be updated so user can see `[c, c', d]`.

If user inserts at position `5` or higher ui will not be updated since there is nothing to do.

This kind of behavior works the best when you use some kind of pagination widget to navigate the data.

### `store::window::ValueTrackingWindow`

This kind of window behavior tries to keep current data in the view as much as possible.

All examples start with data store containing data `[0->a, 1->b, 2->c, 3->d, 4->e, 5->f, 6->g]`. Our view shows the range `[2, 5)` so user can see `[c, d, e]`.

If you insert data at position before range start for example `1->a'` then position of the view will be adjusted to `[3, 6)` so data visible won't change.

If you insert data in the range then nothing we can do about data preservation. For example `3->c'` will end up as `[c, c', d]`.

If you insert data at position `5` or hight ui will not be updated and store view will still show the same range.

This kind of behavior works the best with scrolling. It makes interface feel more static then with `PositionTrackingWindow`. If you have a data generated at
the time user is seeing them and for some reason you can't use pagination this will be the best choice for you. Your users will scroll to the data they are
interested into and whatever happens to the data the view will try it's best to keep them in scope.

### `store::window::KeepOnBottom`

Keeps the view at the certain distance from the end of data. This way if your sorting is stable in terms of adding new records users will see a log like view
of data.

All examples start with data store containing data `[0->a, 1->b, 2->c, 3->d, 4->e, 5->f, 6->g]`. Our view shows the range `[2, 5)` so user can see `[c, d, e]`.

If you add the record at `1->a'` then change would be ignored since distance from the end of the data did not change.

If you add the record inside the data range for example `3->c'` then the data store looks like `[0->a, 1->b, 2->c, 3->c', 4->d, 5->e, 6->f, 7->g]` this means that
to keep the distance from the end we need to slide the window to the right so the range would be `[3, 6)` which in consequence show the user a data `[c', d, e]`.

If you add the record at position after the data range shown to the user, for example `7->h` to keep the distance from the end we need to slide the window to the
right so user would see `[d. e. f]`.

It's intended for log like views. It's useful in special cases.

### `store::window::KeepOnTop`

It's reverse of `KeepOnBottom` where the distance kept is from the beginning of the data (from position `0`). It's another special case behavior.
