use reexport::{gtk, relm4, relm4_macros};
use std::{ cell::RefCell, rc::Rc};
use gtk::prelude::GtkWindowExt;
use relm4::{AppUpdate, Components, Model as ViewModel, Sender, Widgets};
use relm4_macros::widget;
use store::{Source, StoreSize, StoreViewInterface};

use crate::{
    store::Tasks,
    view::{task_list::TasksListConfiguration, task_list::TasksListViewModel}
};

pub enum MainWindowMsg {}

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
        _msg: Self::Msg , 
        _components: &Self::Components, 
        _sender: Sender<Self::Msg>
    ) -> bool {
        true
    }
}

pub struct MainWindowComponents {
    tasks_list: StoreViewInterface<TasksListViewModel<Self>>,
}

impl Components<MainWindowViewModel> for MainWindowComponents {
    fn init_components(
        parent_model: &MainWindowViewModel,
        _parent_widgets: &MainWindowWidgets,
        _parent_sender: Sender<MainWindowMsg>,
    ) -> Self {
        Self {
            tasks_list: Self::store(parent_model),
        }
    }
}

impl Source for MainWindowComponents {
    type ParentViewModel = MainWindowViewModel;
    type SV = StoreViewInterface<TasksListViewModel<Self>>;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV {
        StoreViewInterface::new(parent_model, parent_model.tasks.clone(), StoreSize::Items(50))
    }
}

impl TasksListConfiguration for MainWindowComponents {
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Rc<RefCell<Tasks>> {
        parent_model.tasks.clone()
    }
}



#[widget(visibility=pub, relm4=relm4)]
impl Widgets<MainWindowViewModel, ()> for MainWindowWidgets {
    view!{
        root = gtk::ApplicationWindow {
            set_child: component!(Some(components.tasks_list.root_widget())),
            set_default_size: args!(350, 800),
        }
    }
}