use record::DefaultIdAllocator;
use reexport::{gtk, relm4, relm4_macros};
use std::{ cell::RefCell, rc::Rc};
use gtk::prelude::{BoxExt, OrientableExt, GtkWindowExt};
use relm4::{AppUpdate, Components, Model as ViewModel, Sender, Widgets, WidgetPlus};
use relm4_macros::widget;
use store::{StoreSize, StoreViewComponent, window::{KeepOnBottom, KeepOnTop, PositionTrackingWindow, ValueTrackingWindow}};

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
    tasks_list_1: StoreViewComponent<TasksListViewModel<TaskList1Configuration>, DefaultIdAllocator, DefaultIdAllocator>,
    tasks_list_2: StoreViewComponent<TasksListViewModel<TaskList2Configuration>, DefaultIdAllocator, DefaultIdAllocator>,
    tasks_list_3: StoreViewComponent<TasksListViewModel<TaskList3Configuration>, DefaultIdAllocator, DefaultIdAllocator>,
    tasks_list_4: StoreViewComponent<TasksListViewModel<TaskList4Configuration>, DefaultIdAllocator, DefaultIdAllocator>,
}

impl Components<MainWindowViewModel> for MainWindowComponents {
    fn init_components(
        parent_view_model: &MainWindowViewModel,
        _parent_sender: Sender<MainWindowMsg>,
    ) -> Self {
        Self {
            tasks_list_1: StoreViewComponent::new(parent_view_model, parent_view_model.tasks.clone(), StoreSize::Items(parent_view_model.page_size)),
            tasks_list_2: StoreViewComponent::new(parent_view_model, parent_view_model.tasks.clone(), StoreSize::Items(parent_view_model.page_size)),
            tasks_list_3: StoreViewComponent::new(parent_view_model, parent_view_model.tasks.clone(), StoreSize::Items(parent_view_model.page_size)),
            tasks_list_4: StoreViewComponent::new(parent_view_model, parent_view_model.tasks.clone(), StoreSize::Items(parent_view_model.page_size)),
        }
    }

    fn connect_parent(&mut self, _parent_widgets: &MainWindowWidgets) {}
}

struct TaskList1Configuration {}
impl TasksListConfiguration for TaskList1Configuration {
    type ParentViewModel = MainWindowViewModel;
    type Window = PositionTrackingWindow;
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>> {
        parent_model.tasks.clone()
    }
}

struct TaskList2Configuration {}
impl TasksListConfiguration for TaskList2Configuration {
    type ParentViewModel = MainWindowViewModel;
    type Window = ValueTrackingWindow;
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>> {
        parent_model.tasks.clone()
    }
}

struct TaskList3Configuration {}
impl TasksListConfiguration for TaskList3Configuration {
    type ParentViewModel = MainWindowViewModel;
    type Window = KeepOnTop;
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>> {
        parent_model.tasks.clone()
    }
}

struct TaskList4Configuration {}
impl TasksListConfiguration for TaskList4Configuration {
    type ParentViewModel = MainWindowViewModel;
    type Window = KeepOnBottom;
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
                    set_margin_all: 12,
                    append = &gtk::Label {
                        set_label: "PositionTrackingWindow",
                    },
                    append: components.tasks_list_1.root_widget(),
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 12,
                    append = &gtk::Label {
                        set_label: "ValueTrackingWindow",
                    },
                    append: components.tasks_list_2.root_widget(),
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 12,
                    append = &gtk::Label {
                        set_label: "KeepOnTop",
                    },
                    append: components.tasks_list_3.root_widget(),
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 12,
                    append = &gtk::Label {
                        set_label: "KeepOnBottom",
                    },
                    append: components.tasks_list_4.root_widget(),
                }
            },
            set_default_size: args!(1100, 600),
        }
    }
}