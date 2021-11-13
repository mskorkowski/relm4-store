use reexport::gtk;
use reexport::relm4;
use reexport::relm4_macros;

use std::cell::RefCell;
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
use relm4::Widgets;
use relm4::WidgetPlus;

use relm4_macros::widget;

use components::pagination::PaginationMsg;
use components::pagination::PaginationConfiguration;
use components::pagination::PaginationViewModel;
use record::Id;
use record::Record;
use store::DataStore;
use store::FactoryConfiguration;
use store::FactoryContainerWidgets;
use store::Position;
use store::StoreViewImplementation;
use store::StoreViewInnerComponent;
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
    store_view: Rc<RefCell<StoreViewImplementation<Self>>>,
}

impl<Config> ViewModel for TasksListViewModel<Config> 
where Config: TasksListConfiguration + 'static,
{
    type Msg = TaskMsg;
    type Widgets = TasksListViewWidgets;
    type Components = TasksListComponents<Config>;
}

impl<Config> FactoryConfiguration for TasksListViewModel<Config> 
where Config: TasksListConfiguration + 'static,
{
    type Store = Tasks;
    type RecordWidgets = TaskWidgets;
    type Root = gtk::Box;
    type View = gtk::Box;
    type Window = PositionTrackingWindow;
    type ViewModel = Self;
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

    fn update(view_model: &mut Self, msg: <Self as ViewModel>::Msg, _sender: Sender<<Self as ViewModel>::Msg>) {
        println!("[TasksListViewModel::update] message received, updating data");

        match msg {
            TaskMsg::New => {
                let description = view_model.new_task_description.text();
                let task = Task::new(description, false);
                view_model.new_task_description.set_text("");
                view_model.tasks.borrow().inbox(StoreMsg::Commit(task));
            },
            TaskMsg::Toggle{ complete, id } => {
                let tasks = view_model.tasks.borrow();
                if let Some(record) = tasks.get(&id) {
                    let mut updated = record.clone();
                    updated.completed = complete;
                    tasks.inbox(StoreMsg::Commit(updated));
                }
            },
        }
    }

    fn init_view_model(parent_view_model: &Self::ParentViewModel, store_view: Rc<RefCell<StoreViewImplementation<Self>>>) -> Self {
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
        parent_widget: &TasksListViewWidgets, 
        parent_sender: Sender<<TasksListViewModel<Config> as ViewModel>::Msg>
    ) -> Self {
        Self {
            pagination: RelmComponent::new(parent_model, parent_widget, parent_sender.clone()),
        }
    }
}

impl<Config> PaginationConfiguration for TasksListComponents<Config>
where Config: TasksListConfiguration + 'static {
    type FactoryConfiguration = TasksListViewModel<Config>;

    fn get_view(parent_view_model: &<Self::FactoryConfiguration as FactoryConfiguration>::ViewModel) -> Rc<RefCell<StoreViewImplementation<Self::FactoryConfiguration>>> {
        parent_view_model.store_view.clone()
    }
}

impl<Config> StoreViewInnerComponent<TasksListViewModel<Config>> for TasksListComponents<Config>
where Config: TasksListConfiguration + 'static {
    fn on_store_update(&mut self) {
        self.pagination.send(PaginationMsg::StoreUpdated).unwrap();
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
            append = &gtk::ScrolledWindow {
                set_hexpand: true,
                set_vexpand: true,
                set_child: container = Some(&gtk::Box) {
                    set_orientation: gtk::Orientation::Vertical,
                    factory!(model.store_view.borrow())
                }
            },
            append: component!(pagination)
        }
    }
}

impl<Config: TasksListConfiguration> FactoryContainerWidgets<TasksListViewModel<Config>> for TasksListViewWidgets {
    fn container_widget(&self) -> &<TasksListViewModel<Config> as FactoryConfiguration>::View {
        &self.container
    }
}