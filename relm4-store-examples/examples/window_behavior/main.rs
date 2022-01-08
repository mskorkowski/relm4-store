mod model;
mod store;
mod view;

use backend_inmemory::SortedInMemoryBackend;
use reexport::gtk;
use reexport::log;
use reexport::relm4;

use relm4::RelmApp;

use crate::store::Tasks;
use crate::view::MainWindowViewModel;


fn main() {
    log4rs::init_file("relm4-store-examples/examples/todo_3/etc/log4rs.yaml", Default::default()).unwrap();

    log::info!("");
    log::info!("Todo 1 example!");
    log::info!("");

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
        tasks: Tasks::new(SortedInMemoryBackend::new()),
        page_size: 10,
    };

    println!("\tCreating relm4 app");
    let app = RelmApp::with_app(model, application);

    println!("\tStarting app");
    app.run();
}