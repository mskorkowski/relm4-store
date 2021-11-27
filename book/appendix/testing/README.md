# Libraries

To use `gtk-test` you will need

- `libxdo`, on Fedora `dnf install libxdo libxdo-devel`

## Testing code using `glib::Sender`

Template of the test using `glib` senders without `gtk` involved

```rust

#[test]
fn test_sender() {
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    {
        let context = glib::MainContext::default();
        let _guard = context.acquire().unwrap(); // required so attach won't panic
                                                 // normally gtk::init() is doing it for you

        receiver.attach(Some(&context), move |_msg| {
            glib::Continue(false) // false prevents awaiting for the events if there is no events present
        });
    }

    ...
    sender.send(message).unwrap();
    ...

    {
        let context = glib::MainContext::default();
        context.iteration(false); // this will move glib main loop forward
    }
}

```

If the test depends on the code which invokes `receiver.attach` then you must invoke `let _guard = context.acquire().unwrap();` at the beginning of the test and keep it alive until the end of it. Otherwise you will be hit by the `assertion failed: context.is_owner()`.

```rust

#[test]
fn test_sender() {
    let context = glib::MainContext::default();
    let _guard = context.acquire().unwrap();

    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    {
        receiver.attach(Some(&context), move |_msg| {
            glib::Continue(false) // false prevents awaiting for the events if there is no events present
        });
    }

    ...
    sender.send(message).unwrap();
    ...

    {
        let context = glib::MainContext::default();
        context.iteration(false); // this will move glib main loop forward
    }
}

```

While running the tests which touch upon `glib::MainContext::default(); context.iteration(true);` and friends you **must** run them with `--test-threads=1`.

Example:

```text
RUST_BACKTRACE=1 cargo tarpaulin --packages relm4-store-backend-dummy --exclude-files relm4-store/src/* --exclude-files relm4-store-backend-inmemory/* --exclude-files relm4-store-c* --exclude-files relm4-store-e* --exclude-files relm4-store-r* --exclude-files relm4-store-v* -o lcov -- --test-threads=1
```

Otherwise you are going to hit an error `Value accessed from different thread than where it was created`

## Where to place tests

Tests using `gtk` and/or `gtk_test` must live as integration tests. If you try to place them in the package itself gtk will fail to initialize.
