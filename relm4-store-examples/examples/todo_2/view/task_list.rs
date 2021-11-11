use reexport::gtk;
use reexport::relm4;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use gtk::Box;
use gtk::CheckButton;
use gtk::Label;
use gtk::Orientation;
use gtk::prelude::BoxExt;
use gtk::prelude::CheckButtonExt;
use gtk::prelude::EntryExt;
use gtk::prelude::EntryBufferExtManual;
use gtk::prelude::OrientableExt;
use gtk::prelude::WidgetExt;

use relm4::Components;
use relm4::Model as ViewModel;
use relm4::RelmComponent;
use relm4::send;
use relm4::Sender;
use relm4::WidgetPlus;

use components::pagination::PaginationConfiguration;
use components::pagination::PaginationViewModel;
use record::Id;
use record::Record;
use store::DataStore;
use store::FactoryConfiguration;
use store::FactoryContainerWidgets;
use store::Position;
use store::StoreViewImplementation;
use store::window::PositionTrackingWindow;

use crate::model::Task;
use crate::store::Tasks;

type StoreMsg = store::StoreMsg<Task>;

pub enum TaskMsg {
    Toggle{
        complete: bool,
        id: Id<Task>,
    },
    New,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct TaskWidgets {
    checkbox: CheckButton,
    label: Label,
    root: Box,
}

pub trait TasksListConfiguration {
    type ParentViewModel: ViewModel;

    fn get_tasks(parent_view_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>>;
}

pub struct TasksListViewModel<Config> 
where Config: TasksListConfiguration + 'static,
{
    tasks: Rc<RefCell<Tasks>>,
    new_task_description: gtk::EntryBuffer,
    store_view: Rc<RefCell<StoreViewImplementation<TasksListViewWidgets<Config>, Self>>>,
}

impl<Config> ViewModel for TasksListViewModel<Config> 
where Config: TasksListConfiguration + 'static,
{
    type Msg = TaskMsg;
    type Widgets = TasksListViewWidgets<Config>;
    type Components = TasksListComponents<Config>;
}

impl<Config> FactoryConfiguration<TasksListViewWidgets<Config>> for TasksListViewModel<Config> 
where Config: TasksListConfiguration + 'static,
{
    type Store = Tasks;
    type RecordWidgets = TaskWidgets;
    type Root = gtk::Box;
    type View = gtk::Box;
    type Window = PositionTrackingWindow;
    type ParentViewModel = Config::ParentViewModel;


    fn generate(
        record: &Task,
        _position: Position,
        sender: Sender<TaskMsg>,
    ) -> Self::RecordWidgets {
        let root = Box::builder()
            .orientation(Orientation::Horizontal)
            .build();

        let checkbox = CheckButton::builder()
            .margin_top(12)
            .margin_start(12)
            .margin_end(12)
            .margin_bottom(12)
            .active(record.completed)
            .build();

        {
            let sender = sender.clone();
            let id = record.get_id();

            checkbox.connect_toggled(move |btn| {
                send!(sender, TaskMsg::Toggle{
                    id,
                    complete: btn.is_active()
                });
            });
        }

        let label = Label::builder()
            .margin_top(12)
            .margin_start(12)
            .margin_end(12)
            .margin_bottom(12)
            .label(&record.description)
            .build();

        root.append(&checkbox);
        root.append(&label);

        TaskWidgets {
            checkbox,
            label,
            root,
        }
    }

    /// Function called when record is modified.
    fn update_record(
        record: Task,
        _position: Position,
        widgets: &Self::RecordWidgets,
    ) {
        widgets.checkbox.set_active(record.completed);

        let attrs = widgets.label.attributes().unwrap_or_default();
        attrs.change(gtk::pango::Attribute::new_strikethrough(record.completed));
        widgets.label.set_attributes(Some(&attrs));
    }

    fn position(
        _model: Task, 
        _position: Position,
    ) {}

    /// Get the outermost widget from the widgets.
    fn get_root(widgets: &Self::RecordWidgets) -> &Self::Root {
        &widgets.root
    }

    fn update(&mut self, msg: Self::Msg, _sender: Sender<Self::Msg>) {
        println!("[TasksListViewModel::update] message received, updating data");

        match msg {
            TaskMsg::New => {
                let description = self.new_task_description.text();
                let task = Task::new(description, false);
                self.new_task_description.set_text("");
                self.tasks.borrow().inbox(StoreMsg::Commit(task));
            },
            TaskMsg::Toggle{ complete, id } => {
                let tasks = self.tasks.borrow();
                if let Some(record) = tasks.get(&id) {
                    let mut updated = record.clone();
                    updated.completed = complete;
                    tasks.inbox(StoreMsg::Commit(updated));
                }
            },
        }
    }

    fn init_view_model(parent_view_model: &Self::ParentViewModel, store_view: Rc<RefCell<StoreViewImplementation<TasksListViewWidgets<Config>, Self>>>) -> Self {
        TasksListViewModel{
            tasks: Config::get_tasks(parent_view_model),
            new_task_description: gtk::EntryBuffer::new(None),
            store_view,
        }
    }
}

pub struct TasksListComponents<Config>
where Config: TasksListConfiguration + 'static {
    pagination: RelmComponent<PaginationViewModel<Self>, TasksListViewModel<Config>>
}

impl<Config> Components<TasksListViewModel<Config>> for TasksListComponents<Config> 
where Config: TasksListConfiguration,
{
    fn init_components(
        parent_model: &TasksListViewModel<Config>, 
        parent_widget: &TasksListViewWidgets<Config>, 
        parent_sender: Sender<<TasksListViewModel<Config> as ViewModel>::Msg>
    ) -> Self {
        Self {
            pagination: RelmComponent::new(parent_model, parent_widget, parent_sender.clone()),
        }
    }
}

impl<Config> PaginationConfiguration for TasksListComponents<Config>
where Config: TasksListConfiguration + 'static {
    type ParentViewModel = TasksListViewModel<Config>;
    type ParentWidgets = TasksListViewWidgets<Config>;

