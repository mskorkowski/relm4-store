mod model;
mod store;
mod view;

use reexport::gtk;
use reexport::relm4;

use std::rc::Rc;
use std::cell::RefCell;

use relm4::RelmApp;

use crate::store::Tasks;
use crate::view::MainWindowViewModel;


fn main() {
    println!();
    println!("Todo 2 example!");
    println!();

    
    let app_id = "store.reml4.example.todo-2";
    
    gtk::init().expect("Couldn't initialize gtk");
    let application = gtk::Application::builder()
        .application_id(app_id)
        .build();

    println!("Seeding store");
    let model = MainWindowViewModel{
        tasks: Rc::new(
            RefCell::new(
                Tasks::new()
            )
        )
    };

    println!("\tCreating relm4 app");
    let app = RelmApp::with_app(model, application);

    println!("\tStarting app");
    app.run();
}