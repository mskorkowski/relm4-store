mod model;
mod store;
mod view;

use reexport::log;
use reexport::gtk;
use reexport::relm4;

use std::io::Result;

use relm4::RelmApp;

use crate::store::Tasks;
use crate::view::MainWindowViewModel;

fn main() -> Result<()> {
    log4rs::init_file("relm4-store-examples/examples/todo_1/etc/log4rs.yaml", Default::default()).unwrap();

    log::info!("");
    log::info!("Todo 1 example!");
    log::info!("");

    
    let app_id = "store.relm4.example.todo-1";
    
    gtk::init().expect("Couldn't initialize gtk");
    let application = gtk::Application::builder()
        .application_id(app_id)
        .build();

    let model = MainWindowViewModel{
        tasks: Tasks::new()
    };

    log::info!("\tCreating relm4 app");
    let app = RelmApp::with_app(model, application);

    log::info!("\tStarting app");
    app.run();

    Ok(())
}