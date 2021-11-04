use reexport::gtk;
use reexport::relm4;
use reexport::relm4_macros;

use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;

use relm4::ComponentUpdate;
use relm4::Model as ViewModel;
use relm4::send;
use relm4::Sender;
use relm4::Widgets;
use relm4::WidgetPlus;

use relm4_macros::widget;

use store::DataStoreBase;
use store::Source;
use store::StoreView;
use store::StoreViewInterface;
use store::math::Range;

use crate::model::Task;
use crate::store::Tasks;
use crate::view::task::TaskFactoryBuilder;
use crate::view::task::TaskMsg;

type StoreMsg = store::StoreMsg<Task>;

pub trait TasksListConfiguration: Source<SV=StoreViewInterface<TaskFactoryBuilder>> {
    fn get_tasks(parent: &Self::ParentViewModel) -> Rc<RefCell<Tasks>>;

    fn page_size(parent: &Self::ParentViewModel) -> usize;
}

pub struct TasksListViewModel<Config> 
where Config: TasksListConfiguration,
{
    view: Rc<Config::SV>,
    tasks: Rc<RefCell<Tasks>>,
    new_task_description: gtk::EntryBuffer,
    scroll_adjustment: gtk::Adjustment,
}

impl<Config> ViewModel for TasksListViewModel<Config>
where Config: TasksListConfiguration + 'static,
{
    type Msg = TaskMsg;
    type Widgets = TasksListViewWidgets;
    type Components = ();
}

impl<Config> ComponentUpdate<Config::ParentViewModel> for TasksListViewModel<Config>
where Config: TasksListConfiguration + 'static,
{
    fn init_model(parent_model: &Config::ParentViewModel) -> TasksListViewModel<Config> {
        let view = Rc::new(Config::store(parent_model));
        let view_length = view.len();
        TasksListViewModel{
            view,
            tasks: Config::get_tasks(parent_model),
            new_task_description: gtk::EntryBuffer::new(None),
            scroll_adjustment: gtk::Adjustment::new(0.0, 0.0, view_length as f64, 1.0, 1.0, Config::page_size(parent_model) as f64)
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
                self.scroll_adjustment.set_upper(self.view.len() as f64);
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

                self.view.set_window(
                    Range::new(
                        value.floor() as usize,
                        (value+page_size).floor() as usize,
                    )
                );
            },
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
            append = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                append: scrollbar = &gtk::Scrollbar {
                    set_orientation: gtk::Orientation::Vertical,
                    set_adjustment: Some(&model.scroll_adjustment),
                },
                append: viewport = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    factory!(model.view as &Config::SV),
                    set_vexpand: true,
                },
                set_hexpand: true,
                set_vexpand: false,
            },
        }
    }

    fn post_init() {
        let adjustment = scrollbar.adjustment().unwrap();

        {
            let sender = sender.clone();
            adjustment.connect_value_changed(move |_| {
                send!(sender, TaskMsg::Scrolled);
            });

        }
    }
}