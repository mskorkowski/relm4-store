use record::DefaultIdAllocator;
use reexport::gtk;
use reexport::relm4;
use reexport::relm4_macros;

use std::cell::RefCell;
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
use relm4::Widgets;
use relm4::WidgetPlus;

use relm4_macros::widget;

use record::Id;
use record::Record;
use store::DataStore;
use store::FactoryConfiguration;
use store::FactoryContainerWidgets;
use store::Position;
use store::StoreView;
use store::math::Range;
use store::window::PositionTrackingWindow;
use store_view::StoreViewImplementation;

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
    store_view: Rc<RefCell<StoreViewImplementation<Self, DefaultIdAllocator>>>,
    scroll_adjustment: gtk::Adjustment,
}

impl<Config> ViewModel for TasksListViewModel<Config> 
where Config: TasksListConfiguration + 'static,
{
    type Msg = TaskMsg;
    type Widgets = TasksListViewWidgets;
    type Components = ();
}

impl<Config: TasksListConfiguration> FactoryConfiguration<DefaultIdAllocator> for TasksListViewModel<Config> 
where Config: TasksListConfiguration + 'static,
{
    type Store = Tasks;
    type StoreView = StoreViewImplementation<Self, DefaultIdAllocator>;
    type RecordWidgets = TaskWidgets;
    type Root = gtk::Box;
    type View = gtk::Box;
    type Window = PositionTrackingWindow;
    type ViewModel = Self;
    type ParentViewModel = Config::ParentViewModel;

    fn init_store_view(store: Rc<RefCell<Self::Store>>, size: store::StoreSize, redraw_sender: Sender<store::redraw_messages::RedrawMessages>) -> Self::StoreView {
        StoreViewImplementation::new(store, size.items(), redraw_sender)
    }

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

    fn update(view_model: &mut Self, msg: <Self as ViewModel>::Msg, _sender: Sender<<Self as ViewModel>::Msg>) {
        println!("[TasksListViewModel::update] message received, updating data");

        match msg {
            TaskMsg::New => {
                let description = view_model.new_task_description.text();
                let task = Task::new(description, false);
                view_model.new_task_description.set_text("");
                view_model.tasks.borrow().send(StoreMsg::Commit(task));
            },
            TaskMsg::Toggle{ complete, id } => {
                let tasks = view_model.tasks.borrow();
                if let Some(record) = tasks.get(&id) {
                    let mut updated = record.clone();
                    updated.completed = complete;
                    tasks.send(StoreMsg::Commit(updated));
                }
            },
            TaskMsg::Scrolled => {
                let value = view_model.scroll_adjustment.value();
                let page_size = view_model.scroll_adjustment.page_size(); 

                view_model.store_view.borrow().set_window(
                    Range::new(
                        value.floor() as usize,
                        (value+page_size).floor() as usize,
                    )
                );
            },
        }
    }

    fn init_view_model(parent_view_model: &Self::ParentViewModel, store_view: Rc<RefCell<StoreViewImplementation<Self, DefaultIdAllocator>>>) -> Self {
        let view_length = store_view.borrow().len();

        TasksListViewModel{
            tasks: Config::get_tasks(parent_view_model),
            new_task_description: gtk::EntryBuffer::new(None),
            store_view,
            scroll_adjustment: gtk::Adjustment::new(0.0, 0.0, view_length as f64, 1.0, 1.0, Config::page_size(parent_view_model) as f64),
        }
    }
}

#[widget(visibility=pub, relm4=reexport::relm4)]
impl<Config: TasksListConfiguration> Widgets<TasksListViewModel<Config>, Config::ParentViewModel> for TasksListViewWidgets {
    view!{
        root = gtk::Box {
            set_margin_all: 12,
            set_orientation: gtk::Orientation::Vertical,
            append = &gtk::Entry::with_buffer(&model.new_task_description) {
                connect_activate(sender) => move |_| { 
                    send!(sender, TaskMsg::New); 
                } 
            },
            append = &gtk::Box {
                set_hexpand: true,
                set_vexpand: true,

                append: scrollbar = &gtk::Scrollbar {
                    set_orientation: gtk::Orientation::Vertical,
                    set_adjustment: Some(&model.scroll_adjustment),
                },
                append: container = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    factory!(model.store_view.borrow())
                }
            },
        }
    }

    fn post_init() {
        {
            let sender = sender.clone();
            model.scroll_adjustment.connect_value_changed(move |_| {
                send!(sender, TaskMsg::Scrolled);
            });
        }
    }
}

impl<Config: 'static + TasksListConfiguration> FactoryContainerWidgets<TasksListViewModel<Config>, DefaultIdAllocator> for TasksListViewWidgets {
    fn container_widget(&self) -> &<TasksListViewModel<Config> as FactoryConfiguration<DefaultIdAllocator>>::View {
        &self.container
    }
}