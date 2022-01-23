use reexport::{gtk, relm4, relm4_macros};
use gtk::prelude::{ButtonExt, GtkWindowExt};
use relm4::{AppUpdate, Components, Model as ViewModel, Sender, Widgets, send};
use relm4_macros::widget;
use store::{OrderedStore, StoreSize, StoreViewComponent};

use crate::{
    store::{Tasks, OrderTasksBy},
    view::{task_list::TasksListConfiguration, task_list::TasksListViewModel}
};

pub enum MainWindowMsg {
    ASC,
    DESC,
}

pub struct MainWindowViewModel {
    pub tasks: Tasks,
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
        _components: &Self::Components, 
        _sender: Sender<Self::Msg>
    ) -> bool {
        match msg  {
            MainWindowMsg::ASC => {
                self.tasks.set_order(
                    OrderTasksBy::Name{ascending: true}
                )
            },
            MainWindowMsg::DESC => {
                self.tasks.set_order(
                    OrderTasksBy::Name{ascending: false}
                )
            }
        }

        true
    }
}

pub struct MainWindowComponents {
    tasks_list: StoreViewComponent<TasksListViewModel<Self>>
}

impl Components<MainWindowViewModel> for MainWindowComponents {
    fn init_components(
        parent_model: &MainWindowViewModel,
        _parent_sender: Sender<MainWindowMsg>,
    ) -> Self {
        Self {
            tasks_list: StoreViewComponent::new(
                parent_model,
                parent_model.tasks.clone(),
                StoreSize::Items(10)
            ),
        }
    }

    fn connect_parent(&mut self, _parent_widgets: &MainWindowWidgets) { }
}

impl TasksListConfiguration for MainWindowComponents {
    type ParentViewModel = MainWindowViewModel;

    fn get_tasks(parent_model: &Self::ParentViewModel) -> Tasks {
        parent_model.tasks.clone()
    }
}



#[widget(visibility=pub, relm4=relm4)]
impl Widgets<MainWindowViewModel, ()> for MainWindowWidgets {
    view!{
        root = gtk::ApplicationWindow {
            set_child: Some(components.tasks_list.root_widget()),
            set_titlebar= Some(&gtk::HeaderBar){
                set_title_widget = Some(&gtk::Label::new(Some("todo_3"))){},
                pack_end = &gtk::Button::from_icon_name("view-sort-ascending-symbolic") {
                    connect_clicked(sender) => move |_| {
                        send!(sender, MainWindowMsg::ASC)
                    },
                },
                pack_end = &gtk::Button::from_icon_name("view-sort-descending-symbolic") {
                    connect_clicked(sender) => move |_| {
                        send!(sender, MainWindowMsg::DESC)
                    },
                },
            },
            set_default_size: args!(350, 800),
        }
    }
}