use reexport::gtk;
use reexport::relm4;
use reexport::relm4_macros;
use store_view::View;

use std::marker::PhantomData;

use gtk::Box;
use gtk::Button;
use gtk::CheckButton;
use gtk::Label;
use gtk::Orientation;
use gtk::prelude::BoxExt;
use gtk::prelude::ButtonExt;
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

use components::pagination::PaginationConfiguration;
use components::pagination::PaginationViewModel;
use components::pagination::PaginationMsg;
use record::Id;
use record::Record;
use store::StoreViewInnerComponent;
use store::window::WindowBehavior;
use store::DataStore;
use store::StoreViewPrototype;
use store::FactoryContainerWidgets;
use store::Position;

use crate::model::Task;
use crate::store::Tasks;

type StoreMsg = store::StoreMsg<Task>;

pub enum TaskMsg {
    Toggle{
        complete: bool,
        id: Id<Task>,
    },
    Delete(Id<Task>),
    New,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct TaskWidgets {
    checkbox: CheckButton,
    label: Label,
    delete_button: Button,
    root: Box,
}

pub trait TasksListConfiguration {
    type ParentViewModel: ViewModel;
    type Window: WindowBehavior;
    fn get_tasks(parent_view_model: &Self::ParentViewModel) -> Tasks;
}

pub struct TasksListViewModel<Config> 
where
    Config: TasksListConfiguration + 'static,
{
    tasks: Tasks,
    new_task_description: gtk::EntryBuffer,
    store_view: View<Self>,
    _config: PhantomData<*const Config>,
}

impl<Config> ViewModel for TasksListViewModel<Config> 
where
    Config: TasksListConfiguration + 'static,
{
    type Msg = TaskMsg;
    type Widgets = TasksListViewWidgets;
    type Components = TasksListComponents<Config>;
}

impl<Config> StoreViewPrototype for TasksListViewModel<Config> 
where
    Config: TasksListConfiguration + 'static,
{
    type Store = Tasks;
    type StoreView = View<Self>;
    type RecordWidgets = TaskWidgets;
    type Root = gtk::Box;
    type View = gtk::Box;
    type Window = Config::Window;
    type ViewModel = Self;
    type ParentViewModel = Config::ParentViewModel;

    fn init_store_view(store: Self::Store, size: store::StoreSize, redraw_sender: Sender<store::redraw_messages::RedrawMessages>) -> Self::StoreView {
        View::new(store, size, redraw_sender)
    }

    fn init_view(
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
            .hexpand(true)
            .xalign(0.0)
            .build();

        let delete_button = Button::builder()
            .margin_top(12)
            .margin_start(12)
            .margin_end(12)
            .margin_bottom(12)
            .icon_name("user-trash-symbolic")
            .build();
            
        delete_button.add_css_class("flat");

        {
            let sender = sender.clone();
            let id = record.get_id();

            delete_button.connect_clicked(move |_| {
                send!(sender, TaskMsg::Delete(id));
            });
        }

        root.append(&checkbox);
        root.append(&label);
        root.append(&delete_button);

        TaskWidgets {
            checkbox,
            label,
            delete_button,
            root,
        }
    }

    /// Function called when record is modified.
    fn view(
        record: Task,
        _position: Position,
        widgets: &Self::RecordWidgets,
    ) {
        widgets.checkbox.set_active(record.completed);

        let attrs = widgets.label.attributes().unwrap_or_default();
        attrs.change(gtk::pango::AttrInt::new_strikethrough(record.completed));
        widgets.label.set_attributes(Some(&attrs));
    }

    fn position(
        _model: Task, 
        _position: Position,
    ) {}

    /// Get the outermost widget from the widgets.
    fn root_widget(widgets: &Self::RecordWidgets) -> &Self::Root {
        &widgets.root
    }

    fn update(view_model: &mut Self, msg: <Self as ViewModel>::Msg, _sender: Sender<<Self as ViewModel>::Msg>) {
        match msg {
            TaskMsg::New => {
                let description = view_model.new_task_description.text();
                let task = Task::new(description, false);
                view_model.new_task_description.set_text("");
                view_model.tasks.send(StoreMsg::Commit(task));
            },
            TaskMsg::Toggle{ complete, id } => {
                let tasks = &view_model.tasks;
                if let Some(record) = tasks.get(&id) {
                    let mut updated = record.clone();
                    updated.completed = complete;
                    tasks.send(StoreMsg::Commit(updated));
                }
            },
            TaskMsg::Delete(id) => {
                let tasks = &view_model.tasks;
                tasks.send(StoreMsg::Delete(id));
            }
        }
    }

    fn init_view_model(parent_view_model: &Self::ParentViewModel, store_view: &View<Self>) -> Self {
        TasksListViewModel{
            tasks: Config::get_tasks(parent_view_model),
            new_task_description: gtk::EntryBuffer::new(None),
            store_view: store_view.clone(),
            _config: PhantomData,
        }
    }
}

pub struct TasksListComponents<Config>
where
    Config: TasksListConfiguration + 'static,
{
    pagination: RelmComponent<PaginationViewModel<Self>, TasksListViewModel<Config>>
}

impl<Config> Components<TasksListViewModel<Config>> for TasksListComponents<Config> 
where
    Config: TasksListConfiguration + 'static,
{
    fn init_components(
        parent_model: &TasksListViewModel<Config>, 
        parent_sender: Sender<<TasksListViewModel<Config> as ViewModel>::Msg>
    ) -> Self {
        Self {
            pagination: RelmComponent::new(parent_model, parent_sender.clone()),
        }
    }

    fn connect_parent(&mut self, _parent_widgets: &TasksListViewWidgets) {}
}

impl<Config> PaginationConfiguration for TasksListComponents<Config>
where
    Config: TasksListConfiguration + 'static,
{
    type StoreViewPrototype = TasksListViewModel<Config>;
    
    fn get_view(parent_view_model: &<Self::StoreViewPrototype as StoreViewPrototype>::ViewModel) -> View<Self::StoreViewPrototype> {
        parent_view_model.store_view.clone()
    }
}

impl<Config> StoreViewInnerComponent<TasksListViewModel<Config>> for TasksListComponents<Config>
where
    Config: TasksListConfiguration + 'static,
{
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
                    factory!(model.store_view)
                }
            },
            append: components.pagination.root_widget(),
        }
    }
}

impl<Config: 'static + TasksListConfiguration> FactoryContainerWidgets<TasksListViewModel<Config>> for TasksListViewWidgets {
    fn container_widget(&self) -> &<TasksListViewModel<Config> as StoreViewPrototype>::View {
        &self.container
    }
}