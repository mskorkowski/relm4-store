use reexport::{gtk, relm4, relm4_macros};
use std::{ cell::RefCell, rc::Rc};
use gtk::prelude::{BoxExt, OrientableExt, GtkWindowExt};
use relm4::{AppUpdate, Components, Model as ViewModel, Sender, Widgets};
use relm4_macros::widget;
use store::{Source, StoreSize, StoreViewComponent, window::{PositionTrackingWindow, ValueTrackingWindow}};

use crate::{
    store::Tasks,
    view::{task_list::TasksListConfiguration, task_list::TasksListViewModel}
};

pub enum MainWindowMsg {}

pub struct MainWindowViewModel {
    pub tasks: Rc<RefCell<Tasks>>,
    pub page_size: usize,
}

impl ViewModel for MainWindowViewModel {
    type Msg = MainWindowMsg;
    type Widgets = MainWindowWidgets;
    type Components = MainWindowComponents;
}

impl AppUpdate for MainWindowViewModel {
    fn update(
        &mut self, 
        _msg: Self::Msg , 
        _components: &Self::Components, 
        _sender: Sender<Self::Msg>
    ) -> bool {
        true
    }
}

pub struct MainWindowComponents {
    tasks_list_1: StoreViewComponent<TasksListViewModel<TaskList1Configuration>>,
    tasks_list_2: StoreViewComponent<TasksListViewModel<TaskList2Configuration>>,
    tasks_list_3: StoreViewComponent<TasksListViewModel<TaskList3Configuration>>,
    tasks_list_4: StoreViewComponent<TasksListViewModel<TaskList4Configuration>>,
}

impl Components<MainWindowViewModel> for MainWindowComponents {
    fn init_components(
        parent_model: &MainWindowViewModel,
        _parent_widgets: &MainWindowWidgets,
        _parent_sender: Sender<MainWindowMsg>,
    ) -> Self {
        Self {
            tasks_list_1: TaskList1Configuration::store(parent_model),
            tasks_list_2: TaskList2Configuration::store(parent_model),
            tasks_list_3: TaskList3Configuration::store(parent_model),
            tasks_list_4: TaskList4Configuration::store(parent_model),
        }
    }
}

struct TaskList1Configuration {}
impl Source for TaskList1Configuration {
    type ParentViewModel = MainWindowViewModel;
    type SV = StoreViewComponent<TasksListViewModel<Self>>;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV {
        StoreViewComponent::new(parent_model, parent_model.tasks.clone(), StoreSize::Items(parent_model.page_size))
    }
}

impl TasksListConfiguration for TaskList1Configuration {
    type Window = ValueTrackingWindow;
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>> {
        parent_model.tasks.clone()
    }
}

struct TaskList2Configuration {}
impl Source for TaskList2Configuration {
    type ParentViewModel = MainWindowViewModel;
    type SV = StoreViewComponent<TasksListViewModel<Self>>;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV {
        StoreViewComponent::new(parent_model, parent_model.tasks.clone(), StoreSize::Items(parent_model.page_size))
    }
}

impl TasksListConfiguration for TaskList2Configuration {
    type Window = PositionTrackingWindow;
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>> {
        parent_model.tasks.clone()
    }
}

struct TaskList3Configuration {}
impl Source for TaskList3Configuration {
    type ParentViewModel = MainWindowViewModel;
    type SV = StoreViewComponent<TasksListViewModel<Self>>;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV {
        StoreViewComponent::new(parent_model, parent_model.tasks.clone(), StoreSize::Items(parent_model.page_size))
    }
}

impl TasksListConfiguration for TaskList3Configuration {
    type Window = PositionTrackingWindow;
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>> {
        parent_model.tasks.clone()
    }
}

struct TaskList4Configuration {}
impl Source for TaskList4Configuration {
    type ParentViewModel = MainWindowViewModel;
    type SV = StoreViewComponent<TasksListViewModel<Self>>;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV {
        StoreViewComponent::new(parent_model, parent_model.tasks.clone(), StoreSize::Items(parent_model.page_size))
    }
}

impl TasksListConfiguration for TaskList4Configuration {
    type Window = PositionTrackingWindow;
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>> {
        parent_model.tasks.clone()
    }
}

#[widget(visibility=pub, relm4=relm4)]
impl Widgets<MainWindowViewModel, ()> for MainWindowWidgets {
    view!{
        root = gtk::ApplicationWindow {
            set_child= Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Horizontal,
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    append = &gtk::Label {
                        set_label: "PositionTrackingWindow",
                    },
                    append: component!(components.tasks_list_1.root_widget()),
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    append = &gtk::Label {
                        set_label: "ValueTrackingWindow",
                    },
                    append: component!(components.tasks_list_2.root_widget()),
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    append = &gtk::Label {
                        set_label: "KeepOnTop",
                    },
                    append: component!(components.tasks_list_3.root_widget()),
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    append = &gtk::Label {
                        set_label: "KeepOnBottom",
                    },
                    append: component!(components.tasks_list_4.root_widget()),
                }
            },
            set_default_size: args!(350, 800),
        }
    }
}