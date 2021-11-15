mod model;
mod store;
mod view;

use reexport::gtk;
use reexport::relm4;

use relm4::RelmApp;

use crate::store::Tasks;
use crate::view::MainWindowViewModel;


fn main() {
    println!();
    println!("Todo 3 example!");
    println!();

    
    let app_id = "store.relm4.examples.todo-3";
    
    gtk::init().expect("Couldn't initialize gtk");
    let application = gtk::Application::builder()
        .application_id(app_id)
        .build();

    println!("Seeding store");
    let model = MainWindowViewModel{
        tasks: Tasks::new(),
        page_size: 10,
    };

    println!("\tCreating relm4 app");
    let app = RelmApp::with_app(model, application);

    println!("\tStarting app");
    app.run();
}