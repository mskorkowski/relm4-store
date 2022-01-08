mod model;
mod store;
mod view;

use backend_inmemory::InMemoryBackend;
use reexport::gtk;
use reexport::relm4;

use std::io::Write;

use relm4::RelmApp;

use crate::store::Tasks;
use crate::view::MainWindowViewModel;


fn main() {
    println!();
    println!("Todo 2 example!");
    println!();

    
    let app_id = "store.relm4.example.todo-2-single-scroll";
    
    gtk::init().expect("Couldn't initialize gtk");
    let application = gtk::Application::builder()
        .application_id(app_id)
        .build();

    println!("Building model");
    std::io::stdout().flush().unwrap();
    let model = MainWindowViewModel{
        tasks: Tasks::new(InMemoryBackend::new())
    };
    println!("\tDone");
    std::io::stdout().flush().unwrap();
    

    println!("\tCreating relm4 app");
    std::io::stdout().flush().unwrap();
    let app = RelmApp::with_app(model, application);

    println!("\tStarting app");
    app.run();
}