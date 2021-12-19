# Application architecture

When you write your application in `relm4` you use classic MVC pattern. `relm4-store` introduces some "proxies" into the picture.

In `relm4` terms it looks like this

![relm4 MVC](./assets/mvc.png)

View sends events to component in reaction to user input and component updates the model witch in turn modifies view. `relm4::Model` combines two roles housekeeping of data in the app and controller of app and view. This two roles are not mixed up thanks to Rust language itself. `relm4::ComponentUpdate` interface implementation is the controller part and structure implementing `relm4::Model` is doing the application model part.

When you use this library then this changes into

![relm4-store MVC](./assets/mvc-store.png)

## Let's talk about store a bit more

Store is sharable data set. Store itself can is just a bunch of data. You can't enumerate the store. No point for it. Store can be an interface to some http service or db access layer or whatever else.

What you can show is a store view. Store view tracks a subset of the data from the store. This is an equivalent of the relm4's factory (it even implements one). This allows you to have multiple different views over single store. StoreView is a proxy between the view and business model.
