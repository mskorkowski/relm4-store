# Wrapping it up

main.rs

```rust
use reexport::gtk;
use reexport::relm4;

use relm4::RelmApp;

use std::rc::Rc;
use std::cell::RefCell;

use crate::store::TasksBuilder;
use crate::view::MainWindowViewModel;


fn main() {
    println!();
    println!("Todo 1 example!");
    println!();

    
    let app_id = "com.constellationsoft.store.example.todo-1";
    
    gtk::init().expect("Couldn't initialize gtk");
    let application = gtk::Application::builder()
        .application_id(app_id)
        .build();

    let model = MainWindowViewModel{
        tasks: Rc::new(RefCell::new(TasksBuilder::build()))
    };

    let app = RelmApp::with_app(model, application);
    app.run();
}
```

During creation of main window model, we've created the store with tasks.

```rust
let model = MainWindowViewModel{
    tasks: Rc::new(RefCell::new(TasksBuilder::build()))
};
```

## Summary

- You've learnt how to create an in memory store
- You've learnt how to create a store view and show the data in the store
