use reexport::{gtk, relm4, relm4_macros};
use std::{cell::RefCell, rc::Rc};
use gtk::prelude::{BoxExt, OrientableExt,  GtkWindowExt};
use relm4::{AppUpdate, Components, Model as ViewModel, Sender, RelmComponent, Widgets};
use relm4_macros::widget;
use store::{Source, StoreSize, StoreViewInterface, window::PositionTrackingWindow};

use crate::{
    store::Tasks,
    view::{ task::TaskFactoryBuilder, task_list::TasksListConfiguration, task_list::TasksListViewModel}
};

use super::task::TaskMsg;

pub enum MainWindowMsg {
    LeftListUpdate,
    RightListUpdate,
}

pub struct MainWindowViewModel {
    pub tasks: Rc<RefCell<Tasks>>
}

impl ViewModel for MainWindowViewModel {
    type Msg = MainWindowMsg;
    type Widgets = MainWindowWidgets;
    type Components = MainWindowComponents;
}

impl AppUpdate for MainWindowViewModel {
    fn update(
        &mut self, 
        msg: Self::Msg , 
        components: &Self::Components, 
        _sender: Sender<Self::Msg>
    ) -> bool {
        match msg {
            MainWindowMsg::LeftListUpdate => {
                //force redraw of the components
                components.right_list.send(TaskMsg::Reload).unwrap();
            },
            MainWindowMsg::RightListUpdate => {
                //force redraw of the components
                components.left_list.send(TaskMsg::Reload).unwrap();
            }
        }
        true
    }
}

pub struct MainWindowComponents {
    left_list: RelmComponent<TasksListViewModel<LeftListSource>, MainWindowViewModel>,
    right_list: RelmComponent<TasksListViewModel<TaskList2Source>, MainWindowViewModel>,
}

impl Components<MainWindowViewModel> for MainWindowComponents {
    fn init_components(
        parent_model: &MainWindowViewModel,
        parent_widgets: &MainWindowWidgets,
        parent_sender: Sender<MainWindowMsg>,
    ) -> Self {
        Self {
            left_list: RelmComponent::new(parent_model, parent_widgets, parent_sender.clone()),
            right_list: RelmComponent::new(parent_model, parent_widgets, parent_sender.clone()),
        }
    }
}

struct LeftListSource {}

impl Source for LeftListSource {
    type ParentViewModel = MainWindowViewModel;
    type SV = StoreViewInterface<TaskFactoryBuilder<PositionTrackingWindow>>;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV {
        StoreViewInterface::new(parent_model.tasks.clone(), StoreSize::Items(50))
    }
}

impl TasksListConfiguration for LeftListSource {
    type Window = PositionTrackingWindow;
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>> {
        parent_model.tasks.clone()
    }

    fn ping_parent_message() -> MainWindowMsg {
        MainWindowMsg::LeftListUpdate
    }
}

struct TaskList2Source {}

impl Source for TaskList2Source {
    type ParentViewModel = MainWindowViewModel;
    type SV = StoreViewInterface<TaskFactoryBuilder<PositionTrackingWindow>>;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV {
        StoreViewInterface::new(parent_model.tasks.clone(), StoreSize::Items(50))
    }
}

impl TasksListConfiguration for TaskList2Source {
    type Window = PositionTrackingWindow;
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>> {
        parent_model.tasks.clone()
    }

    fn ping_parent_message() -> MainWindowMsg {
        MainWindowMsg::RightListUpdate
    }
}


#[widget(visibility=pub, relm4=relm4)]
impl Widgets<MainWindowViewModel, ()> for MainWindowWidgets {
    view!{
        root = gtk::ApplicationWindow {
            set_child = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Horizontal,
                append: component!(components.left_list.root_widget()),
                append: component!(components.right_list.root_widget()),
            },
            set_default_size: args!(350, 800),
        }
    }
}