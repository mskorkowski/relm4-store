use reexport::gtk;
use reexport::relm4;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use gtk::Box;
use gtk::CheckButton;
use gtk::Label;
use gtk::Orientation;
use gtk::prelude::AdjustmentExt;
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
use store::StoreView;
use store::StoreViewImplementation;
use store::math::Range;
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
    Scrolled,
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
    fn page_size(parent_view_model: &Self::ParentViewModel) -> usize;
}

pub struct TasksListViewModel<Config> 
where Config: TasksListConfiguration + 'static,
{
    tasks: Rc<RefCell<Tasks>>,
    new_task_description: gtk::EntryBuffer,
    store_view: Rc<RefCell<StoreViewImplementation<Self>>>,
    scroll_adjustment: gtk::Adjustment,
    _config: PhantomData<*const Config>,
}

impl<Config> ViewModel for TasksListViewModel<Config> 
where Config: TasksListConfiguration + 'static,
{
    type Msg = TaskMsg;
    type Widgets = TasksListViewWidgets<Config>;
    type Components = ();
}

impl<Config: TasksListConfiguration> FactoryConfiguration for TasksListViewModel<Config> 
where Config: TasksListConfiguration + 'static,
{
    type Store = Tasks;
    type RecordWidgets = TaskWidgets;
    type Root = gtk::Box;
    type View = gtk::Box;
    type Window = PositionTrackingWindow;
    type ContainerWidgets = TasksListViewWidgets<Config>;
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
            .margin_top(0)
            .margin_start(0)
            .margin_end(0)
            .margin_bottom(0)
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
            .margin_top(0)
            .margin_start(0)
            .margin_end(0)
            .margin_bottom(0)
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
            TaskMsg::Scrolled => {
                let value = self.scroll_adjustment.value();
                let page_size = self.scroll_adjustment.page_size(); 

                self.store_view.borrow().set_window(
                    Range::new(
                        value.floor() as usize,
                        (value+page_size).floor() as usize,
                    )
                );
            },
        }
    }

    fn init_view_model(parent_view_model: &Self::ParentViewModel, store_view: Rc<RefCell<StoreViewImplementation<Self>>>) -> Self {
        let view_length = store_view.borrow().len();

        TasksListViewModel{
            tasks: Config::get_tasks(parent_view_model),
            new_task_description: gtk::EntryBuffer::new(None),
            store_view,
            scroll_adjustment: gtk::Adjustment::new(0.0, 0.0, view_length as f64, 1.0, 1.0, Config::page_size(parent_view_model) as f64),
            _config: PhantomData,
        }
    }
}

pub struct TasksListViewWidgets<Config> 
where Config: TasksListConfiguration + 'static
{
    root: gtk::Box,
    input: gtk::Entry,
    viewport: gtk::Box,
    scrollbar: gtk::Scrollbar,
    scrolled_window: gtk::Box,
    config: PhantomData<*const Config>,
}

impl<Config> FactoryContainerWidgets<TasksListViewModel<Config>> for TasksListViewWidgets<Config> 
where Config: TasksListConfiguration + 'static,
{
    type Root = gtk::Box;

    fn init_view(
        view_model: &TasksListViewModel<Config>, 
        store_view: &StoreViewImplementation<TasksListViewModel<Config>>, 
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

        let viewport = gtk::Box::default();
        viewport.set_orientation(gtk::Orientation::Vertical);
        store_view.generate(&viewport, sender.clone());


        let scrolled_window = gtk::Box::default();
        scrolled_window.set_hexpand(true);
        scrolled_window.set_vexpand(true);
        
        let scrollbar = gtk::Scrollbar::default();
        scrollbar.set_orientation(gtk::Orientation::Vertical);
        scrollbar.set_adjustment(Some(&view_model.scroll_adjustment));

        {
            let sender = sender.clone();
            view_model.scroll_adjustment.connect_value_changed(move |_| {
                send!(sender, TaskMsg::Scrolled);
            });
        }

        
        TasksListViewWidgets {
            root,
            input,
            viewport,
            scrollbar,
            scrolled_window,
            config: PhantomData,
        }
    }
    
    fn connect_components(&self, _model: &TasksListViewModel<Config>, _components: &()) {
        
        self.root.append(&self.input);
        self.root.append(&self.scrolled_window);
        
        self.scrolled_window.append(&self.scrollbar);
        self.scrolled_window.append(&self.viewport);

    }

    fn view(&mut self, _view_model: &TasksListViewModel<Config>, _store_view: &StoreViewImplementation<TasksListViewModel<Config>>, _sender: Sender<<TasksListViewModel<Config> as ViewModel>::Msg>) {
        println!("Updating the view");
    }

    fn root_widget(&self) -> Self::Root {
        self.root.clone()
    }

    fn container_widget(&self) -> &<TasksListViewModel<Config> as FactoryConfiguration>::View {
        &self.viewport
    }
}

