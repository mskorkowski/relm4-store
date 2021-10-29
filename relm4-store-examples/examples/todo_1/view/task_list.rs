use reexport::{gtk, relm4, relm4_macros};
use std::{cell::RefCell, rc::Rc};
use gtk::prelude::{BoxExt, EntryExt, EntryBufferExtManual, OrientableExt, WidgetExt};
use relm4::{ComponentUpdate, Model as ViewModel, send, Sender, Widgets, WidgetPlus};
use relm4_macros::widget;
use store::{DataStoreBase, Source, StoreViewInterface};
use crate::{ model::Task, store::Tasks, view::task::{TaskFactoryBuilder, TaskMsg}};

type StoreMsg = store::StoreMsg<Task>;

pub trait TasksListConfiguration: Source<SV=StoreViewInterface<TaskFactoryBuilder>> {
    fn get_tasks(parent: &Self::ParentViewModel) -> Rc<RefCell<Tasks>>;
}

pub struct TasksListViewModel<Config> 
where Config: TasksListConfiguration,
{
    view: Config::SV,
    tasks: Rc<RefCell<Tasks>>,
    new_task_description: gtk::EntryBuffer,
}

impl<Config> ViewModel for TasksListViewModel<Config>
where Config: TasksListConfiguration,
{
    type Msg = TaskMsg;
    type Widgets = TasksListViewWidgets;
    type Components = ();
}

impl<Config> ComponentUpdate<Config::ParentViewModel> for TasksListViewModel<Config>
where Config: TasksListConfiguration,
{
    fn init_model(parent_model: &Config::ParentViewModel) -> TasksListViewModel<Config> {
        TasksListViewModel{
            view: Config::store(parent_model),
            tasks: Config::get_tasks(parent_model),
            new_task_description: gtk::EntryBuffer::new(None),
        }
    }

    fn update(
        &mut self,
        msg: TaskMsg,
        _components: &(),
        _sender: Sender<TaskMsg>,
        _parent_sender: Sender<<Config::ParentViewModel as ViewModel>::Msg>
    ) {
        match msg {
            TaskMsg::New => {
                let description = self.new_task_description.text();
                let task = Task::new(description, false);
                self.new_task_description.set_text("");
                self.tasks.borrow().inbox(StoreMsg::New(task));
            },
            TaskMsg::Toggle{ complete, id } => {
                let tasks = self.tasks.borrow();
                if let Some((_, record)) = tasks.get(&id) {
                    let mut updated = record.clone();
                    updated.completed = complete;
                    tasks.inbox(StoreMsg::Commit(updated));
                }
            },
        }
    }
}

#[widget(visibility=pub, relm4 = relm4)]
impl<Config> Widgets<TasksListViewModel<Config>, Config::ParentViewModel> for TasksListViewWidgets
where Config: TasksListConfiguration,
{
    view!{
        root = &gtk::Box {
            set_margin_all: 12,
            set_orientation: gtk::Orientation::Vertical,
            append: input = &gtk::Entry::with_buffer(&model.new_task_description) {
                connect_activate => move |_| {
                    send!(sender, TaskMsg::New)
                },  
            },
            append = &gtk::ScrolledWindow {
                set_child = Some(&gtk::Box) {
                    set_orientation: gtk::Orientation::Vertical,
                    factory!(model.view),
                },
                set_hexpand: true,
                set_vexpand: true,
            }
        }
    }
}