# Application architecture

In `relm4` architecture looks like this:

```asciiart
+--------------+        +------+
| relm4::Model |--------| View |
+--------------+        +------+
```

View sends events to view model and view model updates it state base on that which in turn modifies view. ViewModel combines two roles housekeeping of data in the app and controller of app and view. This two roles are not mixed up thanks to Rust language itself. `relm4::ComponentUpdate` interface implementation is the controller part and structure implementing `relm4::Model` is doing the application model part.

When you use this library then this changes into

```asciiart
+-------+    +-------+    +----------------+    +------+
| Model |----| Store |----|    ViewModel   |----| View |
|       |    |       |    | (relm4::Model) |    |      |
+-------+    +-------+    +----------------+    +------+
```

The responsibilities of the view `relm4::Model` has been shrunk in terms of data management.

## Let's talk about store a bit more

Store is sharable data set. Store itself can is just a bunch of data. You can't see a store. You can't enumerate the store. No point for it. Store can be an interface to some http service or db access layer or whatever else.

What you can see is a store view. Store view tracks a subset of the data from the store. This is an equivalent of the relm4's factory (it even implements one). This allows you to have multiple different views over single store.
