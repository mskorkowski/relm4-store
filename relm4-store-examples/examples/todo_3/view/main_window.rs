use reexport::{gtk, relm4, relm4_macros};
use std::{cell::RefCell, rc::Rc};
use gtk::prelude::{BoxExt, OrientableExt,  GtkWindowExt};
use relm4::{AppUpdate, Components, Model as ViewModel, Sender, RelmComponent, Widgets};
use relm4_macros::widget;
use store::{Source, StoreSize, StoreViewInterface, window::{KeepOnBottom, KeepOnTop, PositionTrackingWindow, ValueTrackingWindow}};

use crate::{
    store::Tasks,
    view::{ task::TaskFactoryBuilder, task_list::TasksListConfiguration, task_list::TasksListViewModel}
};

use super::task::TaskMsg;

pub enum MainWindowMsg {
    List1Update,
    List2Update,
    List3Update,
    List4Update,
}

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
        msg: Self::Msg , 
        components: &Self::Components, 
        _sender: Sender<Self::Msg>
    ) -> bool {
        match msg {
            MainWindowMsg::List1Update => {
                //force redraw of the components
                components.list2.send(TaskMsg::Reload).unwrap();
                components.list3.send(TaskMsg::Reload).unwrap();
                components.list4.send(TaskMsg::Reload).unwrap();
            },
            MainWindowMsg::List2Update => {
                //force redraw of the components
                components.list1.send(TaskMsg::Reload).unwrap();
                components.list3.send(TaskMsg::Reload).unwrap();
                components.list4.send(TaskMsg::Reload).unwrap();
            },
            MainWindowMsg::List3Update => {
                components.list1.send(TaskMsg::Reload).unwrap();
                components.list2.send(TaskMsg::Reload).unwrap();
                components.list4.send(TaskMsg::Reload).unwrap();
            },
            MainWindowMsg::List4Update => {
                components.list1.send(TaskMsg::Reload).unwrap();
                components.list2.send(TaskMsg::Reload).unwrap();
                components.list3.send(TaskMsg::Reload).unwrap();
            }
        }
        true
    }
}

pub struct MainWindowComponents {
    list1: RelmComponent<TasksListViewModel<ListSource1>, MainWindowViewModel>,
    list2: RelmComponent<TasksListViewModel<ListSource2>, MainWindowViewModel>,
    list3: RelmComponent<TasksListViewModel<ListSource3>, MainWindowViewModel>,
    list4: RelmComponent<TasksListViewModel<ListSource4>, MainWindowViewModel>,
}

impl Components<MainWindowViewModel> for MainWindowComponents {
    fn init_components(
        parent_model: &MainWindowViewModel,
        parent_widgets: &MainWindowWidgets,
        parent_sender: Sender<MainWindowMsg>,
    ) -> Self {
        Self {
            list1: RelmComponent::new(parent_model, parent_widgets, parent_sender.clone()),
            list2: RelmComponent::new(parent_model, parent_widgets, parent_sender.clone()),
            list3: RelmComponent::new(parent_model, parent_widgets, parent_sender.clone()),
            list4: RelmComponent::new(parent_model, parent_widgets, parent_sender.clone()),
        }
    }
}

struct ListSource1 {}

impl Source for ListSource1 {
    type ParentViewModel = MainWindowViewModel;
    type SV = StoreViewInterface<TaskFactoryBuilder<PositionTrackingWindow>>;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV {
        StoreViewInterface::new(parent_model.tasks.clone(), StoreSize::Items(parent_model.page_size))
    }
}

impl TasksListConfiguration for ListSource1 {
    type Window = PositionTrackingWindow;
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>> {
        parent_model.tasks.clone()
    }

    fn ping_parent_message() -> MainWindowMsg {
        MainWindowMsg::List1Update
    }
}

struct ListSource2 {}

impl Source for ListSource2 {
    type ParentViewModel = MainWindowViewModel;
    type SV = StoreViewInterface<TaskFactoryBuilder<ValueTrackingWindow>>;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV {
        StoreViewInterface::new(parent_model.tasks.clone(), StoreSize::Items(parent_model.page_size))
    }
}

impl TasksListConfiguration for ListSource2 {
    type Window = ValueTrackingWindow;
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>> {
        parent_model.tasks.clone()
    }

    fn ping_parent_message() -> MainWindowMsg {
        MainWindowMsg::List2Update
    }
}

struct ListSource3 {}

impl Source for ListSource3 {
    type ParentViewModel = MainWindowViewModel;
    type SV = StoreViewInterface<TaskFactoryBuilder<KeepOnTop>>;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV {
        StoreViewInterface::new(parent_model.tasks.clone(), StoreSize::Items(parent_model.page_size))
    }
}

impl TasksListConfiguration for ListSource3 {
    type Window = KeepOnTop;
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>> {
        parent_model.tasks.clone()
    }

    fn ping_parent_message() -> MainWindowMsg {
        MainWindowMsg::List3Update
    }
}

struct ListSource4 {}

impl Source for ListSource4 {
    type ParentViewModel = MainWindowViewModel;
    type SV = StoreViewInterface<TaskFactoryBuilder<KeepOnBottom>>;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV {
        StoreViewInterface::new(parent_model.tasks.clone(), StoreSize::Items(parent_model.page_size))
    }
}

impl TasksListConfiguration for ListSource4 {
    type Window = KeepOnBottom;
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>> {
        parent_model.tasks.clone()
    }

    fn ping_parent_message() -> MainWindowMsg {
        MainWindowMsg::List4Update
    }
}



#[widget(visibility=pub, relm4=relm4)]
impl Widgets<MainWindowViewModel, ()> for MainWindowWidgets {
    view!{
        root = gtk::ApplicationWindow {
            set_child = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Horizontal,
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    append = &gtk::Label {
                        set_label: "PositionTrackingWindow"
                    },
                    append: component!(components.list1.root_widget()),
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    append = &gtk::Label {
                        set_label: "ValueTrackingWindow"
                    },
                    append: component!(components.list2.root_widget()),
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    append = &gtk::Label {
                        set_label: "KeepOnTop"
                    },
                    append: component!(components.list3.root_widget()),
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    append = &gtk::Label {
                        set_label: "KeepOnBottom"
                    },
                    append: component!(components.list4.root_widget()),
                }
            },
            set_default_size: args!(350, 800),
        }
    }
}