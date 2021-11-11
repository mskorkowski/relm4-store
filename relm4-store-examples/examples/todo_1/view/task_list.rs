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

use relm4::Model as ViewModel;
use relm4::send;
use relm4::Sender;
use relm4::WidgetPlus;

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

pub struct TasksListViewModel<Config: TasksListConfiguration> {
    tasks: Rc<RefCell<Tasks>>,
    new_task_description: gtk::EntryBuffer,
    _config: PhantomData<*const Config>
}

impl<Config: TasksListConfiguration> ViewModel for TasksListViewModel<Config> {
    type Msg = TaskMsg;
    type Widgets = TasksListViewWidgets<Config>;
    type Components = ();
}

impl<Config: TasksListConfiguration> FactoryConfiguration<TasksListViewWidgets<Config>> for TasksListViewModel<Config> {
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
                self.tasks.borrow().inbox(StoreMsg::New(task));
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

    fn init_view_model(parent_view_model: &Self::ParentViewModel, _store_view: Rc<RefCell<StoreViewImplementation<TasksListViewWidgets<Config>, Self>>>) -> Self {
        TasksListViewModel{
            tasks: Config::get_tasks(parent_view_model),
            new_task_description: gtk::EntryBuffer::new(None),
            _config: PhantomData,
        }
    }
}

pub struct TasksListViewWidgets<Config: TasksListConfiguration> {
    root: gtk::Box,
    _input: gtk::Entry,
    scrolled_box: gtk::Box,
    _scrolled_window: gtk::ScrolledWindow,
    _config: PhantomData<*const Config>,
}

impl<Config: TasksListConfiguration> FactoryContainerWidgets<TasksListViewModel<Config>> for TasksListViewWidgets<Config> {
    type Root = gtk::Box;

    fn init_view(view_model: &TasksListViewModel<Config>, store_view: &StoreViewImplementation<Self, TasksListViewModel<Config>>, sender: Sender<<TasksListViewModel<Config> as ViewModel>::Msg>) -> Self {
        let root = gtk::Box::default();
        root.set_margin_all(12);
        root.set_orientation(gtk::Orientation::Vertical);

        let _input = gtk::Entry::with_buffer(&view_model.new_task_description);
        {
            let sender = sender.clone();
            _input.connect_activate(move |_| send!(sender, TaskMsg::New));
        }

        let scrolled_box = gtk::Box::default();
        scrolled_box.set_orientation(gtk::Orientation::Vertical);

        store_view.generate(&scrolled_box, sender.clone());


        let _scrolled_window = gtk::ScrolledWindow::default();
        _scrolled_window.set_hexpand(true);
        _scrolled_window.set_vexpand(true);
        _scrolled_window.set_child(Some(&scrolled_box));

        root.append(&_input);
        root.append(&_scrolled_window);

        TasksListViewWidgets {
            root,
            _input,
            scrolled_box,
            _scrolled_window,
            _config: PhantomData,
        }
    }

    fn view(&mut self, _view_model: &TasksListViewModel<Config>, _store_view: &StoreViewImplementation<Self, TasksListViewModel<Config>>, _sender: Sender<<TasksListViewModel<Config> as ViewModel>::Msg>) {
        println!("Updating the view");
    }

    fn root_widget(&self) -> Self::Root {
        self.root.clone()
    }

    fn container_widget(&self) -> &<TasksListViewModel<Config> as FactoryConfiguration<Self>>::View {
        &self.scrolled_box
    }
}