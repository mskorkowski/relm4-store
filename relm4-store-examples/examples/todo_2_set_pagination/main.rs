mod model;
mod store;
mod view;

use reexport::log;
use reexport::gtk;
use reexport::relm4;

use std::io::Result;

use relm4::RelmApp;

use crate::store::TasksBuilder;
use crate::view::MainWindowViewModel;

fn main() -> Result<()> {
    log4rs::init_file("relm4-store-examples/examples/todo_2_set_pagination/etc/log4rs.yaml", Default::default()).unwrap();

    log::info!("");
    log::info!("Todo 2 example!");
    log::info!("");

    
    let app_id = "store.relm4.example.todo-2-set-pagination";
    
    gtk::init().expect("Couldn't initialize gtk");
    let application = gtk::Application::builder()
        .application_id(app_id)
        .build();

    let model = MainWindowViewModel{
        tasks: TasksBuilder::build()
    };

    log::info!("\tCreating relm4 app");
    let app = RelmApp::with_app(model, application);

    log::info!("\tStarting app");
    app.run();

    Ok(())
}