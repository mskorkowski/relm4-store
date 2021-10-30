use reexport::{gtk, relm4, relm4_macros};
use std::{cell::RefCell, rc::Rc};
use gtk::prelude::{BoxExt, EntryExt, EntryBufferExtManual, OrientableExt, WidgetExt};
use relm4::{Components, ComponentUpdate, Model as ViewModel, RelmComponent, send, Sender, Widgets, WidgetPlus};
use relm4_macros::widget;
use store::{DataStoreBase, Source, StoreViewInterface, math::Window};
use components::pagination::PaginationViewModel;
use components::pagination::PaginationConfiguration;
use crate::{ model::Task, store::Tasks, view::task::{TaskFactoryBuilder, TaskMsg}};

type StoreMsg = store::StoreMsg<Task>;

pub trait TasksListConfiguration: Source<SV=StoreViewInterface<TaskFactoryBuilder<Self::Window>>> 
where
    <Self as TasksListConfiguration>::Window: 'static
{
    type Window: Window;
    fn get_tasks(parent: &Self::ParentViewModel) -> Rc<RefCell<Tasks>>;
    fn ping_parent_message() -> <Self::ParentViewModel as ViewModel>::Msg;
}

pub struct TasksListViewModel<Config>
where 
    Config: TasksListConfiguration,
{
    view: Rc<Config::SV>,
    tasks: Rc<RefCell<Tasks>>,
    new_task_description: gtk::EntryBuffer,
}

impl<Config> ViewModel for TasksListViewModel<Config>
where Config: TasksListConfiguration + 'static,
{
    type Msg = TaskMsg;
    type Widgets = TasksListViewWidgets;
    type Components = TasksListComponents<Config>;
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
    type ParentViewModel = TasksListViewModel<Config>;
    type SV = Config::SV;

    fn get_view(parent_view_model: &Self::ParentViewModel) -> Rc<Self::SV> {
        parent_view_model.view.clone()
    }

    fn update_message() -> <Self::ParentViewModel as ViewModel>::Msg {
        TaskMsg::Reload
    }
}

impl<Config> ComponentUpdate<Config::ParentViewModel> for TasksListViewModel<Config>
where Config: TasksListConfiguration + 'static,
{
    fn init_model(parent_model: &Config::ParentViewModel) -> TasksListViewModel<Config> {
        TasksListViewModel{
            view: Rc::new(Config::store(parent_model)),
            tasks: Config::get_tasks(parent_model),
            new_task_description: gtk::EntryBuffer::new(None),
        }
    }

    fn update(
        &mut self,
        msg: TaskMsg,
        _components: &TasksListComponents<Config>,
        _sender: Sender<TaskMsg>,
        parent_sender: Sender<<Config::ParentViewModel as ViewModel>::Msg>
    ) {
        match msg {
            TaskMsg::New => {
                let description = self.new_task_description.text();
                let task = Task::new(description, false);
                self.new_task_description.set_text("");
                self.tasks.borrow().inbox(StoreMsg::New(task));

                //force redraw of the components
                send!(parent_sender, Config::ping_parent_message());
            },
            TaskMsg::Toggle{ complete, id } => {
                let tasks = self.tasks.borrow();
                if let Some(record) = tasks.get(&id) {
                    let mut updated = record.clone();
                    updated.completed = complete;
                    tasks.inbox(StoreMsg::Commit(updated));

                    //force redraw of the components
                    send!(parent_sender, Config::ping_parent_message());
                }
            },
            TaskMsg::Reload => {}
        }
    }
}

#[widget(visibility=pub, relm4 = relm4)]
impl<Config> Widgets<TasksListViewModel<Config>, Config::ParentViewModel> for TasksListViewWidgets
where Config: TasksListConfiguration + 'static,
{
    view!{
        root = &gtk::Box {
            set_margin_all: 12,
            set_orientation: gtk::Orientation::Vertical,
            append: input = &gtk::Entry::with_buffer(&model.new_task_description) {
                connect_activate(sender) => move |_| {
                    send!(sender, TaskMsg::New)
                },  
            },
            append = &gtk::ScrolledWindow {
                set_child = Some(&gtk::Box) {
                    set_orientation: gtk::Orientation::Vertical,
                    factory!(model.view as &Config::SV),
                },
                set_hexpand: true,
                set_vexpand: true,
            },
            append: component!(components.pagination.root_widget())
        }
    }
}