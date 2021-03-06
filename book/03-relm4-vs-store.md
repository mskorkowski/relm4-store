# Relm4 vs stores

Stores are extension to relm4 but they introduce a separation between a business model and view model.

When you learn relm4 you start by throwing whole application state into model. This model is tightly related to view. You are forced to mix the application code and view related updates.

Stores introduces strict separation of business model and view. So in this book whenever we talk about view model we literally mean structures implementing `relm4::Model`. All imports will be done as `use relm4::Model as ViewModel`. When we talk about model or business model we will think about structures implementing `relm4_store_record::Record`.

## What is a benefit of this separation?

Record has stable identifier. This provides a few benefits like:

1. You know when you talk about same instance even if they are not binary equal.
2. Data becomes data and not the application state.

Both points are interrelated but slightly nuanced. Let's start from second point.

### Data becomes data

This is point of view of data users. They don't care about the id itself. They care about the business logic.

Since you have a stable id, you perfectly know when you talk about the same record. So you can keep a cloned values of your model. Update them and later commit them back or drop them at your leisure. You don't need to track information like `it's fifth item on list of emails in 3 item in list of users`. Just think a bit how much code you would need to write if you must add a record into users and user emails while you have a copy of your data laying somewhere around and you must track this relationship. Since you have a stable id, user model can trivially provide update email logic for things like this.

### Same instance even when not binary equal

This is point of view of data management. If you receive a record in a store you know what to do with a record. It either exists in a store or it's something new. This information is stored in records id and not a complex logic of passing data around.

So your store is committing an instance of user. Now since your user has stable id, you know if this record was stored without even looking into whatever your store backend is. Either final id is set or not. Your whole logic around business doesn't care about id's values. Whatever they are they are. What's important is if they are set they are known for eternity.

What's more new id is not empty. It has a value, so if you store your record in the database, or file, or some other service you just need to propagate a tuple of new id, permanent id to the places you store a copy of your record without tracking the position, nesting, or any other extra information. Your model logic have a way to find it wherever it is since it needs to be able to retrieve it in running application.

## List of differences

| relm4-store | relm4 |
|:------------|:------|
| **StoreViewComponent**: custom component type which knows how to work around all the state updates in presence of the `store-view` | **Component** |
| **StoreViewPrototype**: To make a store view/store view component behave. It's like a merge of `FactoryPrototype` and `ComponentUpdate`. | **FactoryPrototype** + **ComponentUpdate** |
| **StoreViewInnerComponent**: Extra trait extending `relm4::Components` required by the `StoreViewComponent`. It allows to send messages to the components in case store was updated. | N/A |
| **StoreViewContainerWidget**: Extra trait extending `relm4::Widgets` required by the `StoreViewComponent`. It returns the reference to the widget to which factory will add the widgets. | relm4 doesn't need it. Factory implementation can work it out in all cases |