    fn get_view(parent_view_model: &Self::ParentViewModel) -> Rc<RefCell<StoreViewImplementation<Self::ParentWidgets, Self::ParentViewModel>>> {
        parent_view_model.store_view.clone()
    }
}

pub struct TasksListViewWidgets<Config> 
where Config: TasksListConfiguration + 'static
{
    root: gtk::Box,
    input: gtk::Entry,
    scrolled_box: gtk::Box,
    scrolled_window: gtk::ScrolledWindow,
    config: PhantomData<*const Config>,
}

impl<Config> FactoryContainerWidgets<TasksListViewModel<Config>> for TasksListViewWidgets<Config> 
where Config: TasksListConfiguration + 'static,
{
    type Root = gtk::Box;

    fn init_view(
        view_model: &TasksListViewModel<Config>, 
        sender: Sender<<TasksListViewModel<Config> as ViewModel>::Msg>
    ) -> Self {
        let root = gtk::Box::default();
        root.set_margin_all(12);
        root.set_orientation(gtk::Orientation::Vertical);

        let input = gtk::Entry::with_buffer(&view_model.new_task_description);
        {
            let sender = sender.clone();
            input.connect_activate(move |_| send!(sender, TaskMsg::New));
        }

        let scrolled_box = gtk::Box::default();
        scrolled_box.set_orientation(gtk::Orientation::Vertical);
        view_model.store_view.borrow().generate(&scrolled_box, sender.clone());


        let scrolled_window = gtk::ScrolledWindow::default();
        scrolled_window.set_hexpand(true);
        scrolled_window.set_vexpand(true);
        
        
        TasksListViewWidgets {
            root,
            input,
            scrolled_box,
            scrolled_window,
            config: PhantomData,
        }
    }
    
    fn connect_components(&self, _model: &TasksListViewModel<Config>, components: &<TasksListViewModel<Config> as ViewModel>::Components) {
        
        self.root.append(&self.input);
        self.root.append(&self.scrolled_window);
        self.root.append(components.pagination.root_widget());
        self.scrolled_window.set_child(Some(&self.scrolled_box));

    }

    fn view(&mut self, _view_model: &TasksListViewModel<Config>, _sender: Sender<<TasksListViewModel<Config> as ViewModel>::Msg>) {
        println!("Updating the view");
    }

    fn root_widget(&self) -> Self::Root {
        self.root.clone()
    }

    fn container_widget(&self) -> &<TasksListViewModel<Config> as FactoryConfiguration<Self>>::View {
        &self.scrolled_box
    }
}